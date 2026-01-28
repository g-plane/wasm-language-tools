use super::Diagnostic;
use crate::{binder::SymbolKey, document::Document, imex};
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "import-occurrence";

pub fn check(db: &dyn salsa::Database, document: Document, node: &SyntaxNode) -> Option<Diagnostic> {
    let imports = imex::get_imports(db, document);
    if node.prev_sibling().is_some_and(|prev| {
        matches!(
            prev.kind(),
            SyntaxKind::MODULE_FIELD_FUNC
                | SyntaxKind::MODULE_FIELD_TABLE
                | SyntaxKind::MODULE_FIELD_MEMORY
                | SyntaxKind::MODULE_FIELD_GLOBAL
                | SyntaxKind::MODULE_FIELD_TAG
        ) && !imports.contains(&SymbolKey::new(&prev))
    }) {
        Some(Diagnostic {
            range: node.text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message: "import must occur before all non-import definitions".into(),
            ..Default::default()
        })
    } else {
        None
    }
}
