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
            let mut is_final = false;
            let mut inherits = None;
            if let Some(sub_type) = node.sub_type() {
                is_final = sub_type.keyword().is_none() || sub_type.final_keyword().is_some();
                if let Some(index) = sub_type.indexes().next() {
                    inherits =
                        symbol_table
                            .find_def(SymbolKey::new(index.syntax()))
                            .map(|symbol| Inherits {
                                symbol: symbol.key,
                                idx: Idx {
                                    num: index
                                        .unsigned_int_token()
                                        .and_then(|int| int.text().parse().ok()),
                                    name: index
                                        .ident_token()
                                        .map(|ident| db.ident(ident.text().into())),
                                },
                            });
                }
            }
            let kind = match node.sub_type()?.comp_type()? {
                CompType::Func(func_type) => {
                    DefTypeKind::Func(db.extract_sig(func_type.syntax().green().into()))
                }
                CompType::Struct(struct_type) => {
                    DefTypeKind::Struct(extract_fields(db, &struct_type))
                }
                CompType::Array(array_type) => DefTypeKind::Array(
                    array_type
                        .field_type()
                        .and_then(|node| FieldType::from_ast(&node, db)),
                ),
            };
            Some(DefType {
                key: symbol.key,
                idx: symbol.idx,
                is_final,
                inherits,
                kind,
            })
        })
        .collect();
    Arc::new(types)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DefType {
    pub key: SymbolKey,
    pub idx: Idx,
    pub is_final: bool,
    pub inherits: Option<Inherits>,
    pub kind: DefTypeKind,
}
impl DefType {
    pub(crate) fn type_equals(
        &self,
        other: &Self,
        db: &dyn TypesAnalyzerCtx,
        uri: InternUri,
        module_id: u32,
    ) -> bool {
        if !self.kind.type_equals(&other.kind, db, uri, module_id) {
            return false;
        }
        if self.is_final != other.is_final {
            return false;
        }
        let def_types = db.def_types(uri);
        match (
            self.inherits
                .as_ref()
                .and_then(|a| def_types.iter().find(|def_type| def_type.key == a.symbol)),
            other
                .inherits
                .as_ref()
                .and_then(|b| def_types.iter().find(|def_type| def_type.key == b.symbol)),
        ) {
            (Some(a), Some(b)) => {
                if !a.type_equals(b, db, uri, module_id) {
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
            a_index == b_index && a_group.type_equals(b_group, db, uri, module_id)
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Inherits {
    pub(crate) symbol: SymbolKey,
    pub(crate) idx: Idx,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum DefTypeKind {
    Func(Signature),
    Struct(Fields),
    Array(Option<FieldType>),
}
impl DefTypeKind {
    pub(crate) fn type_equals(
        &self,
        other: &Self,
        db: &dyn TypesAnalyzerCtx,
        uri: InternUri,
        module_id: u32,
    ) -> bool {
        match (self, other) {
            (DefTypeKind::Func(a), DefTypeKind::Func(b)) => a.type_equals(b, db, uri, module_id),
            (DefTypeKind::Struct(a), DefTypeKind::Struct(b)) => {
                a.type_equals(b, db, uri, module_id)
            }
            (DefTypeKind::Array(Some(a)), DefTypeKind::Array(Some(b))) => {
                a.type_equals(b, db, uri, module_id)
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
    pub(crate) fn type_equals(
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
                    // check whether their super types equal or not
                    match (&a.inherits, &b.inherits) {
                        (Some(Inherits { idx: a, .. }), Some(Inherits { idx: b, .. })) => {
                            let mut a = HeapType::Type(*a);
                            let mut b = HeapType::Type(*b);
                            if substitute_heap_type(&mut a, &symbol_table, module.key, self)
                                .is_err()
                                || substitute_heap_type(&mut b, &symbol_table, module.key, other)
                                    .is_err()
                                || !a.type_equals(&b, db, uri, module_id)
                            {
                                return false;
                            }
                        }
                        (None, None) => {}
                        _ => return false,
                    }
                    // check whether their composite types equal or not
                    let mut a = a.clone();
                    let mut b = b.clone();
                    substitute_def_type(&mut a, &symbol_table, module.key, self).is_ok()
                        && substitute_def_type(&mut b, &symbol_table, module.key, other).is_ok()
                        && a.kind.type_equals(&b.kind, db, uri, module_id)
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
    match &mut def_type.kind {
        DefTypeKind::Func(signature) => {
            signature.params.iter_mut().try_for_each(|(param, name)| {
                if let ValType::Ref(RefType { heap_ty, .. }) = param {
                    substitute_heap_type(heap_ty, symbol_table, module, rec_group)?;
                }
                *name = None;
                Ok(())
            })?;
            signature.results.iter_mut().try_for_each(|result| {
                if let ValType::Ref(RefType { heap_ty, .. }) = result {
                    substitute_heap_type(heap_ty, symbol_table, module, rec_group)?;
                }
                Ok(())
            })
        }
        DefTypeKind::Struct(fields) => fields.0.iter_mut().try_for_each(|(field, name)| {
            if let FieldType {
                storage: StorageType::Val(ValType::Ref(RefType { heap_ty, .. })),
                ..
            } = field
            {
                substitute_heap_type(heap_ty, symbol_table, module, rec_group)?;
            }
            *name = None;
            Ok(())
        }),
        DefTypeKind::Array(field) => {
            if let Some(FieldType {
                storage: StorageType::Val(ValType::Ref(RefType { heap_ty, .. })),
                ..
            }) = field
            {
                substitute_heap_type(heap_ty, symbol_table, module, rec_group)?;
            }
            Ok(())
        }
    }
}
fn substitute_heap_type(
    heap_type: &mut HeapType,
    symbol_table: &SymbolTable,
    module: SymbolKey,
    rec_group: &RecTypeGroup,
) -> Result<(), ()> {
    if let HeapType::Type(idx) = heap_type {
        if let Some(symbol) = symbol_table.symbols.iter().find(|symbol| {
            symbol.kind == SymbolKind::Type
                && symbol.region == module
                && idx.is_defined_by(&symbol.idx)
        }) {
            if let Some(i) = rec_group
                .type_defs
                .iter()
                .position(|key| *key == symbol.key)
            {
                *heap_type = HeapType::Rec(i as u32);
            } else if symbol.key.text_range().start() > rec_group.range.end() {
                return Err(());
            } else {
                *heap_type = HeapType::Type(Idx {
                    num: symbol.idx.num,
                    name: None,
                });
            }
        }
    }
    Ok(())
}
