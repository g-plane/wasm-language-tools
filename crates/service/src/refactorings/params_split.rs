use crate::{files::FilesCtx, helpers, InternUri, LanguageService};
use line_index::LineIndex;
use lsp_types::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use std::collections::HashMap;
use wat_syntax::{SyntaxKind, SyntaxNode};

pub fn act(
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    node: &SyntaxNode,
) -> Option<CodeAction> {
    let types = node
        .children_with_tokens()
        .filter(|child| child.kind() == SyntaxKind::VAL_TYPE)
        .collect::<Vec<_>>();
    let [first, rest @ ..] = &types[..] else {
        return None;
    };
    if rest.is_empty() {
        return None;
    }

    let new_text = rest
        .iter()
        .fold(format!("(param {first})"), |mut new_text, ty| {
            new_text.push_str(" (param ");
            new_text.push_str(&ty.to_string());
            new_text.push(')');
            new_text
        });

    let mut changes = HashMap::with_capacity(1);
    changes.insert(
        service.lookup_uri(uri),
        vec![TextEdit {
            range: helpers::rowan_range_to_lsp_range(line_index, node.text_range()),
            new_text,
        }],
    );
    Some(CodeAction {
        title: "Split parameters".into(),
        kind: Some(CodeActionKind::REFACTOR_REWRITE),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        ..Default::default()
    })
}
