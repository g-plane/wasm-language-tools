use super::Diagnostic;
use crate::config::LintLevel;
use lspt::{DiagnosticSeverity, DiagnosticTag};
use rowan::ast::{AstNode, support};
use wat_syntax::{SyntaxKind, SyntaxNode, ast::Cat};

const DIAGNOSTIC_CODE: &str = "needless-try-table";

pub fn check(lint_level: LintLevel, node: &SyntaxNode) -> Option<Diagnostic> {
    let severity = match lint_level {
        LintLevel::Allow => return None,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };
    if let Some(keyword) = support::token(node, SyntaxKind::KEYWORD)
        && !node.children().any(|child| Cat::can_cast(child.kind()))
    {
        Some(Diagnostic {
            range: keyword.text_range(),
            severity,
            code: DIAGNOSTIC_CODE.into(),
            message: "`try_table` block without catch clauses is unnecessary".into(),
            tags: Some(vec![DiagnosticTag::Unnecessary]),
            ..Default::default()
        })
    } else {
        None
    }
}
