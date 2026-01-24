use super::Diagnostic;
use crate::{
    binder::{SymbolKey, SymbolTable},
    document::Document,
    types_analyzer::{self, CompositeType},
};
use rowan::ast::{AstNode, support};
use wat_syntax::{SyntaxNode, ast::TypeUse};

const DIAGNOSTIC_CODE: &str = "block-type";

pub fn check(
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) -> Option<Diagnostic> {
    let index = support::child::<TypeUse>(node).and_then(|type_use| type_use.index())?;
    let index = index.syntax();
    let def_types = types_analyzer::get_def_types(db, document);
    if symbol_table
        .resolved
        .get(&SymbolKey::new(index))
        .and_then(|key| def_types.get(key))
        .is_some_and(|def_type| !matches!(def_type.comp, CompositeType::Func(..)))
    {
        Some(Diagnostic {
            range: index.text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message: "block type must be function type".into(),
            ..Default::default()
        })
    } else {
        None
    }
}
