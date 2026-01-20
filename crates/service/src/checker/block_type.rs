use crate::{
    binder::{SymbolKey, SymbolTable},
    document::Document,
    helpers,
    types_analyzer::{self, CompositeType},
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use rowan::ast::{AstNode, support};
use wat_syntax::{SyntaxNode, ast::TypeUse};

const DIAGNOSTIC_CODE: &str = "block-type";

pub fn check(
    db: &dyn salsa::Database,
    document: Document,
    line_index: &LineIndex,
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
            range: helpers::rowan_range_to_lsp_range(line_index, index.text_range()),
            severity: Some(DiagnosticSeverity::Error),
            source: Some("wat".into()),
            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
            message: "block type must be function type".into(),
            ..Default::default()
        })
    } else {
        None
    }
}
