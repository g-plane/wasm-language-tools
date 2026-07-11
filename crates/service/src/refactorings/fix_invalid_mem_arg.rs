use crate::{helpers::LineIndexExt, uri::InternUri};
use line_index::LineIndex;
use lspt::{
    CodeAction, CodeActionContext, CodeActionKind, NumberOrString, StringOrMarkupContent, TextEdit, WorkspaceEdit,
};
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
    let text_edits = node
        .tokens_by_kind(SyntaxKind::is_trivia)
        .filter_map(|token| line_index.convert(token.text_range()))
        .map(|range| TextEdit {
            range,
            new_text: String::new(),
        })
        .collect::<Vec<_>>();
    if text_edits.is_empty() {
        None
    } else {
        let mut changes = HashMap::with_capacity_and_hasher(1, FxBuildHasher);
        changes.insert(uri.raw(db), text_edits);
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
                        if let Some(NumberOrString::String(code)) = &diagnostic.code
                            && let StringOrMarkupContent::String(message) = &diagnostic.message
                        {
                            code.starts_with("syntax") && message.contains("memory argument")
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
