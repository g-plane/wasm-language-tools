use crate::{config::LintLevel, helpers};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, DiagnosticTag, Union2};
use rowan::ast::{AstNode, support};
use wat_syntax::{SyntaxKind, SyntaxNode, ast::Cat};

const DIAGNOSTIC_CODE: &str = "needless-try-table";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
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
    if let Some(keyword) = support::token(node, SyntaxKind::KEYWORD)
        && !node.children().any(|child| Cat::can_cast(child.kind()))
    {
        diagnostics.push(Diagnostic {
            range: helpers::rowan_range_to_lsp_range(line_index, keyword.text_range()),
            severity: Some(severity),
            source: Some("wat".into()),
            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
            message: "`try_table` block without catch clauses is unnecessary".into(),
            tags: Some(vec![DiagnosticTag::Unnecessary]),
            ..Default::default()
        });
    }
}
