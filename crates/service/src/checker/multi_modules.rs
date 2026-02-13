use super::Diagnostic;
use crate::config::LintLevel;
use lspt::DiagnosticSeverity;
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "multiple-modules";

pub fn check(diagnostics: &mut Vec<Diagnostic>, lint_level: LintLevel, root: &SyntaxNode) {
    let severity = match lint_level {
        LintLevel::Allow => return,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };
    diagnostics.extend(
        root.children_by_kind(|kind| kind == SyntaxKind::MODULE)
            .skip(1)
            .map(|module| Diagnostic {
                range: module.text_range(),
                severity,
                code: DIAGNOSTIC_CODE.into(),
                message: "only one module is allowed".into(),
                ..Default::default()
            }),
    );
}
