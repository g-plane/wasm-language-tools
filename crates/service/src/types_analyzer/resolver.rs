use super::{
    def_type::{CompositeType, DefType},
    signature::get_block_sig,
    types::{FieldType, Fields, OperandType},
    TypesAnalyzerCtx,
};
use crate::{
    binder::{SymbolKey, SymbolKind, SymbolTable, SymbolTablesCtx},
    data_set::INSTR_SIG,
    idx::Idx,
    syntax_tree::SyntaxTreeCtx,
    uri::InternUri,
    LanguageService,
};
use rowan::ast::{support, AstNode};
use wat_syntax::{ast::Immediate, SyntaxKind, SyntaxNode};

pub(crate) fn resolve_param_types(
    service: &LanguageService,
    uri: InternUri,
    instr: &SyntaxNode,
) -> Option<Vec<OperandType>> {
    debug_assert!(instr.kind() == SyntaxKind::PLAIN_INSTR);
    let instr_name = support::token(instr, SyntaxKind::INSTR_NAME)?;
    let instr_name = instr_name.text();
    if matches!(instr_name, "call" | "return_call") {
        let symbol_table = service.symbol_table(uri);
        let idx = instr
            .children()
            .find(|child| child.kind() == SyntaxKind::IMMEDIATE)?;
        let func = symbol_table.find_def(SymbolKey::new(&idx))?;
        service
            .get_func_sig(uri, func.key, func.green.clone())
            .map(|sig| {
                sig.params
                    .iter()
                    .map(|(ty, ..)| OperandType::Val(*ty))
                    .collect()
            })
    } else {
        INSTR_SIG.get(instr_name).map(|sig| sig.params.clone())
    }
}

pub(crate) fn resolve_br_types(
    service: &LanguageService,
    uri: InternUri,
    symbol_table: &SymbolTable,
    immediate: &Immediate,
) -> Vec<OperandType> {
    let key = SymbolKey::new(immediate.syntax());
    symbol_table
        .blocks
        .iter()
        .find(|block| block.ref_key == key)
        .and_then(|block| {
            get_block_sig(
                service,
                uri,
                &block
                    .def_key
                    .to_node(&SyntaxNode::new_root(service.root(uri))),
            )
        })
        .map(|sig| sig.results.into_iter().map(OperandType::Val).collect())
        .unwrap_or_default()
}

pub(crate) fn resolve_array_type_with_idx(
    symbol_table: &SymbolTable,
    def_types: &[DefType],
    immediate: &Immediate,
) -> Option<(Idx, Option<OperandType>)> {
    symbol_table
        .find_def(SymbolKey::new(immediate.syntax()))
        .and_then(|symbol| def_types.iter().find(|def_type| def_type.key == symbol.key))
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

pub(super) fn resolve_field_type(
    db: &dyn TypesAnalyzerCtx,
    uri: InternUri,
    key: SymbolKey,
) -> Option<FieldType> {
    let symbol_table = db.symbol_table(uri);
    let def_types = db.def_types(uri);
    let symbol = symbol_table
        .symbols
        .iter()
        .find(|symbol| symbol.key == key)?;
    let field_def_symbol = match symbol.kind {
        SymbolKind::FieldDef => symbol,
        SymbolKind::FieldRef => symbol_table.symbols.iter().find(|other| {
            other.kind == SymbolKind::FieldDef
                && other.region == symbol.region
                && symbol.idx.is_defined_by(&other.idx)
        })?,
        _ => return None,
    };
    let idx = field_def_symbol.idx.num?;
    if let Some(DefType {
        comp: CompositeType::Struct(Fields(fields)),
        ..
    }) = def_types
        .iter()
        .find(|def_type| def_type.key == field_def_symbol.region)
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
