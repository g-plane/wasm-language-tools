use crate::{helpers::LineIndexExt, uri::InternUri};
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionContext, CodeActionKind, TextEdit, Union2, WorkspaceEdit};
use rowan::ast::support;
use rustc_hash::FxBuildHasher;
use std::collections::HashMap;
use wat_syntax::{SyntaxKind, SyntaxNode};

pub fn act(
    db: &dyn salsa::Database,
    uri: InternUri,
    line_index: &LineIndex,
    node: &SyntaxNode,
    context: &CodeActionContext,
) -> Option<CodeAction> {
    let mut_token = support::token(node, SyntaxKind::KEYWORD).filter(|keyword| keyword.text() == "mut")?;
    let token_lsp_range = line_index.convert(mut_token.text_range());
    let diagnostic = context.diagnostics.iter().find(|diagnostic| match &diagnostic.code {
        Some(Union2::B(code)) => code == "needless-mut" && diagnostic.range == token_lsp_range,
        _ => false,
    })?;

    let mut text_edits = Vec::with_capacity(4);
    if let Some(l_paren) = support::token(node, SyntaxKind::L_PAREN) {
        text_edits.push(TextEdit {
            range: line_index.convert(l_paren.text_range()),
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
            range: line_index.convert(whitespace.text_range()),
            new_text: "".into(),
        });
    }
    if let Some(r_paren) = support::token(node, SyntaxKind::R_PAREN) {
        text_edits.push(TextEdit {
            range: line_index.convert(r_paren.text_range()),
            new_text: "".into(),
        });
    }

    if text_edits.is_empty() {
        None
    } else {
        let mut changes = HashMap::with_capacity_and_hasher(1, FxBuildHasher);
        changes.insert(uri.raw(db), text_edits);
        Some(CodeAction {
            title: "Remove `mut`".into(),
            kind: Some(CodeActionKind::QuickFix),
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
