use crate::{helpers::LineIndexExt, uri::InternUri};
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use rustc_hash::FxBuildHasher;
use std::collections::HashMap;
use wat_syntax::{NodeOrToken, SyntaxKind, SyntaxNode, TextRange};

pub fn act(db: &dyn salsa::Database, uri: InternUri, line_index: &LineIndex, node: &SyntaxNode) -> Option<CodeAction> {
    let new_text = format!(" {node}");
    let mut text_edits = node
        .parent()?
        .children_by_kind(SyntaxKind::IMPORT_ITEM)
        .filter_map(|import_item| {
            import_item
                .children_by_kind(SyntaxKind::NAME)
                .next()
                .map(|name| TextEdit {
                    range: line_index.convert(TextRange::empty(name.text_range().end())),
                    new_text: new_text.clone(),
                })
        })
        .collect::<Vec<_>>();
    if text_edits.is_empty() {
        None
    } else {
        let range = if let Some(NodeOrToken::Token(token)) = node.prev_sibling_or_token()
            && token.kind() == SyntaxKind::WHITESPACE
        {
            TextRange::cover(token.text_range(), node.text_range())
        } else {
            node.text_range()
        };
        text_edits.push(TextEdit {
            range: line_index.convert(range),
            new_text: "".into(),
        });
        let mut changes = HashMap::with_capacity_and_hasher(1, FxBuildHasher);
        changes.insert(uri.raw(db), text_edits);
        Some(CodeAction {
            title: "Inline extern type".into(),
            kind: Some(CodeActionKind::RefactorInline),
            edit: Some(WorkspaceEdit {
                changes: Some(changes),
                ..Default::default()
            }),
            ..Default::default()
        })
    }
}
