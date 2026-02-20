use super::Diagnostic;
use crate::{document::Document, imex};
use wat_syntax::{
    AmberNode, SyntaxKind,
    ast::{AstNode, ModuleField},
};

const DIAGNOSTIC_CODE: &str = "import-occurrence";

pub fn check(diagnostics: &mut Vec<Diagnostic>, db: &dyn salsa::Database, document: Document, node: AmberNode) {
    let imports = imex::get_imports(db, document);
    diagnostics.extend(
        node.children_by_kind(ModuleField::can_cast)
            .scan(false, |has_non_import, child| match child.kind() {
                SyntaxKind::MODULE_FIELD_FUNC
                | SyntaxKind::MODULE_FIELD_TABLE
                | SyntaxKind::MODULE_FIELD_MEMORY
                | SyntaxKind::MODULE_FIELD_GLOBAL
                | SyntaxKind::MODULE_FIELD_TAG
                    if !imports.contains(&child.to_ptr().into()) =>
                {
                    *has_non_import = true;
                    Some(None)
                }
                SyntaxKind::MODULE_FIELD_IMPORT if *has_non_import => Some(Some(Diagnostic {
                    range: child.text_range(),
                    code: DIAGNOSTIC_CODE.into(),
                    message: "import must occur before all non-import definitions".into(),
                    ..Default::default()
                })),
                _ => Some(None),
            })
            .flatten(),
    );
}
