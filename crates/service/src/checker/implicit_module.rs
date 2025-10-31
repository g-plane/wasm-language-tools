use crate::{LintLevel, helpers};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use rowan::ast::support;
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "implicit-module";

pub fn check(
    lint_level: LintLevel,
    line_index: &LineIndex,
    node: &SyntaxNode,
) -> Option<Diagnostic> {
    let severity = match lint_level {
        LintLevel::Allow => return None,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };
    if support::token(node, SyntaxKind::L_PAREN).is_none() {
        Some(Diagnostic {
            range: helpers::rowan_range_to_lsp_range(line_index, node.text_range()),
            severity: Some(severity),
            source: Some("wat".into()),
            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
            message: "top-level module fields should be wrapped in a module".into(),
            ..Default::default()
        })
    } else {
        None
    }
}
