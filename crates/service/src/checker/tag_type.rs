use crate::{LanguageService, document::Document, helpers, types_analyzer};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use wat_syntax::{SyntaxKind, SyntaxNode, SyntaxNodePtr};

const DIAGNOSTIC_CODE: &str = "tag-type";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    service: &LanguageService,
    document: Document,
    line_index: &LineIndex,
    node: &SyntaxNode,
) {
    let Some(type_use) = node.first_child_by_kind(&|kind| kind == SyntaxKind::TYPE_USE) else {
        return;
    };
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
