use crate::{
    helpers,
    uri::{InternUri, UrisCtx},
    LanguageService,
};
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionContext, CodeActionKind, TextEdit, Union2, WorkspaceEdit};
use rustc_hash::FxBuildHasher;
use std::collections::HashMap;
use wat_syntax::{SyntaxElement, SyntaxNode};

pub fn act(
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    node: &SyntaxNode,
    context: &CodeActionContext,
) -> Option<CodeAction> {
    let text_edits = node
        .children_with_tokens()
        .filter_map(|node_or_token| match node_or_token {
            SyntaxElement::Token(token) if token.kind().is_trivia() => Some(token.text_range()),
            _ => None,
        })
        .map(|range| TextEdit {
            range: helpers::rowan_range_to_lsp_range(line_index, range),
            new_text: String::new(),
        })
        .collect::<Vec<_>>();
    if text_edits.is_empty() {
        None
    } else {
        let mut changes = HashMap::with_capacity_and_hasher(1, FxBuildHasher);
        changes.insert(service.lookup_uri(uri), text_edits);
        Some(CodeAction {
            title: "Fix invalid memory argument".into(),
            kind: Some(CodeActionKind::QuickFix),
            edit: Some(WorkspaceEdit {
                changes: Some(changes),
                ..Default::default()
            }),
            is_preferred: Some(true),
            diagnostics: Some(
                context
                    .diagnostics
                    .iter()
                    .filter(|diagnostic| {
                        if let Some(Union2::B(s)) = &diagnostic.code {
                            s.starts_with("syntax")
                                && diagnostic.message.contains("memory argument")
                        } else {
                            false
                        }
                    })
                    .cloned()
                    .collect(),
            ),
            ..Default::default()
        })
    }
}
