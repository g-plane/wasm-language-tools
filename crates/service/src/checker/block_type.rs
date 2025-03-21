use crate::{
    binder::{SymbolKey, SymbolTable},
    helpers,
    types_analyzer::{CompositeType, TypesAnalyzerCtx},
    uri::InternUri,
    LanguageService,
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use rowan::ast::{support, AstNode};
use wat_syntax::{ast::TypeUse, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "block-type";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) {
    let Some(index) = support::child::<TypeUse>(node).and_then(|type_use| type_use.index()) else {
        return;
    };
    let index = index.syntax();
    let def_types = service.def_types(uri);
    if symbol_table
        .find_def(SymbolKey::new(index))
        .and_then(|symbol| def_types.iter().find(|def_type| def_type.key == symbol.key))
        .is_some_and(|def_type| !matches!(def_type.comp, CompositeType::Func(..)))
    {
        diagnostics.push(Diagnostic {
            range: helpers::rowan_range_to_lsp_range(line_index, index.text_range()),
            severity: Some(DiagnosticSeverity::Error),
            source: Some("wat".into()),
            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
            message: "block type must be function type".into(),
            ..Default::default()
        });
    }
}
