use super::Diagnostic;
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "import-occurrence";

pub fn check(node: &SyntaxNode) -> Option<Diagnostic> {
    if node.prev_sibling().is_some_and(|prev| {
        matches!(
            prev.kind(),
            SyntaxKind::MODULE_FIELD_FUNC
                | SyntaxKind::MODULE_FIELD_TABLE
                | SyntaxKind::MODULE_FIELD_MEMORY
                | SyntaxKind::MODULE_FIELD_GLOBAL
        ) && !prev.children().any(|child| child.kind() == SyntaxKind::IMPORT)
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
