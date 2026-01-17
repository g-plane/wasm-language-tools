use crate::helpers;
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "multiple-starts";

pub fn check(diagnostics: &mut Vec<Diagnostic>, line_index: &LineIndex, module: &SyntaxNode) {
    diagnostics.extend(
        module
            .children()
            .filter(|child| child.kind() == SyntaxKind::MODULE_FIELD_START)
            .skip(1)
            .map(|start| Diagnostic {
                range: helpers::rowan_range_to_lsp_range(line_index, start.text_range()),
                severity: Some(DiagnosticSeverity::Error),
                source: Some("wat".into()),
                code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                message: "only one start section is allowed".into(),
                ..Default::default()
            }),
    );
}
