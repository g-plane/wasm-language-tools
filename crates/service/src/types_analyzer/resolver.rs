use super::{
    def_type::{CompositeType, DefType, DefTypes, get_def_types},
    signature::{get_block_sig, get_func_sig},
    types::{FieldType, Fields, OperandType},
};
use crate::{
    binder::{SymbolKey, SymbolKind, SymbolTable},
    data_set::INSTR_SIG,
    document::Document,
    idx::Idx,
};
use rowan::ast::{AstNode, support};
use wat_syntax::{SyntaxKind, SyntaxNode, ast::Immediate};

pub(crate) fn resolve_param_types<'db>(
    service: &'db dyn salsa::Database,
    document: Document,
    instr: &SyntaxNode,
) -> Option<Vec<OperandType<'db>>> {
    debug_assert!(instr.kind() == SyntaxKind::PLAIN_INSTR);
    let instr_name = support::token(instr, SyntaxKind::INSTR_NAME)?;
    let instr_name = instr_name.text();
    if matches!(instr_name, "call" | "return_call") {
        let symbol_table = SymbolTable::of(service, document);
        let idx = instr
            .children()
            .find(|child| child.kind() == SyntaxKind::IMMEDIATE)?;
        let func = symbol_table.find_def(SymbolKey::new(&idx))?;
        Some(
            get_func_sig(service, document, *func.key, &func.green)
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
    service: &'db dyn salsa::Database,
    document: Document,
    symbol_table: &'db SymbolTable<'db>,
    immediate: &Immediate,
) -> Vec<OperandType<'db>> {
    let key = SymbolKey::new(immediate.syntax());
    symbol_table
        .blocks
        .iter()
        .find(|block| block.ref_key == key)
        .map(|block| {
            get_block_sig(
                service,
                document,
                &block.def_key.to_node(&document.root_tree(service)),
            )
            .results
            .into_iter()
            .map(OperandType::Val)
            .collect()
        })
        .unwrap_or_default()
}

pub(crate) fn resolve_array_type_with_idx<'db>(
    symbol_table: &SymbolTable,
    def_types: &DefTypes<'db>,
    immediate: &Immediate,
) -> Option<(Idx<'db>, Option<OperandType<'db>>)> {
    symbol_table
        .find_def(SymbolKey::new(immediate.syntax()))
        .and_then(|symbol| def_types.get(&symbol.key))
        .map(|def_type| {
            if let CompositeType::Array(field) = &def_type.comp {
                (
                    def_type.idx,
                    field.as_ref().map(|field| field.storage.clone().into()),
                )
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
            other.kind == SymbolKind::FieldDef
                && other.region == region
                && symbol.idx.is_defined_by(&other.idx)
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
    service: &'db dyn salsa::Database,
    document: Document,
    struct_ref: &Immediate,
    field_ref: &Immediate,
) -> Option<(Idx<'db>, Option<OperandType<'db>>)> {
    let symbol_table = SymbolTable::of(service, document);
    let struct_def_symbol = symbol_table.find_def(SymbolKey::new(struct_ref.syntax()))?;
    let ty = resolve_field_type(
        service,
        document,
        SymbolKey::new(field_ref.syntax()),
        struct_def_symbol.key,
    )
    .map(|ty| ty.into());
    Some((struct_def_symbol.idx, ty))
}
