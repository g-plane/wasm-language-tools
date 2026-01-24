use super::{Diagnostic, RelatedInformation};
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "import-with-def";

pub fn check(node: &SyntaxNode) -> Option<Diagnostic> {
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
        range: first.text_range().cover(last.text_range()),
        code: DIAGNOSTIC_CODE.into(),
        message: "imported item can't contain definition".into(),
        related_information: Some(vec![RelatedInformation {
            range: import.text_range(),
            message: "import declared here".into(),
        }]),
        ..Default::default()
    })
}
