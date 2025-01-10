use crate::helpers;
use line_index::LineIndex;
use lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString};
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "multiple-modules";

pub fn check(diags: &mut Vec<Diagnostic>, line_index: &LineIndex, root: &SyntaxNode) {
    diags.extend(
        root.children()
            .filter(|child| child.kind() == SyntaxKind::MODULE)
            .skip(1)
            .map(|module| Diagnostic {
                range: helpers::rowan_range_to_lsp_range(line_index, module.text_range()),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("wat".into()),
                code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
                message: "only one module is allowed".into(),
                ..Default::default()
            }),
    );
}
