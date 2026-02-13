use crate::{helpers::LineIndexExt, uri::InternUri};
use itertools::Itertools;
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use rustc_hash::FxBuildHasher;
use std::collections::HashMap;
use wat_syntax::{SyntaxKind, SyntaxNode, TextRange};

pub fn act(
    db: &dyn salsa::Database,
    uri: InternUri,
    line_index: &LineIndex,
    node: &SyntaxNode,
    kind: SyntaxKind,
    range: TextRange,
) -> Option<CodeAction> {
    let items = node
        .children()
        .skip_while(|child| !can_join(child, kind, range))
        .take_while(|child| can_join(child, kind, range))
        .collect::<Vec<_>>();
    let first_node = items.first()?;
    let types = items.iter().flat_map(|item| item.children()).collect::<Vec<_>>();
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
    let new_text = format!("({keyword} {})", types.iter().map(|ty| ty.to_string()).join(" "));

    let mut changes = HashMap::with_capacity_and_hasher(1, FxBuildHasher);
    changes.insert(
        uri.raw(db),
        vec![TextEdit {
            range: line_index.convert(TextRange::new(
                first_node.text_range().start(),
                items.last().unwrap_or(first_node).text_range().end(),
            )),
            new_text,
        }],
    );
    let title = match kind {
        SyntaxKind::PARAM => "Join parameters".into(),
        SyntaxKind::RESULT => "Join results".into(),
        SyntaxKind::LOCAL => "Join locals".into(),
        SyntaxKind::FIELD => "Join fields".into(),
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

fn can_join(node: &SyntaxNode, kind: SyntaxKind, range: TextRange) -> bool {
    node.kind() == kind
        && range.contains_range(node.text_range())
        && !node.has_child_or_token_by_kind(|kind| kind == SyntaxKind::IDENT)
}
