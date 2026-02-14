use super::{Diagnostic, RelatedInformation};
use crate::{document::Document, imex};
use wat_syntax::{AmberNode, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "import-with-def";

pub fn check(db: &dyn salsa::Database, document: Document, node: AmberNode) -> Option<Diagnostic> {
    let imports = imex::get_imports(db, document);
    if !imports.contains(&node.to_ptr().into()) {
        return None;
    }
    let first = node.children().find(|child| {
        !matches!(
            child.kind(),
            SyntaxKind::EXPORT
                | SyntaxKind::IMPORT
                | SyntaxKind::TYPE_USE
                | SyntaxKind::GLOBAL_TYPE
                | SyntaxKind::MEM_TYPE
                | SyntaxKind::TABLE_TYPE
        )
    })?;
    let last = node.children().next_back()?;
    Some(Diagnostic {
        range: first.text_range().cover(last.text_range()),
        code: DIAGNOSTIC_CODE.into(),
        message: "imported item can't contain definition".into(),
        related_information: node
            .children()
            .find(|child| child.kind() == SyntaxKind::IMPORT)
            .map(|import| {
                vec![RelatedInformation {
                    range: import.text_range(),
                    message: "import declared here".into(),
                }]
            }),
        ..Default::default()
    })
}
