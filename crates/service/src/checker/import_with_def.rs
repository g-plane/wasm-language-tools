use crate::{helpers, uri::InternUri};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, Union2};
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "import-with-def";

pub fn check(
    db: &dyn salsa::Database,
    uri: InternUri,
    line_index: &LineIndex,
    node: &SyntaxNode,
) -> Option<Diagnostic> {
    let import = node.first_child_by_kind(&|kind| kind == SyntaxKind::IMPORT)?;
    let first = node.first_child_by_kind(&|kind| {
        !matches!(
            kind,
            SyntaxKind::EXPORT
                | SyntaxKind::IMPORT
                | SyntaxKind::TYPE_USE
                | SyntaxKind::GLOBAL_TYPE
                | SyntaxKind::MEMORY_TYPE
                | SyntaxKind::TABLE_TYPE
        )
    })?;
    let last = node.last_child()?;
    Some(Diagnostic {
        range: helpers::rowan_range_to_lsp_range(line_index, first.text_range().cover(last.text_range())),
        severity: Some(DiagnosticSeverity::Error),
        source: Some("wat".into()),
        code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
        message: "imported item can't contain definition".into(),
        related_information: Some(vec![DiagnosticRelatedInformation {
            location: Location {
                uri: uri.raw(db),
                range: helpers::rowan_range_to_lsp_range(line_index, import.text_range()),
            },
            message: "import declared here".into(),
        }]),
        ..Default::default()
    })
}
