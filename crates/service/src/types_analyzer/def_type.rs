use super::{
    extractor::{extract_fields, extract_sig},
    signature::Signature,
    types::{FieldType, Fields, HeapType, RefType, StorageType, ValType},
};
use crate::{
    binder::{SymbolKey, SymbolKind, SymbolTable},
    document::Document,
    idx::{Idx, InternIdent},
};
use rowan::{TextRange, ast::AstNode};
use rustc_hash::FxHashMap;
use wat_syntax::ast::{CompType, ModuleField, Root, TypeDef};

#[salsa::tracked(returns(ref))]
pub(crate) fn get_def_types(db: &dyn salsa::Database, document: Document) -> DefTypes<'_> {
    let root = document.root_tree(db);
    let symbol_table = SymbolTable::of(db, document);
    symbol_table
        .symbols
        .values()
        .filter(|symbol| symbol.kind == SymbolKind::Type)
        .filter_map(|symbol| {
            let node = TypeDef::cast(symbol.key.to_node(&root))?;
            let mut is_final = false;
            let mut inherits = None;
            if let Some(sub_type) = node.sub_type() {
                is_final = sub_type.keyword().is_none() || sub_type.final_keyword().is_some();
                if let Some(index) = sub_type.indexes().next() {
                    inherits = symbol_table
                        .resolved
                        .get(&SymbolKey::new(index.syntax()))
                        .map(|key| Inherits {
                            symbol: *key,
                            idx: Idx {
                                num: index
                                    .unsigned_int_token()
                                    .and_then(|int| int.text().parse().ok()),
                                name: index
                                    .ident_token()
                                    .map(|ident| InternIdent::new(db, ident.text())),
                            },
                        });
                }
            }
            let comp = match node.sub_type()?.comp_type()? {
                CompType::Func(func_type) => {
                    CompositeType::Func(extract_sig(db, &func_type.syntax().green()))
                }
                CompType::Struct(struct_type) => {
                    CompositeType::Struct(extract_fields(db, &struct_type))
                }
                CompType::Array(array_type) => CompositeType::Array(
                    array_type
                        .field_type()
                        .and_then(|node| FieldType::from_ast(&node, db)),
                ),
            };
            Some((
                symbol.key,
                DefType {
                    key: symbol.key,
                    idx: symbol.idx,
                    is_final,
                    inherits,
                    comp,
                },
            ))
        })
        .collect()
}

pub(crate) type DefTypes<'db> = FxHashMap<SymbolKey, DefType<'db>>;

#[derive(Clone, Debug, PartialEq, Eq, salsa::Update)]
pub(crate) struct DefType<'db> {
    pub key: SymbolKey,
    pub idx: Idx<'db>,
    pub is_final: bool,
    pub inherits: Option<Inherits<'db>>,
    pub comp: CompositeType<'db>,
}
impl<'db> DefType<'db> {
    pub(crate) fn type_equals(
        &self,
        other: &Self,
        db: &'db dyn salsa::Database,
        document: Document,
        module_id: u32,
    ) -> bool {
        if !self.comp.type_equals(&other.comp, db, document, module_id) {
            return false;
        }
        if self.is_final != other.is_final {
            return false;
        }
        let def_types = get_def_types(db, document);
        match (
            self.inherits
                .as_ref()
                .and_then(|a| def_types.get(&a.symbol)),
            other
                .inherits
                .as_ref()
                .and_then(|b| def_types.get(&b.symbol)),
        ) {
            (Some(a), Some(b)) => {
                if !a.type_equals(b, db, document, module_id) {
                    return false;
                }
            }
            (None, None) => {}
            _ => return false,
        }
        let rec_type_groups = get_rec_type_groups(db, document);
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
            a_index == b_index && a_group.type_equals(b_group, db, document, module_id)
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, salsa::Update)]
pub(crate) struct Inherits<'db> {
    pub(crate) symbol: SymbolKey,
    pub(crate) idx: Idx<'db>,
}

#[derive(Clone, Debug, PartialEq, Eq, salsa::Update)]
pub(crate) enum CompositeType<'db> {
    Func(Signature<'db>),
    Struct(Fields<'db>),
    Array(Option<FieldType<'db>>),
}
impl<'db> CompositeType<'db> {
    pub(crate) fn matches(
        &self,
        other: &Self,
        db: &'db dyn salsa::Database,
        document: Document,
        module_id: u32,
    ) -> bool {
        match (self, other) {
            (CompositeType::Func(a), CompositeType::Func(b)) => {
                a.matches(b, db, document, module_id)
            }
            (CompositeType::Struct(a), CompositeType::Struct(b)) => {
                a.matches(b, db, document, module_id)
            }
            (CompositeType::Array(Some(a)), CompositeType::Array(Some(b))) => {
                a.matches(b, db, document, module_id)
            }
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
            (CompositeType::Func(a), CompositeType::Func(b)) => {
                a.type_equals(b, db, document, module_id)
            }
            (CompositeType::Struct(a), CompositeType::Struct(b)) => {
                a.type_equals(b, db, document, module_id)
            }
            (CompositeType::Array(Some(a)), CompositeType::Array(Some(b))) => {
                a.type_equals(b, db, document, module_id)
            }
            _ => false,
        }
    }

    pub(crate) fn as_func(&self) -> Option<&Signature<'db>> {
        if let CompositeType::Func(sig) = self {
            Some(sig)
        } else {
            None
        }
    }
}

#[salsa::tracked(returns(ref))]
pub(crate) fn get_rec_type_groups<'db>(
    db: &'db dyn salsa::Database,
    document: Document,
) -> Vec<RecTypeGroup> {
    let root = Root::cast(document.root_tree(db)).expect("expected root tree");
    let symbol_table = SymbolTable::of(db, document);
    root.modules()
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
                        .values()
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
        .collect()
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
        db: &dyn salsa::Database,
        document: Document,
        module_id: u32,
    ) -> bool {
        let symbol_table = SymbolTable::of(db, document);
        let def_types = get_def_types(db, document);
        self.type_defs
            .iter()
            .map(|key| def_types.get(key))
            .zip(other.type_defs.iter().map(|key| def_types.get(key)))
            .all(|(a, b)| {
                if let (Some(a), Some(b), Some(module)) =
                    (a, b, symbol_table.find_module(module_id))
                {
                    // check whether their super types equal or not
                    match (&a.inherits, &b.inherits) {
                        (Some(Inherits { idx: a, .. }), Some(Inherits { idx: b, .. })) => {
                            let mut a = HeapType::Type(*a);
                            let mut b = HeapType::Type(*b);
                            if substitute_heap_type(&mut a, symbol_table, module.key, self).is_err()
                                || substitute_heap_type(&mut b, symbol_table, module.key, other)
                                    .is_err()
                                || !a.type_equals(&b, db, document, module_id)
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
                    substitute_def_type(&mut a, symbol_table, module.key, self).is_ok()
                        && substitute_def_type(&mut b, symbol_table, module.key, other).is_ok()
                        && a.comp.type_equals(&b.comp, db, document, module_id)
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
fn substitute_def_type<'db>(
    def_type: &mut DefType<'db>,
    symbol_table: &SymbolTable<'db>,
    module: SymbolKey,
    rec_group: &RecTypeGroup,
) -> Result<(), ()> {
    match &mut def_type.comp {
        CompositeType::Func(signature) => {
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
        CompositeType::Struct(fields) => fields.0.iter_mut().try_for_each(|(field, idx)| {
            if let FieldType {
                storage: StorageType::Val(ValType::Ref(RefType { heap_ty, .. })),
                ..
            } = field
            {
                substitute_heap_type(heap_ty, symbol_table, module, rec_group)?;
            }
            idx.name = None;
            Ok(())
        }),
        CompositeType::Array(field) => {
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
fn substitute_heap_type<'db>(
    heap_type: &mut HeapType<'db>,
    symbol_table: &SymbolTable<'db>,
    module: SymbolKey,
    rec_group: &RecTypeGroup,
) -> Result<(), ()> {
    if let HeapType::Type(idx) = heap_type
        && let Some(symbol) = symbol_table.symbols.values().find(|symbol| {
            symbol.kind == SymbolKind::Type
                && symbol.region == module
                && idx.is_defined_by(&symbol.idx)
        })
    {
        if let Some(i) = rec_group
            .type_defs
            .iter()
            .position(|key| *key == symbol.key)
        {
            *heap_type = HeapType::Rec(i as u32);
        } else if symbol.key.text_range().start() > rec_group.range.end() {
            return Err(());
        } else {
            *heap_type = HeapType::Type(symbol.idx);
        }
    }
    Ok(())
}
