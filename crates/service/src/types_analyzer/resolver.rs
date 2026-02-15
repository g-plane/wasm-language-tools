use super::{
    def_type::{CompositeType, DefType, DefTypes, get_def_types},
    signature::get_func_sig,
    types::{FieldType, Fields, OperandType},
};
use crate::{
    binder::{SymbolKey, SymbolKind, SymbolTable},
    data_set::INSTR_SIG,
    document::Document,
    idx::Idx,
};
use wat_syntax::{SyntaxKind, SyntaxNode, SyntaxNodePtr, ast::support};

pub(crate) fn resolve_param_types<'db>(
    db: &'db dyn salsa::Database,
    document: Document,
    instr: &SyntaxNode,
) -> Option<Vec<OperandType<'db>>> {
    debug_assert!(instr.kind() == SyntaxKind::PLAIN_INSTR);
    let instr_name = support::token(instr, SyntaxKind::INSTR_NAME)?;
    let instr_name = instr_name.text();
    if matches!(instr_name, "call" | "return_call") {
        let symbol_table = SymbolTable::of(db, document);
        let idx = instr.children_by_kind(|kind| kind == SyntaxKind::IMMEDIATE).next()?;
        let func = symbol_table.find_def(SymbolKey::new(&idx))?;
        Some(
            get_func_sig(db, document, func.key, &func.green)
                .params
                .into_iter()
                .map(|(ty, ..)| OperandType::Val(ty))
                .collect(),
        )
    } else {
        INSTR_SIG.get(instr_name).map(|sig| sig.params.clone())
    }
}

pub(crate) fn resolve_br_types<'db>(
    db: &'db dyn salsa::Database,
    document: Document,
    symbol_table: &'db SymbolTable<'db>,
    ref_key: SymbolKey,
) -> Option<impl DoubleEndedIterator<Item = OperandType<'db>> + use<'db>> {
    symbol_table.find_def(ref_key).map(|def_symbol| {
        get_func_sig(db, document, def_symbol.key, &def_symbol.green)
            .results
            .into_iter()
            .map(OperandType::Val)
    })
}

pub(crate) fn resolve_array_type_with_idx<'db>(
    symbol_table: &SymbolTable,
    def_types: &DefTypes<'db>,
    immediate: SyntaxNodePtr,
) -> Option<(Idx<'db>, Option<OperandType<'db>>)> {
    symbol_table
        .resolved
        .get(&immediate.into())
        .and_then(|key| def_types.get(key))
        .map(|def_type| {
            if let CompositeType::Array(field) = &def_type.comp {
                (def_type.idx, field.as_ref().map(|field| field.storage.clone().into()))
            } else {
                (def_type.idx, None)
            }
        })
}

pub(crate) fn resolve_field_type<'db>(
    db: &'db dyn salsa::Database,
    document: Document,
    key: SymbolKey,
    region: SymbolKey,
) -> Option<FieldType<'db>> {
    let symbol_table = SymbolTable::of(db, document);
    let def_types = get_def_types(db, document);
    let symbol = symbol_table.symbols.get(&key)?;
    let field_def_symbol = match symbol.kind {
        SymbolKind::FieldDef => symbol,
        SymbolKind::FieldRef => symbol_table.symbols.values().find(|other| {
            other.kind == SymbolKind::FieldDef && other.region == region && symbol.idx.is_defined_by(&other.idx)
        })?,
        _ => return None,
    };
    let idx = field_def_symbol.idx.num?;
    if let Some(DefType {
        comp: CompositeType::Struct(Fields(fields)),
        ..
    }) = def_types.get(&field_def_symbol.region)
    {
        fields
            .iter()
            .enumerate()
            .find(|(i, _)| *i as u32 == idx)
            .map(|(_, (field, _))| field.clone())
    } else {
        None
    }
}

pub(crate) fn resolve_field_type_with_struct_idx<'db>(
    db: &'db dyn salsa::Database,
    document: Document,
    struct_ref: SyntaxNodePtr,
    field_ref: SyntaxNodePtr,
) -> Option<(Idx<'db>, Option<OperandType<'db>>)> {
    let symbol_table = SymbolTable::of(db, document);
    let struct_def_symbol = symbol_table.find_def(struct_ref.into())?;
    let ty = resolve_field_type(db, document, field_ref.into(), struct_def_symbol.key).map(|ty| ty.into());
    Some((struct_def_symbol.idx, ty))
}
