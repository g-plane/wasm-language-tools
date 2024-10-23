use crate::helpers;
use line_index::LineIndex;
use lsp_types::{Diagnostic, DiagnosticSeverity};
use wat_syntax::{SyntaxKind, SyntaxNode};

pub fn check(diags: &mut Vec<Diagnostic>, line_index: &LineIndex, root: &SyntaxNode) {
    diags.extend(
        root.children()
            .filter(|child| child.kind() == SyntaxKind::MODULE)
            .skip(1)
            .map(|module| Diagnostic {
                range: helpers::rowan_range_to_lsp_range(line_index, module.text_range()),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("wat".into()),
                message: "only one module is allowed in one file".into(),
                ..Default::default()
            }),
    );
}
