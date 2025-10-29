use crate::{
    LanguageService,
    binder::{SymbolKey, SymbolTable},
    document::Document,
    helpers,
    types_analyzer::{self, CompositeType},
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use wat_syntax::{SyntaxKind, SyntaxNode, SyntaxNodePtr};

const DIAGNOSTIC_CODE: &str = "tag-type";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    service: &LanguageService,
    document: Document,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) {
    let Some(type_use) = node.first_child_by_kind(&|kind| kind == SyntaxKind::TYPE_USE) else {
        return;
    };
    if let Some(index) = type_use.first_child_by_kind(&|kind| kind == SyntaxKind::INDEX) {
        let def_types = types_analyzer::get_def_types(service, document);
        if symbol_table
            .resolved
            .get(&SymbolKey::new(&index))
            .and_then(|def_key| def_types.get(def_key))
            .is_some_and(|def_type| !matches!(def_type.comp, CompositeType::Func(..)))
        {
            diagnostics.push(Diagnostic {
                range: helpers::rowan_range_to_lsp_range(line_index, index.text_range()),
                severity: Some(DiagnosticSeverity::Error),
                source: Some("wat".into()),
                code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                message: "tag type must be function type".into(),
                ..Default::default()
            });
        }
    }
    let sig = types_analyzer::get_type_use_sig(
        service,
        document,
        SyntaxNodePtr::new(&type_use),
        &type_use.green(),
    );
    if !sig.results.is_empty() {
        diagnostics.push(Diagnostic {
            range: helpers::rowan_range_to_lsp_range(line_index, type_use.text_range()),
            severity: Some(DiagnosticSeverity::Error),
            source: Some("wat".into()),
            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
            message: "tag type's result type must be empty".into(),
            ..Default::default()
        });
    }
}
