use super::{def_type::DefTypeKind, TypesAnalyzerCtx};
use crate::{
    binder::SymbolKind,
    idx::{Idx, InternIdent},
    uri::InternUri,
};
use rowan::{ast::AstNode, GreenNodeData, Language, NodeOrToken};
use wat_syntax::{
    ast::{FieldType as AstFieldType, StorageType as AstStorageType, ValType as AstValType},
    SyntaxKind, WatLanguage,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum ValType {
    I32,
    I64,
    F32,
    F64,
    V128,
    Ref(RefType),
}
impl ValType {
    pub(crate) fn from_ast(node: &AstValType, db: &dyn TypesAnalyzerCtx) -> Option<Self> {
        Self::from_green(&node.syntax().green(), db)
    }

    pub(crate) fn from_green(node: &GreenNodeData, db: &dyn TypesAnalyzerCtx) -> Option<Self> {
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
            SyntaxKind::REF_TYPE => {
                let mut children = node.children();
                match children.next().and_then(|child| child.into_token())?.text() {
                    "anyref" => {
                        return Some(ValType::Ref(RefType {
                            heap_ty: HeapType::Any,
                            nullable: true,
                        }));
                    }
                    "eqref" => {
                        return Some(ValType::Ref(RefType {
                            heap_ty: HeapType::Eq,
                            nullable: true,
                        }));
                    }
                    "i31ref" => {
                        return Some(ValType::Ref(RefType {
                            heap_ty: HeapType::I31,
                            nullable: true,
                        }));
                    }
                    "structref" => {
                        return Some(ValType::Ref(RefType {
                            heap_ty: HeapType::Struct,
                            nullable: true,
                        }));
                    }
                    "arrayref" => {
                        return Some(ValType::Ref(RefType {
                            heap_ty: HeapType::Array,
                            nullable: true,
                        }));
                    }
                    "nullref" => {
                        return Some(ValType::Ref(RefType {
                            heap_ty: HeapType::None,
                            nullable: true,
                        }));
                    }
                    "funcref" => {
                        return Some(ValType::Ref(RefType {
                            heap_ty: HeapType::Func,
                            nullable: true,
                        }));
                    }
                    "nullfuncref" => {
                        return Some(ValType::Ref(RefType {
                            heap_ty: HeapType::NoFunc,
                            nullable: true,
                        }));
                    }
                    "externref" => {
                        return Some(ValType::Ref(RefType {
                            heap_ty: HeapType::Extern,
                            nullable: true,
                        }));
                    }
                    "nullexternref" => {
                        return Some(ValType::Ref(RefType {
                            heap_ty: HeapType::NoExtern,
                            nullable: true,
                        }));
                    }
                    _ => {}
                }
                let mut nullable = false;
                for node_or_token in children {
                    match node_or_token {
                        NodeOrToken::Node(node) if node.kind() == SyntaxKind::HEAP_TYPE.into() => {
                            return match node.children().next() {
                                Some(NodeOrToken::Node(node))
                                    if node.kind() == SyntaxKind::INDEX.into() =>
                                {
                                    let token = node.children().next()?.into_token()?;
                                    match WatLanguage::kind_from_raw(token.kind()) {
                                        SyntaxKind::UNSIGNED_INT => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::Type(Idx {
                                                num: token.text().parse().ok(),
                                                name: None,
                                            }),
                                            nullable,
                                        })),
                                        SyntaxKind::IDENT => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::Type(Idx {
                                                num: None,
                                                name: Some(db.ident(token.text().into())),
                                            }),
                                            nullable,
                                        })),
                                        _ => None,
                                    }
                                }
                                Some(NodeOrToken::Token(token))
                                    if token.kind() == SyntaxKind::TYPE_KEYWORD.into() =>
                                {
                                    match token.text() {
                                        "any" => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::Any,
                                            nullable,
                                        })),
                                        "eq" => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::Eq,
                                            nullable,
                                        })),
                                        "i31" => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::I31,
                                            nullable,
                                        })),
                                        "struct" => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::Struct,
                                            nullable,
                                        })),
                                        "array" => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::Array,
                                            nullable,
                                        })),
                                        "none" => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::None,
                                            nullable,
                                        })),
                                        "func" => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::Func,
                                            nullable,
                                        })),
                                        "nofunc" => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::NoFunc,
                                            nullable,
                                        })),
                                        "extern" => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::Extern,
                                            nullable,
                                        })),
                                        "noextern" => Some(ValType::Ref(RefType {
                                            heap_ty: HeapType::NoExtern,
                                            nullable,
                                        })),
                                        _ => None,
                                    }
                                }
                                _ => None,
                            };
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
            _ => None,
        }
    }

    pub(crate) fn matches(
        &self,
        other: &Self,
        db: &dyn TypesAnalyzerCtx,
        uri: InternUri,
        module_id: u32,
    ) -> bool {
        match (self, other) {
            (ValType::Ref(a), ValType::Ref(b)) => a.matches(b, db, uri, module_id),
            _ => self == other,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct RefType {
    pub heap_ty: HeapType,
    pub nullable: bool,
}
impl RefType {
    pub(crate) fn matches(
        &self,
        other: &Self,
        db: &dyn TypesAnalyzerCtx,
        uri: InternUri,
        module_id: u32,
    ) -> bool {
        self.heap_ty.matches(&other.heap_ty, db, uri, module_id)
            && (!self.nullable || other.nullable)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum HeapType {
    Type(Idx),
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
    Rec(u32), // internal use, not actually a valid heap type
}
impl HeapType {
    pub(crate) fn matches(
        &self,
        other: &Self,
        db: &dyn TypesAnalyzerCtx,
        uri: InternUri,
        module_id: u32,
    ) -> bool {
        match (self, other) {
            (
                HeapType::Any | HeapType::Eq | HeapType::Struct | HeapType::Array | HeapType::I31,
                HeapType::Any,
            )
            | (HeapType::I31 | HeapType::Struct | HeapType::Array, HeapType::Eq) => true,
            (HeapType::None, other) => other.matches(&HeapType::Any, db, uri, module_id),
            (HeapType::NoFunc, other) => other.matches(&HeapType::Func, db, uri, module_id),
            (HeapType::NoExtern, other) => other.matches(&HeapType::Extern, db, uri, module_id),
            (HeapType::Type(a), heap_ty_b @ HeapType::Type(b)) => {
                let symbol_table = db.symbol_table(uri);
                let Some(module) = symbol_table.find_module(module_id) else {
                    return false;
                };
                let def_types = db.def_types(uri);
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
                        if a == b {
                            true
                        } else if let Some(a) = def_types.iter().find(|def_type| def_type.key == a)
                        {
                            def_types
                                .iter()
                                .find(|def_type| def_type.key == b)
                                .is_some_and(|b| a.matches(b, db, uri, module_id))
                                || a.inherits
                                    .and_then(|inherits| {
                                        symbol_table
                                            .symbols
                                            .iter()
                                            .find(|symbol| symbol.key == inherits)
                                    })
                                    .is_some_and(|symbol| {
                                        let idx = Idx {
                                            num: symbol.idx.num,
                                            name: None,
                                        };
                                        HeapType::Type(idx).matches(heap_ty_b, db, uri, module_id)
                                    })
                        } else {
                            false
                        }
                    })
            }
            (HeapType::Type(a), b) => {
                let symbol_table = db.symbol_table(uri);
                let Some(module) = symbol_table.find_module(module_id) else {
                    return false;
                };
                let def_types = db.def_types(uri);
                symbol_table
                    .symbols
                    .iter()
                    .find(|symbol| {
                        symbol.kind == SymbolKind::Type
                            && symbol.region == module.key
                            && a.is_defined_by(&symbol.idx)
                    })
                    .and_then(|symbol| def_types.iter().find(|def_type| def_type.key == symbol.key))
                    .is_some_and(|def_type| match (&def_type.kind, b) {
                        (
                            DefTypeKind::Struct(..),
                            HeapType::Any | HeapType::Eq | HeapType::Struct,
                        ) => true,
                        (
                            DefTypeKind::Array(..),
                            HeapType::Any | HeapType::Eq | HeapType::Array,
                        ) => true,
                        (DefTypeKind::Func(..), HeapType::Func) => true,
                        _ => false,
                    })
            }
            (a, b) => a == b,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum OperandType {
    Val(ValType),
    Any,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Fields(pub Vec<(FieldType, Option<InternIdent>)>);
impl Fields {
    pub(crate) fn matches(
        &self,
        other: &Self,
        db: &dyn TypesAnalyzerCtx,
        uri: InternUri,
        module_id: u32,
    ) -> bool {
        self.0.len() >= other.0.len()
            && self
                .0
                .iter()
                .zip(&other.0)
                .all(|((a, _), (b, _))| a.matches(b, db, uri, module_id))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FieldType {
    pub(super) storage: StorageType,
    pub(super) mutable: bool,
}
impl FieldType {
    pub(super) fn from_ast(node: &AstFieldType, db: &dyn TypesAnalyzerCtx) -> Option<Self> {
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
        db: &dyn TypesAnalyzerCtx,
        uri: InternUri,
        module_id: u32,
    ) -> bool {
        match (self.mutable, other.mutable) {
            (true, true) => {
                self.storage.matches(&other.storage, db, uri, module_id)
                    && other.storage.matches(&self.storage, db, uri, module_id)
            }
            (false, false) => self.storage.matches(&other.storage, db, uri, module_id),
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum StorageType {
    Val(ValType),
    PackedI8,
    PackedI16,
}
impl StorageType {
    fn from_ast(node: &AstStorageType, db: &dyn TypesAnalyzerCtx) -> Option<Self> {
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
        db: &dyn TypesAnalyzerCtx,
        uri: InternUri,
        module_id: u32,
    ) -> bool {
        match (self, other) {
            (StorageType::Val(a), StorageType::Val(b)) => a.matches(b, db, uri, module_id),
            (StorageType::PackedI8, StorageType::PackedI8) => true,
            (StorageType::PackedI16, StorageType::PackedI16) => true,
            _ => false,
        }
    }
}
