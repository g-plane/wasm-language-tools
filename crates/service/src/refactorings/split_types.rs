use crate::{helpers, uri::InternUri};
use itertools::Itertools;
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use rustc_hash::FxBuildHasher;
use std::collections::HashMap;
use wat_syntax::{SyntaxKind, SyntaxNode};

pub fn act(
    db: &dyn salsa::Database,
    uri: InternUri,
    line_index: &LineIndex,
    node: &SyntaxNode,
    kind: SyntaxKind,
) -> Option<CodeAction> {
    let types = node.children().collect::<Vec<_>>();
    if types.len() <= 1 {
        return None;
    }

    let keyword = match kind {
        SyntaxKind::PARAM => "param",
        SyntaxKind::RESULT => "result",
        SyntaxKind::LOCAL => "local",
        SyntaxKind::FIELD => "field",
        _ => return None,
    };
    let new_text = types.into_iter().map(|ty| format!("({keyword} {ty})")).join(" ");

    let mut changes = HashMap::with_capacity_and_hasher(1, FxBuildHasher);
    changes.insert(
        uri.raw(db),
        vec![TextEdit {
            range: helpers::rowan_range_to_lsp_range(line_index, node.text_range()),
            new_text,
        }],
    );
    let title = match kind {
        SyntaxKind::PARAM => "Split parameters".into(),
        SyntaxKind::RESULT => "Split results".into(),
        SyntaxKind::LOCAL => "Split locals".into(),
        SyntaxKind::FIELD => "Split fields".into(),
        _ => return None,
    };
    Some(CodeAction {
        title,
        kind: Some(CodeActionKind::RefactorRewrite),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        ..Default::default()
    })
}
