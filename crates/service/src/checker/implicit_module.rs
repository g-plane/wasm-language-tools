use crate::{helpers, InternUri, LanguageService, LintLevel};
use line_index::LineIndex;
use lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString};
use rowan::ast::support;
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "implicit-module";

pub fn check(
    service: &LanguageService,
    diags: &mut Vec<Diagnostic>,
    uri: InternUri,
    line_index: &LineIndex,
    node: &SyntaxNode,
) {
    let severity = match service.get_config(uri).lint.implicit_module {
        LintLevel::Allow => return,
        LintLevel::Warn => DiagnosticSeverity::WARNING,
        LintLevel::Deny => DiagnosticSeverity::ERROR,
    };
    if support::token(node, SyntaxKind::L_PAREN).is_none() {
        diags.push(Diagnostic {
            range: helpers::rowan_range_to_lsp_range(line_index, node.text_range()),
            severity: Some(severity),
            source: Some("wat".into()),
            code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
            message: "top-level module fields should be wrapped in a module".into(),
            ..Default::default()
        });
    }
}
