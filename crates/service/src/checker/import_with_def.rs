use super::{Diagnostic, RelatedInformation};
use crate::{binder::SymbolKey, document::Document, imex};
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "import-with-def";

pub fn check(db: &dyn salsa::Database, document: Document, node: &SyntaxNode) -> Option<Diagnostic> {
    let imports = imex::get_imports(db, document);
    if !imports.contains(&SymbolKey::new(node)) {
        return None;
    }
    let first = node.first_child_by_kind(|kind| {
        !matches!(
            kind,
            SyntaxKind::EXPORT
                | SyntaxKind::IMPORT
                | SyntaxKind::TYPE_USE
                | SyntaxKind::GLOBAL_TYPE
                | SyntaxKind::MEM_TYPE
                | SyntaxKind::TABLE_TYPE
        )
    })?;
    let last = node.last_child()?;
    Some(Diagnostic {
        range: first.text_range().cover(last.text_range()),
        code: DIAGNOSTIC_CODE.into(),
        message: "imported item can't contain definition".into(),
        related_information: node
            .first_child_by_kind(|kind| kind == SyntaxKind::IMPORT)
            .map(|import| {
                vec![RelatedInformation {
                    range: import.text_range(),
                    message: "import declared here".into(),
                }]
            }),
        ..Default::default()
    })
}
