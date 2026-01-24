use super::Diagnostic;
use crate::LintLevel;
use lspt::DiagnosticSeverity;
use rowan::ast::support;
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "implicit-module";

pub fn check(lint_level: LintLevel, node: &SyntaxNode) -> Option<Diagnostic> {
    let severity = match lint_level {
        LintLevel::Allow => return None,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };
    if support::token(node, SyntaxKind::L_PAREN).is_none() {
        Some(Diagnostic {
            range: node.text_range(),
            severity,
            code: DIAGNOSTIC_CODE.into(),
            message: "top-level module fields should be wrapped in a module".into(),
            ..Default::default()
        })
    } else {
        None
    }
}
