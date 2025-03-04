use crate::{
    binder::{SymbolKey, SymbolTable},
    helpers,
    types_analyzer::TypesAnalyzerCtx,
    uri::InternUri,
    LanguageService,
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use rowan::ast::{support, AstNode};
use wat_syntax::{ast::Index, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "start";

pub fn check(
    diags: &mut Vec<Diagnostic>,
    service: &LanguageService,
    uri: InternUri,
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
        .and_then(|func| service.get_func_sig(uri, func.key, func.green.clone()))
        .is_some_and(|sig| !sig.params.is_empty() || !sig.results.is_empty())
    {
        diags.push(Diagnostic {
            range: helpers::rowan_range_to_lsp_range(line_index, index.text_range()),
            severity: Some(DiagnosticSeverity::Error),
            source: Some("wat".into()),
            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
            message: "start function must be type of [] -> []".into(),
            ..Default::default()
        });
    }
}
