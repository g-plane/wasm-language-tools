use super::{
    extractor::extract_fields,
    signature::Signature,
    types::{FieldType, Fields, HeapType, RefType, StorageType, ValType},
    TypesAnalyzerCtx,
};
use crate::{
    binder::{SymbolKey, SymbolKind, SymbolTable},
    idx::Idx,
    uri::InternUri,
};
use rowan::ast::AstNode;
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
            match node.sub_type()?.comp_type()? {
                CompType::Func(func_type) => Some(DefType {
                    key: symbol.key,
                    idx: symbol.idx,
                    kind: DefTypeKind::Func(db.extract_sig(func_type.syntax().green().into())),
                }),
                CompType::Struct(struct_type) => Some(DefType {
                    key: symbol.key,
                    idx: symbol.idx,
                    kind: DefTypeKind::Struct(extract_fields(db, &struct_type)),
                }),
                CompType::Array(array_type) => Some(DefType {
                    key: symbol.key,
                    idx: symbol.idx,
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
            ModuleField::Type(type_def) => Some(RecTypeGroup {
                type_defs: vec![SymbolKey::new(type_def.syntax())],
                closed: true,
            }),
            ModuleField::RecType(rec_type) => {
                let rec_range = rec_type.syntax().text_range();
                let symbols = symbol_table
                    .symbols
                    .iter()
                    .filter(|symbol| {
                        symbol.kind == SymbolKind::Type
                            && rec_range.contains_range(symbol.key.text_range())
                    })
                    .collect::<Vec<_>>();
                Some(RecTypeGroup {
                    type_defs: symbols.iter().map(|symbol| symbol.key).collect(),
                    closed: symbol_table
                        .symbols
                        .iter()
                        .filter(|symbol| {
                            symbol.kind == SymbolKind::TypeUse
                                && rec_range.contains_range(symbol.key.text_range())
                        })
                        .all(|ref_symbol| {
                            symbols
                                .iter()
                                .any(|def_symbol| ref_symbol.idx.is_defined_by(&def_symbol.idx))
                        }),
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
    pub(crate) closed: bool,
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
        self.closed
            && other.closed
            && self
                .type_defs
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
                        rollup_def_type(&mut a, &symbol_table, module.key, self);
                        rollup_def_type(&mut b, &symbol_table, module.key, other);
                        // We can't use `a.kind.matches(&b.kind, ...)` here;
                        // otherwise it will cause infinite loop.
                        a.kind == b.kind
                    } else {
                        false
                    }
                })
    }
}
fn rollup_def_type(
    def_type: &mut DefType,
    symbol_table: &SymbolTable,
    module: SymbolKey,
    rec_group: &RecTypeGroup,
) {
    fn search_index_in_rec_group(
        idx: &Idx,
        symbol_table: &SymbolTable,
        module: SymbolKey,
        rec_group: &RecTypeGroup,
    ) -> Option<u32> {
        symbol_table
            .symbols
            .iter()
            .find(|symbol| {
                symbol.kind == SymbolKind::Type
                    && symbol.region == module
                    && idx.is_defined_by(&symbol.idx)
            })
            .and_then(|symbol| {
                rec_group
                    .type_defs
                    .iter()
                    .position(|key| *key == symbol.key)
            })
            .map(|i| i as u32)
    }
    match &mut def_type.kind {
        DefTypeKind::Func(signature) => {
            signature.params.iter_mut().for_each(|(param, name)| {
                if let ValType::Ref(RefType {
                    heap_ty: HeapType::Type(idx),
                    ..
                }) = param
                {
                    *idx = Idx {
                        num: search_index_in_rec_group(idx, symbol_table, module, rec_group),
                        name: None,
                    };
                }
                *name = None;
            });
            signature.results.iter_mut().for_each(|result| {
                if let ValType::Ref(RefType {
                    heap_ty: HeapType::Type(idx),
                    ..
                }) = result
                {
                    *idx = Idx {
                        num: search_index_in_rec_group(idx, symbol_table, module, rec_group),
                        name: None,
                    };
                }
            });
        }
        DefTypeKind::Struct(fields) => {
            fields.0.iter_mut().for_each(|(field, name)| {
                if let FieldType {
                    storage:
                        StorageType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(idx),
                            ..
                        })),
                    ..
                } = field
                {
                    *idx = Idx {
                        num: search_index_in_rec_group(idx, symbol_table, module, rec_group),
                        name: None,
                    };
                }
                *name = None;
            });
        }
        DefTypeKind::Array(Some(FieldType {
            storage:
                StorageType::Val(ValType::Ref(RefType {
                    heap_ty: HeapType::Type(idx),
                    ..
                })),
            ..
        })) => {
            *idx = Idx {
                num: search_index_in_rec_group(idx, symbol_table, module, rec_group),
                name: None,
            };
        }
        _ => {}
    }
}
