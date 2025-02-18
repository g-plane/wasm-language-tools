use crate::{helpers, LintLevel};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use rowan::ast::support;
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "implicit-module";

pub fn check(
    diags: &mut Vec<Diagnostic>,
    lint_level: LintLevel,
    line_index: &LineIndex,
    node: &SyntaxNode,
) {
    let severity = match lint_level {
        LintLevel::Allow => return,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };
    if support::token(node, SyntaxKind::L_PAREN).is_none() {
        diags.push(Diagnostic {
            range: helpers::rowan_range_to_lsp_range(line_index, node.text_range()),
            severity: Some(severity),
            source: Some("wat".into()),
            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
            message: "top-level module fields should be wrapped in a module".into(),
            ..Default::default()
        });
    }
}
