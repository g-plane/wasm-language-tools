use crate::{config::LintLevel, helpers};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "multiple-memories";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    lint_level: LintLevel,
    line_index: &LineIndex,
    root: &SyntaxNode,
) {
    let severity = match lint_level {
        LintLevel::Allow => return,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };
    diagnostics.extend(
        root.children()
            .filter(|child| child.kind() == SyntaxKind::MODULE_FIELD_MEMORY)
            .skip(1)
            .map(|module| Diagnostic {
                range: helpers::rowan_range_to_lsp_range(line_index, module.text_range()),
                severity: Some(severity),
                source: Some("wat".into()),
                code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                message: "only one memory is allowed".into(),
                ..Default::default()
            }),
    );
}
