use crate::{
    LanguageService,
    binder::{SymbolKey, SymbolTable},
    document::Document,
    helpers, types_analyzer,
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use rowan::ast::{AstNode, support};
use wat_syntax::{SyntaxNode, ast::Index};

const DIAGNOSTIC_CODE: &str = "start";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    service: &LanguageService,
    document: Document,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) {
    let Some(index) = support::child::<Index>(node) else {
        return;
    };
    let index = index.syntax();
    if symbol_table
        .find_def(SymbolKey::new(index))
        .map(|func| types_analyzer::get_func_sig(service, document, func.key, &func.green))
        .is_some_and(|sig| !sig.params.is_empty() || !sig.results.is_empty())
    {
        diagnostics.push(Diagnostic {
            range: helpers::rowan_range_to_lsp_range(line_index, index.text_range()),
            severity: Some(DiagnosticSeverity::Error),
            source: Some("wat".into()),
            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
            message: "start function must be type of [] -> []".into(),
            ..Default::default()
        });
    }
}
