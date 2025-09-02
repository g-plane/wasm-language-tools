use crate::{helpers, uri::InternUri, LanguageService};
use itertools::Itertools;
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use rowan::TextRange;
use rustc_hash::FxBuildHasher;
use std::collections::HashMap;
use wat_syntax::{SyntaxKind, SyntaxNode};

pub fn act(
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    node: &SyntaxNode,
    kind: SyntaxKind,
    range: TextRange,
) -> Option<CodeAction> {
    let header_items = node
        .children()
        .skip_while(|child| !can_join(child, kind, range))
        .take_while(|child| can_join(child, kind, range))
        .collect::<Vec<_>>();
    let [first_node, ..] = &header_items[..] else {
        return None;
    };
    let types = header_items
        .iter()
        .flat_map(|header_item| header_item.children())
        .collect::<Vec<_>>();
    if types.len() <= 1 {
        return None;
    }

    let keyword = match kind {
        SyntaxKind::PARAM => "param",
        SyntaxKind::RESULT => "result",
        SyntaxKind::LOCAL => "local",
        _ => return None,
    };
    let new_text = format!(
        "({keyword} {})",
        types.iter().map(|ty| ty.to_string()).join(" ")
    );

    let mut changes = HashMap::with_capacity_and_hasher(1, FxBuildHasher);
    changes.insert(
        uri.raw(service),
        vec![TextEdit {
            range: helpers::rowan_range_to_lsp_range(
                line_index,
                TextRange::new(
                    first_node.text_range().start(),
                    header_items.last().unwrap_or(first_node).text_range().end(),
                ),
            ),
            new_text,
        }],
    );
    let title = match kind {
        SyntaxKind::PARAM => "Join parameters".into(),
        SyntaxKind::RESULT => "Join results".into(),
        SyntaxKind::LOCAL => "Join locals".into(),
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
        && !node
            .children_with_tokens()
            .any(|it| it.kind() == SyntaxKind::IDENT)
}
