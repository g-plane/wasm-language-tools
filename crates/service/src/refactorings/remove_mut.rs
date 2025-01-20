use crate::{
    helpers,
    uri::{InternUri, UrisCtx},
    LanguageService,
};
use line_index::LineIndex;
use lsp_types::{
    CodeAction, CodeActionContext, CodeActionKind, NumberOrString, TextEdit, WorkspaceEdit,
};
use rowan::ast::support;
use std::collections::HashMap;
use wat_syntax::{SyntaxKind, SyntaxNode};

pub fn act(
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    node: &SyntaxNode,
    context: &CodeActionContext,
) -> Option<CodeAction> {
    let mut_token =
        support::token(node, SyntaxKind::KEYWORD).filter(|keyword| keyword.text() == "mut")?;
    let token_lsp_range = helpers::rowan_range_to_lsp_range(line_index, mut_token.text_range());
    let diagnostic = context
        .diagnostics
        .iter()
        .find(|diagnostic| match &diagnostic.code {
            Some(NumberOrString::String(code)) => {
                code == "needless-mut" && diagnostic.range == token_lsp_range
            }
            _ => false,
        })?;

    let mut text_edits = Vec::with_capacity(4);
    if let Some(l_paren) = support::token(node, SyntaxKind::L_PAREN) {
        text_edits.push(TextEdit {
            range: helpers::rowan_range_to_lsp_range(line_index, l_paren.text_range()),
            new_text: "".into(),
        });
    }
    text_edits.push(TextEdit {
        range: token_lsp_range,
        new_text: "".into(),
    });
    if let Some(whitespace) = mut_token
        .next_token()
        .filter(|token| token.kind() == SyntaxKind::WHITESPACE)
    {
        text_edits.push(TextEdit {
            range: helpers::rowan_range_to_lsp_range(line_index, whitespace.text_range()),
            new_text: "".into(),
        });
    }
    if let Some(r_paren) = support::token(node, SyntaxKind::R_PAREN) {
        text_edits.push(TextEdit {
            range: helpers::rowan_range_to_lsp_range(line_index, r_paren.text_range()),
            new_text: "".into(),
        });
    }

    #[expect(clippy::mutable_key_type)]
    if text_edits.is_empty() {
        None
    } else {
        let mut changes = HashMap::with_capacity(1);
        changes.insert(service.lookup_uri(uri), text_edits);
        Some(CodeAction {
            title: "Remove `mut`".into(),
            kind: Some(CodeActionKind::QUICKFIX),
            edit: Some(WorkspaceEdit {
                changes: Some(changes),
                ..Default::default()
            }),
            is_preferred: Some(true),
            diagnostics: Some(vec![diagnostic.clone()]),
            ..Default::default()
        })
    }
}
