use super::def_type::{CompositeType, get_def_types};
use crate::{
    binder::{SymbolKind, SymbolTable},
    document::Document,
    idx::{Idx, InternIdent},
    types_analyzer::{self, DefType},
};
use rowan::{GreenNodeData, Language, NodeOrToken, ast::AstNode};
use wat_syntax::{
    SyntaxKind, WatLanguage,
    ast::{FieldType as AstFieldType, StorageType as AstStorageType, ValType as AstValType},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub(crate) enum ValType<'db> {
    I32,
    I64,
    F32,
    F64,
    V128,
    Ref(RefType<'db>),
}
impl<'db> ValType<'db> {
    pub(crate) fn from_ast(node: &AstValType, db: &'db dyn salsa::Database) -> Option<Self> {
        Self::from_green(&node.syntax().green(), db)
    }

    pub(crate) fn from_green(node: &GreenNodeData, db: &'db dyn salsa::Database) -> Option<Self> {
        match WatLanguage::kind_from_raw(node.kind()) {
            SyntaxKind::NUM_TYPE => match node
                .children()
                .next()
                .and_then(|child| child.into_token())?
                .text()
            {
                "i32" => Some(ValType::I32),
                "i64" => Some(ValType::I64),
                "f32" => Some(ValType::F32),
                "f64" => Some(ValType::F64),
                _ => None,
            },
            SyntaxKind::VEC_TYPE => Some(ValType::V128),
            SyntaxKind::REF_TYPE => RefType::from_green(node, db).map(ValType::Ref),
            _ => None,
        }
    }

    pub(crate) fn matches(
        &self,
        other: &Self,
        db: &'db dyn salsa::Database,
        document: Document,
        module_id: u32,
    ) -> bool {
        match (self, other) {
            (ValType::Ref(a), ValType::Ref(b)) => a.matches(b, db, document, module_id),
            _ => self == other,
        }
    }

    pub(crate) fn type_equals(
        &self,
        other: &Self,
        db: &'db dyn salsa::Database,
        document: Document,
        module_id: u32,
    ) -> bool {
        if let (ValType::Ref(a), ValType::Ref(b)) = (self, other) {
            a.type_equals(b, db, document, module_id)
        } else {
            self == other
        }
    }

    pub(crate) fn defaultable(&self) -> bool {
        match self {
            ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 | ValType::V128 => true,
            ValType::Ref(RefType { nullable, .. }) => *nullable,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub(crate) struct RefType<'db> {
    pub heap_ty: HeapType<'db>,
    pub nullable: bool,
}
impl<'db> RefType<'db> {
    pub(crate) fn from_green(node: &GreenNodeData, db: &'db dyn salsa::Database) -> Option<Self> {
        let mut children = node.children();
        match children.next().and_then(|child| child.into_token())?.text() {
            "anyref" => Some(RefType {
                heap_ty: HeapType::Any,
                nullable: true,
            }),
            "eqref" => Some(RefType {
                heap_ty: HeapType::Eq,
                nullable: true,
            }),
            "i31ref" => Some(RefType {
                heap_ty: HeapType::I31,
                nullable: true,
            }),
            "structref" => Some(RefType {
                heap_ty: HeapType::Struct,
                nullable: true,
            }),
            "arrayref" => Some(RefType {
                heap_ty: HeapType::Array,
                nullable: true,
            }),
            "nullref" => Some(RefType {
                heap_ty: HeapType::None,
                nullable: true,
            }),
            "funcref" => Some(RefType {
                heap_ty: HeapType::Func,
                nullable: true,
            }),
            "nullfuncref" => Some(RefType {
                heap_ty: HeapType::NoFunc,
                nullable: true,
            }),
            "externref" => Some(RefType {
                heap_ty: HeapType::Extern,
                nullable: true,
            }),
            "nullexternref" => Some(RefType {
                heap_ty: HeapType::NoExtern,
                nullable: true,
            }),
            _ => {
                let mut nullable = false;
                for node_or_token in children {
                    match node_or_token {
                        NodeOrToken::Node(node) if node.kind() == SyntaxKind::HEAP_TYPE.into() => {
                            return HeapType::from_green(node, db)
                                .map(|heap_ty| RefType { heap_ty, nullable });
                        }
                        NodeOrToken::Token(token)
                            if token.kind() == SyntaxKind::KEYWORD.into()
                                && token.text() == "null" =>
                        {
                            nullable = true;
                        }
                        _ => {}
                    }
                }
                None
            }
        }
    }

    pub(crate) fn matches(
        &self,
        other: &Self,
        db: &'db dyn salsa::Database,
        document: Document,
        module_id: u32,
    ) -> bool {
        self.heap_ty
            .matches(&other.heap_ty, db, document, module_id)
            && (!self.nullable || other.nullable)
    }

    pub(crate) fn type_equals(
        &self,
        other: &Self,
        db: &'db dyn salsa::Database,
        document: Document,
        module_id: u32,
    ) -> bool {
        self.nullable == other.nullable
            && self
                .heap_ty
                .type_equals(&other.heap_ty, db, document, module_id)
    }

    pub(crate) fn diff(&self, other: &Self) -> Self {
        RefType {
            heap_ty: self.heap_ty.clone(),
            nullable: if other.nullable { false } else { self.nullable },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub(crate) enum HeapType<'db> {
    Type(Idx<'db>),
    Any,
    Eq,
    I31,
    Struct,
    Array,
    None,
    Func,
    NoFunc,
    Extern,
    NoExtern,
    Rec(u32),          // internal use, not actually a valid heap type
    DefFunc(Idx<'db>), // internal use
}
impl<'db> HeapType<'db> {
    pub(crate) fn from_green(node: &GreenNodeData, db: &'db dyn salsa::Database) -> Option<Self> {
        match node.children().next() {
            Some(NodeOrToken::Node(node)) if node.kind() == SyntaxKind::INDEX.into() => {
                let token = node.children().next()?.into_token()?;
                match WatLanguage::kind_from_raw(token.kind()) {
                    SyntaxKind::UNSIGNED_INT => Some(HeapType::Type(Idx {
                        num: token.text().parse().ok(),
                        name: None,
                    })),
                    SyntaxKind::IDENT => Some(HeapType::Type(Idx {
                        num: None,
                        name: Some(InternIdent::new(db, token.text())),
                    })),
                    _ => None,
                }
            }
            Some(NodeOrToken::Token(token)) if token.kind() == SyntaxKind::TYPE_KEYWORD.into() => {
                match token.text() {
                    "any" => Some(HeapType::Any),
                    "eq" => Some(HeapType::Eq),
                    "i31" => Some(HeapType::I31),
                    "struct" => Some(HeapType::Struct),
                    "array" => Some(HeapType::Array),
                    "none" => Some(HeapType::None),
                    "func" => Some(HeapType::Func),
                    "nofunc" => Some(HeapType::NoFunc),
                    "extern" => Some(HeapType::Extern),
                    "noextern" => Some(HeapType::NoExtern),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub(crate) fn matches(
        &self,
        other: &Self,
        db: &'db dyn salsa::Database,
        document: Document,
        module_id: u32,
    ) -> bool {
        match (self, other) {
            (
                HeapType::Any | HeapType::Eq | HeapType::Struct | HeapType::Array | HeapType::I31,
                HeapType::Any,
            )
            | (HeapType::I31 | HeapType::Struct | HeapType::Array, HeapType::Eq) => true,
            (HeapType::None, other) => other.matches(&HeapType::Any, db, document, module_id),
            (HeapType::NoFunc, other) => other.matches(&HeapType::Func, db, document, module_id),
            (HeapType::NoExtern, other) => {
                other.matches(&HeapType::Extern, db, document, module_id)
            }
            (heap_ty_a @ &HeapType::Type(mut a), heap_ty_b @ &HeapType::Type(mut b)) => {
                if a.is_def() {
                    a.name = None;
                }
                if b.is_def() {
                    b.name = None;
                }
                let symbol_table = SymbolTable::of(db, document);
                let Some(module) = symbol_table.find_module(module_id) else {
                    return false;
                };
                let def_types = get_def_types(db, document);
                symbol_table
                    .symbols
                    .iter()
                    .find(|symbol| {
                        symbol.kind == SymbolKind::Type
                            && symbol.region == module.key
                            && a.is_defined_by(&symbol.idx)
                    })
                    .zip(symbol_table.symbols.iter().find(|symbol| {
                        symbol.kind == SymbolKind::Type
                            && symbol.region == module.key
                            && b.is_defined_by(&symbol.idx)
                    }))
                    .map(|(a, b)| (a.key, b.key))
                    .is_some_and(|(a, b)| {
                        a == b
                            || heap_ty_a.type_equals(heap_ty_b, db, document, module_id)
                            || def_types
                                .iter()
                                .find(|def_type| def_type.key == a)
                                .and_then(|def_type| def_type.inherits.as_ref())
                                .is_some_and(|inherits| {
                                    HeapType::Type(inherits.idx)
                                        .matches(heap_ty_b, db, document, module_id)
                                })
                    })
            }
            (&HeapType::Type(mut a), b) => {
                if a.is_def() {
                    a.name = None;
                }
                let symbol_table = SymbolTable::of(db, document);
                let Some(module) = symbol_table.find_module(module_id) else {
                    return false;
                };
                let def_types = get_def_types(db, document);
                #[expect(clippy::match_like_matches_macro)]
                symbol_table
                    .symbols
                    .iter()
                    .find(|symbol| {
                        symbol.kind == SymbolKind::Type
                            && symbol.region == module.key
                            && a.is_defined_by(&symbol.idx)
                    })
                    .and_then(|symbol| def_types.iter().find(|def_type| def_type.key == symbol.key))
                    .is_some_and(|def_type| match (&def_type.comp, b) {
                        (
                            CompositeType::Struct(..),
                            HeapType::Any | HeapType::Eq | HeapType::Struct,
                        ) => true,
                        (
                            CompositeType::Array(..),
                            HeapType::Any | HeapType::Eq | HeapType::Array,
                        ) => true,
                        (CompositeType::Func(..), HeapType::Func) => true,
                        _ => false,
                    })
            }
            (HeapType::DefFunc(a), &HeapType::Type(mut b)) => {
                if b.is_def() {
                    b.name = None;
                }
                let symbol_table = SymbolTable::of(db, document);
                let Some(module) = symbol_table.find_module(module_id) else {
                    return false;
                };
                let def_types = get_def_types(db, document);
                if let Some((
                    a,
                    DefType {
                        comp: CompositeType::Func(b),
                        ..
                    },
                )) = symbol_table
                    .symbols
                    .iter()
                    .find(|symbol| {
                        symbol.kind == SymbolKind::Func
                            && symbol.region == module.key
                            && a.is_defined_by(&symbol.idx)
                    })
                    .map(|symbol| {
                        types_analyzer::get_func_sig(db, document, symbol.key, symbol.green.clone())
                    })
                    .zip(
                        symbol_table
                            .symbols
                            .iter()
                            .find(|symbol| {
                                symbol.kind == SymbolKind::Type
                                    && symbol.region == module.key
                                    && b.is_defined_by(&symbol.idx)
                            })
                            .filter(|symbol| {
                                types_analyzer::get_rec_type_groups(db, document)
                                    .iter()
                                    .find(|group| group.type_defs.contains(&symbol.key))
                                    .is_none_or(|group| group.type_defs.len() <= 1)
                            })
                            .and_then(|symbol| {
                                def_types.iter().find(|def_type| def_type.key == symbol.key)
                            }),
                    )
                {
                    a.type_equals(b, db, document, module_id)
                } else {
                    false
                }
            }
            (a, b) => a == b,
        }
    }

    pub(crate) fn type_equals(
        &self,
        other: &Self,
        db: &'db dyn salsa::Database,
        document: Document,
        module_id: u32,
    ) -> bool {
        if let (&HeapType::Type(mut a), &HeapType::Type(mut b)) = (self, other) {
            if a.is_def() {
                a.name = None;
            }
            if b.is_def() {
                b.name = None;
            }
            let symbol_table = SymbolTable::of(db, document);
            let Some(module) = symbol_table.find_module(module_id) else {
                return false;
            };
            let def_types = get_def_types(db, document);
            symbol_table
                .symbols
                .iter()
                .find(|symbol| {
                    symbol.kind == SymbolKind::Type
                        && symbol.region == module.key
                        && a.is_defined_by(&symbol.idx)
                })
                .zip(symbol_table.symbols.iter().find(|symbol| {
                    symbol.kind == SymbolKind::Type
                        && symbol.region == module.key
                        && b.is_defined_by(&symbol.idx)
                }))
                .map(|(a, b)| (a.key, b.key))
                .is_some_and(|(a, b)| {
                    a == b
                        || def_types
                            .iter()
                            .find(|def_type| def_type.key == a)
                            .zip(def_types.iter().find(|def_type| def_type.key == b))
                            .is_some_and(|(a, b)| a.type_equals(b, db, document, module_id))
                })
        } else {
            self == other
        }
    }

    pub(crate) fn to_top_type(
        &self,
        db: &'db dyn salsa::Database,
        document: Document,
        module_id: u32,
    ) -> Option<Self> {
        match self {
            HeapType::Any
            | HeapType::None
            | HeapType::Eq
            | HeapType::Struct
            | HeapType::Array
            | HeapType::I31 => Some(HeapType::Any),
            HeapType::Func | HeapType::NoFunc => Some(HeapType::Func),
            HeapType::Extern | HeapType::NoExtern => Some(HeapType::Extern),
            HeapType::Type(idx) => {
                let mut idx = *idx;
                if idx.is_def() {
                    idx.name = None;
                }
                let symbol_table = SymbolTable::of(db, document);
                let module = symbol_table.find_module(module_id)?;
                let def_types = get_def_types(db, document);
                symbol_table
                    .symbols
                    .iter()
                    .find(|symbol| {
                        symbol.kind == SymbolKind::Type
                            && symbol.region == module.key
                            && idx.is_defined_by(&symbol.idx)
                    })
                    .and_then(|symbol| def_types.iter().find(|def_type| def_type.key == symbol.key))
                    .map(|def_type| match &def_type.comp {
                        CompositeType::Struct(..) | CompositeType::Array(..) => HeapType::Struct,
                        CompositeType::Func(..) => HeapType::Func,
                    })
            }
            HeapType::Rec(..) | HeapType::DefFunc(..) => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub(crate) enum OperandType<'db> {
    Val(ValType<'db>),
    Any,
}
impl<'db> From<StorageType<'db>> for OperandType<'db> {
    fn from(value: StorageType<'db>) -> Self {
        match value {
            StorageType::Val(ty) => OperandType::Val(ty),
            StorageType::PackedI8 | StorageType::PackedI16 => OperandType::Val(ValType::I32),
        }
    }
}
impl<'db> From<FieldType<'db>> for OperandType<'db> {
    fn from(value: FieldType<'db>) -> Self {
        value.storage.into()
    }
}

pub(crate) fn operand_type_matches<'db>(
    db: &'db dyn salsa::Database,
    document: Document,
    module_id: u32,
    sub: OperandType<'db>,
    sup: OperandType<'db>,
) -> bool {
    match (sub, sup) {
        (OperandType::Val(sub), OperandType::Val(sup)) => {
            sub.matches(&sup, db, document, module_id)
        }
        (OperandType::Any, _) | (_, OperandType::Any) => true,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, salsa::Update)]
pub(crate) struct Fields<'db>(pub Vec<(FieldType<'db>, Idx<'db>)>);
impl<'db> Fields<'db> {
    pub(crate) fn matches(
        &self,
        other: &Self,
        db: &'db dyn salsa::Database,
        document: Document,
        module_id: u32,
    ) -> bool {
        self.0.len() >= other.0.len()
            && self
                .0
                .iter()
                .zip(&other.0)
                .all(|((a, _), (b, _))| a.matches(b, db, document, module_id))
    }

    pub(crate) fn type_equals(
        &self,
        other: &Self,
        db: &'db dyn salsa::Database,
        document: Document,
        module_id: u32,
    ) -> bool {
        self.0.len() == other.0.len()
            && self
                .0
                .iter()
                .zip(&other.0)
                .all(|((a, _), (b, _))| a.type_equals(b, db, document, module_id))
    }

    pub(crate) fn to_operand_types(&self) -> Vec<OperandType<'db>> {
        self.0
            .iter()
            .map(|(field, _)| field.storage.clone().into())
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, salsa::Update)]
pub(crate) struct FieldType<'db> {
    pub(crate) storage: StorageType<'db>,
    pub(crate) mutable: bool,
}
impl<'db> FieldType<'db> {
    pub(super) fn from_ast(node: &AstFieldType, db: &'db dyn salsa::Database) -> Option<Self> {
        node.storage_type()
            .and_then(|storage_type| StorageType::from_ast(&storage_type, db))
            .map(|storage| FieldType {
                storage,
                mutable: node.mut_keyword().is_some(),
            })
    }

    pub(crate) fn matches(
        &self,
        other: &Self,
        db: &'db dyn salsa::Database,
        document: Document,
        module_id: u32,
    ) -> bool {
        match (self.mutable, other.mutable) {
            (true, true) => {
                self.storage
                    .matches(&other.storage, db, document, module_id)
                    && other
                        .storage
                        .matches(&self.storage, db, document, module_id)
            }
            (false, false) => self
                .storage
                .matches(&other.storage, db, document, module_id),
            _ => false,
        }
    }

    pub(crate) fn type_equals(
        &self,
        other: &Self,
        db: &'db dyn salsa::Database,
        document: Document,
        module_id: u32,
    ) -> bool {
        self.mutable == other.mutable
            && self
                .storage
                .type_equals(&other.storage, db, document, module_id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, salsa::Update)]
pub(crate) enum StorageType<'db> {
    Val(ValType<'db>),
    PackedI8,
    PackedI16,
}
impl<'db> StorageType<'db> {
    fn from_ast(node: &AstStorageType, db: &'db dyn salsa::Database) -> Option<Self> {
        match node {
            AstStorageType::Val(ty) => ValType::from_ast(ty, db).map(StorageType::Val),
            AstStorageType::Packed(ty) => {
                ty.type_keyword()
                    .and_then(|type_keyword| match type_keyword.text() {
                        "i8" => Some(StorageType::PackedI8),
                        "i16" => Some(StorageType::PackedI16),
                        _ => None,
                    })
            }
        }
    }

    pub(crate) fn matches(
        &self,
        other: &Self,
        db: &'db dyn salsa::Database,
        document: Document,
        module_id: u32,
    ) -> bool {
        match (self, other) {
            (StorageType::Val(a), StorageType::Val(b)) => a.matches(b, db, document, module_id),
            (StorageType::PackedI8, StorageType::PackedI8) => true,
            (StorageType::PackedI16, StorageType::PackedI16) => true,
            _ => false,
        }
    }

    pub(crate) fn type_equals(
        &self,
        other: &Self,
        db: &'db dyn salsa::Database,
        document: Document,
        module_id: u32,
    ) -> bool {
        match (self, other) {
            (StorageType::Val(a), StorageType::Val(b)) => a.type_equals(b, db, document, module_id),
            (StorageType::PackedI8, StorageType::PackedI8) => true,
            (StorageType::PackedI16, StorageType::PackedI16) => true,
            _ => false,
        }
    }

    pub(crate) fn is_packed(&self) -> bool {
        matches!(self, StorageType::PackedI8 | StorageType::PackedI16)
    }
}
