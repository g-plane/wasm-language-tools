use super::{
    extractor::extract_fields,
    signature::Signature,
    types::{FieldType, Fields, HeapType, RefType, StorageType, ValType},
    TypesAnalyzerCtx,
};
use crate::{
    binder::{Symbol, SymbolKey, SymbolKind, SymbolTable},
    idx::Idx,
    uri::InternUri,
};
use rowan::{ast::AstNode, TextRange};
use std::sync::Arc;
use wat_syntax::{
    ast::{CompType, ModuleField, Root, TypeDef},
    SyntaxNode,
};

pub(super) fn create_def_types(db: &dyn TypesAnalyzerCtx, uri: InternUri) -> Arc<Vec<DefType>> {
    let root = SyntaxNode::new_root(db.root(uri));
    let symbol_table = db.symbol_table(uri);
    let types = symbol_table
        .symbols
        .iter()
        .filter(|symbol| symbol.kind == SymbolKind::Type)
        .filter_map(|symbol| {
            let node = TypeDef::cast(symbol.key.to_node(&root))?;
            let mut can_be_super = false;
            let mut inherits = None;
            if let Some(sub_type) = node.sub_type() {
                can_be_super = sub_type.keyword().is_some() && sub_type.final_keyword().is_none();
                inherits = sub_type
                    .indexes()
                    .next()
                    .and_then(|index| symbol_table.find_def(SymbolKey::new(index.syntax())))
                    .map(|def| def.key);
            }
            match node.sub_type()?.comp_type()? {
                CompType::Func(func_type) => Some(DefType {
                    key: symbol.key,
                    idx: symbol.idx,
                    can_be_super,
                    inherits,
                    kind: DefTypeKind::Func(db.extract_sig(func_type.syntax().green().into())),
                }),
                CompType::Struct(struct_type) => Some(DefType {
                    key: symbol.key,
                    idx: symbol.idx,
                    can_be_super,
                    inherits,
                    kind: DefTypeKind::Struct(extract_fields(db, &struct_type)),
                }),
                CompType::Array(array_type) => Some(DefType {
                    key: symbol.key,
                    idx: symbol.idx,
                    can_be_super,
                    inherits,
                    kind: DefTypeKind::Array(
                        array_type
                            .field_type()
                            .and_then(|node| FieldType::from_ast(&node, db)),
                    ),
                }),
            }
        })
        .collect();
    Arc::new(types)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DefType {
    pub key: SymbolKey,
    pub idx: Idx,
    pub can_be_super: bool,
    pub inherits: Option<SymbolKey>,
    pub kind: DefTypeKind,
}
impl DefType {
    pub(crate) fn matches(
        &self,
        other: &Self,
        db: &dyn TypesAnalyzerCtx,
        uri: InternUri,
        module_id: u32,
    ) -> bool {
        if !self.kind.matches(&other.kind, db, uri, module_id) {
            return false;
        }
        if self.can_be_super != other.can_be_super {
            return false;
        }
        let def_types = db.def_types(uri);
        match (
            self.inherits
                .and_then(|a| def_types.iter().find(|def_type| def_type.key == a)),
            other
                .inherits
                .and_then(|b| def_types.iter().find(|def_type| def_type.key == b)),
        ) {
            (Some(a), Some(b)) => {
                if !a.matches(b, db, uri, module_id) {
                    return false;
                }
            }
            (None, None) => {}
            _ => return false,
        }
        let rec_type_groups = db.rec_type_groups(uri);
        if let Some(((a_group, a_index), (b_group, b_index))) = rec_type_groups
            .iter()
            .find_map(|group| {
                group
                    .type_defs
                    .iter()
                    .position(|key| *key == self.key)
                    .map(|i| (group, i))
            })
            .zip(rec_type_groups.iter().find_map(|group| {
                group
                    .type_defs
                    .iter()
                    .position(|key| *key == other.key)
                    .map(|i| (group, i))
            }))
        {
            a_index == b_index && a_group.equals(b_group, db, uri, module_id)
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum DefTypeKind {
    Func(Signature),
    Struct(Fields),
    Array(Option<FieldType>),
}
impl DefTypeKind {
    pub(crate) fn matches(
        &self,
        other: &Self,
        db: &dyn TypesAnalyzerCtx,
        uri: InternUri,
        module_id: u32,
    ) -> bool {
        match (self, other) {
            (DefTypeKind::Func(a), DefTypeKind::Func(b)) => a.matches(b, db, uri, module_id),
            (DefTypeKind::Struct(a), DefTypeKind::Struct(b)) => a.matches(b, db, uri, module_id),
            (DefTypeKind::Array(Some(a)), DefTypeKind::Array(Some(b))) => {
                a.matches(b, db, uri, module_id)
            }
            _ => false,
        }
    }
}

pub(super) fn create_recursive_types(
    db: &dyn TypesAnalyzerCtx,
    uri: InternUri,
) -> Arc<Vec<RecTypeGroup>> {
    let root = Root::cast(SyntaxNode::new_root(db.root(uri))).expect("expected root tree");
    let symbol_table = db.symbol_table(uri);
    let recursive_types = root
        .modules()
        .flat_map(|module| module.module_fields())
        .filter_map(|module_field| match module_field {
            ModuleField::Type(type_def) => {
                let node = type_def.syntax();
                Some(RecTypeGroup {
                    type_defs: vec![SymbolKey::new(node)],
                    range: node.text_range(),
                })
            }
            ModuleField::RecType(rec_type) => {
                let rec_range = rec_type.syntax().text_range();
                Some(RecTypeGroup {
                    type_defs: symbol_table
                        .symbols
                        .iter()
                        .filter(|symbol| {
                            symbol.kind == SymbolKind::Type
                                && rec_range.contains_range(symbol.key.text_range())
                        })
                        .map(|symbol| symbol.key)
                        .collect(),
                    range: rec_range,
                })
            }
            _ => None,
        })
        .collect();
    Arc::new(recursive_types)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RecTypeGroup {
    pub(crate) type_defs: Vec<SymbolKey>,
    pub(crate) range: TextRange,
}
impl RecTypeGroup {
    pub(crate) fn equals(
        &self,
        other: &Self,
        db: &dyn TypesAnalyzerCtx,
        uri: InternUri,
        module_id: u32,
    ) -> bool {
        let symbol_table = db.symbol_table(uri);
        let def_types = db.def_types(uri);
        self.type_defs
            .iter()
            .map(|key| def_types.iter().find(|def_type| def_type.key == *key))
            .zip(
                other
                    .type_defs
                    .iter()
                    .map(|key| def_types.iter().find(|def_type| def_type.key == *key)),
            )
            .all(|(a, b)| {
                if let (Some(a), Some(b), Some(module)) =
                    (a, b, symbol_table.find_module(module_id))
                {
                    let mut a = a.clone();
                    let mut b = b.clone();
                    if substitute_def_type(&mut a, &symbol_table, module.key, self).is_err() {
                        return false;
                    }
                    if substitute_def_type(&mut b, &symbol_table, module.key, other).is_err() {
                        return false;
                    }
                    a.kind.matches(&b.kind, db, uri, module_id)
                } else {
                    false
                }
            })
    }
}
/// Substitute typeidx.
/// If they're in the given recursive types group, substitute it with its index in that rec group.
/// If not, substitute it with a numeric-only idx for better comparison in later use.
///
/// This function will return Err if its typeidx is greater than all types in the rec group
/// (defined after the rec group).
/// This case is invalid according to WasmGC spec so this behaivor makes sense.
/// Additionally, this will prevent typeidx resolution infinite loop,
/// because this ensures that resolution is backward only.
fn substitute_def_type(
    def_type: &mut DefType,
    symbol_table: &SymbolTable,
    module: SymbolKey,
    rec_group: &RecTypeGroup,
) -> Result<(), ()> {
    fn find_type_def<'a>(
        idx: &Idx,
        symbol_table: &'a SymbolTable,
        module: SymbolKey,
    ) -> Option<&'a Symbol> {
        symbol_table.symbols.iter().find(|symbol| {
            symbol.kind == SymbolKind::Type
                && symbol.region == module
                && idx.is_defined_by(&symbol.idx)
        })
    }
    fn search_index_in_rec_group(symbol: &Symbol, rec_group: &RecTypeGroup) -> Option<u32> {
        rec_group
            .type_defs
            .iter()
            .position(|key| *key == symbol.key)
            .map(|i| i as u32)
    }
    match &mut def_type.kind {
        DefTypeKind::Func(signature) => {
            signature.params.iter_mut().try_for_each(|(param, name)| {
                if let ValType::Ref(RefType {
                    heap_ty: HeapType::Type(idx),
                    nullable,
                }) = param
                {
                    if let Some(symbol) = find_type_def(idx, symbol_table, module) {
                        if let Some(i) = search_index_in_rec_group(symbol, rec_group) {
                            *param = ValType::Ref(RefType {
                                heap_ty: HeapType::Rec(i),
                                nullable: *nullable,
                            });
                        } else if symbol.key.text_range().start() > rec_group.range.end() {
                            return Err(());
                        } else {
                            *idx = Idx {
                                num: symbol.idx.num,
                                name: None,
                            };
                        }
                    }
                }
                *name = None;
                Ok(())
            })?;
            signature.results.iter_mut().try_for_each(|result| {
                if let ValType::Ref(RefType {
                    heap_ty: HeapType::Type(idx),
                    nullable,
                }) = result
                {
                    if let Some(symbol) = find_type_def(idx, symbol_table, module) {
                        if let Some(i) = search_index_in_rec_group(symbol, rec_group) {
                            *result = ValType::Ref(RefType {
                                heap_ty: HeapType::Rec(i),
                                nullable: *nullable,
                            });
                        } else if symbol.key.text_range().start() > rec_group.range.end() {
                            return Err(());
                        } else {
                            *idx = Idx {
                                num: symbol.idx.num,
                                name: None,
                            };
                        }
                    }
                }
                Ok(())
            })
        }
        DefTypeKind::Struct(fields) => fields.0.iter_mut().try_for_each(|(field, name)| {
            if let FieldType {
                storage:
                    StorageType::Val(ValType::Ref(RefType {
                        heap_ty: HeapType::Type(idx),
                        nullable,
                    })),
                mutable,
            } = field
            {
                if let Some(symbol) = find_type_def(idx, symbol_table, module) {
                    if let Some(i) = search_index_in_rec_group(symbol, rec_group) {
                        *field = FieldType {
                            storage: StorageType::Val(ValType::Ref(RefType {
                                heap_ty: HeapType::Rec(i),
                                nullable: *nullable,
                            })),
                            mutable: *mutable,
                        };
                    } else if symbol.key.text_range().start() > rec_group.range.end() {
                        return Err(());
                    } else {
                        *idx = Idx {
                            num: symbol.idx.num,
                            name: None,
                        };
                    }
                }
            }
            *name = None;
            Ok(())
        }),
        DefTypeKind::Array(field) => {
            if let Some(FieldType {
                storage:
                    StorageType::Val(ValType::Ref(RefType {
                        heap_ty: HeapType::Type(idx),
                        nullable,
                    })),
                mutable,
            }) = field
            {
                if let Some(symbol) = find_type_def(idx, symbol_table, module) {
                    if let Some(i) = search_index_in_rec_group(symbol, rec_group) {
                        *field = Some(FieldType {
                            storage: StorageType::Val(ValType::Ref(RefType {
                                heap_ty: HeapType::Rec(i),
                                nullable: *nullable,
                            })),
                            mutable: *mutable,
                        });
                    } else if symbol.key.text_range().start() > rec_group.range.end() {
                        return Err(());
                    } else {
                        *idx = Idx {
                            num: symbol.idx.num,
                            name: None,
                        };
                    }
                }
            }
            Ok(())
        }
    }
}
