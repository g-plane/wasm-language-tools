use crate::helpers;
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "import-occurrence";

pub fn check(diagnostics: &mut Vec<Diagnostic>, line_index: &LineIndex, node: &SyntaxNode) {
    if node.prev_sibling().is_some_and(|prev| {
        matches!(
            prev.kind(),
            SyntaxKind::MODULE_FIELD_FUNC
                | SyntaxKind::MODULE_FIELD_TABLE
                | SyntaxKind::MODULE_FIELD_MEMORY
                | SyntaxKind::MODULE_FIELD_GLOBAL
        ) && !prev
            .children()
            .any(|child| child.kind() == SyntaxKind::IMPORT)
    }) {
        diagnostics.push(Diagnostic {
            range: helpers::rowan_range_to_lsp_range(line_index, node.text_range()),
            severity: Some(DiagnosticSeverity::Error),
            source: Some("wat".into()),
            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
            message: "import must occur before all non-import definitions".into(),
            ..Default::default()
        });
    }
}
