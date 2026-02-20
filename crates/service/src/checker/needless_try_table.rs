use super::Diagnostic;
use crate::config::LintLevel;
use lspt::{DiagnosticSeverity, DiagnosticTag};
use wat_syntax::{
    AmberNode, SyntaxKind,
    ast::{AstNode, Cat},
};

const DIAGNOSTIC_CODE: &str = "needless-try-table";

pub fn check(lint_level: LintLevel, node: AmberNode) -> Option<Diagnostic> {
    let severity = match lint_level {
        LintLevel::Allow => return None,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };
    if let Some(keyword) = node.tokens_by_kind(SyntaxKind::KEYWORD).next()
        && node.children_by_kind(Cat::can_cast).next().is_none()
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
