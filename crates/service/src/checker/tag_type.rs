use super::Diagnostic;
use crate::{
    binder::{SymbolKey, SymbolTable},
    document::Document,
    types_analyzer::{self, CompositeType},
};
use wat_syntax::{SyntaxKind, SyntaxNode, SyntaxNodePtr};

const DIAGNOSTIC_CODE: &str = "tag-type";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) {
    let Some(type_use) = node.first_child_by_kind(&|kind| kind == SyntaxKind::TYPE_USE) else {
        return;
    };
    if let Some(index) = type_use.first_child_by_kind(&|kind| kind == SyntaxKind::INDEX) {
        let def_types = types_analyzer::get_def_types(db, document);
        if symbol_table
            .resolved
            .get(&SymbolKey::new(&index))
            .and_then(|def_key| def_types.get(def_key))
            .is_some_and(|def_type| !matches!(def_type.comp, CompositeType::Func(..)))
        {
            diagnostics.push(Diagnostic {
                range: index.text_range(),
                code: DIAGNOSTIC_CODE.into(),
                message: "tag type must be function type".into(),
                ..Default::default()
            });
        }
    }
    let sig = types_analyzer::get_type_use_sig(db, document, SyntaxNodePtr::new(&type_use), &type_use.green());
    if !sig.results.is_empty() {
        diagnostics.push(Diagnostic {
            range: type_use.text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message: "tag type's result type must be empty".into(),
            ..Default::default()
        });
    }
}
