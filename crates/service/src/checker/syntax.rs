use crate::{helpers, syntax_tree::SyntaxTreeCtx, uri::InternUri, LanguageService};
use line_index::LineIndex;
use lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString};
use std::rc::Rc;
use wat_syntax::{SyntaxElement, SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "syntax";

pub fn check(
    service: &LanguageService,
    diags: &mut Vec<Diagnostic>,
    uri: InternUri,
    line_index: &LineIndex,
    root: &SyntaxNode,
) {
    let mut errors = Rc::unwrap_or_clone(service.parse(uri).1);
    diags.append(&mut errors);
    diags.extend(
        root.children_with_tokens()
            .filter_map(|element| match element {
                SyntaxElement::Token(token) if token.kind() == SyntaxKind::ERROR => Some(token),
                _ => None,
            })
            .map(|token| Diagnostic {
                range: helpers::rowan_range_to_lsp_range(line_index, token.text_range()),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("wat".into()),
                code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
                message: "syntax error: unexpected token".into(),
                ..Default::default()
            }),
    );
}
