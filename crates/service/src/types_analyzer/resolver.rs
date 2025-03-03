use super::{signature::get_block_sig, types::OperandType, TypesAnalyzerCtx};
use crate::{
    binder::{SymbolKey, SymbolTable, SymbolTablesCtx},
    data_set::INSTR_SIG,
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
