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
    kind: SyntaxKind,
) -> Option<CodeAction> {
    let types = node
        .children()
        .filter(|child| child.kind() == SyntaxKind::VAL_TYPE)
        .collect::<Vec<_>>();
    let [first, rest @ ..] = &types[..] else {
        return None;
    };
    if rest.is_empty() {
        return None;
    }

    let keyword = match kind {
        SyntaxKind::PARAM => "param",
        SyntaxKind::RESULT => "result",
        SyntaxKind::LOCAL => "local",
        _ => return None,
    };
    let new_text = rest
        .iter()
        .fold(format!("({keyword} {first})"), |mut new_text, ty| {
            new_text.push_str(" (");
            new_text.push_str(keyword);
            new_text.push(' ');
            new_text.push_str(&ty.to_string());
            new_text.push(')');
            new_text
        });

    #[expect(clippy::mutable_key_type)]
    let mut changes = HashMap::with_capacity(1);
    changes.insert(
        service.lookup_uri(uri),
        vec![TextEdit {
            range: helpers::rowan_range_to_lsp_range(line_index, node.text_range()),
            new_text,
        }],
    );
    let title = match kind {
        SyntaxKind::PARAM => "Split parameters".into(),
        SyntaxKind::RESULT => "Split results".into(),
        SyntaxKind::LOCAL => "Split locals".into(),
        _ => return None,
    };
    Some(CodeAction {
        title,
        kind: Some(CodeActionKind::REFACTOR_REWRITE),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        ..Default::default()
    })
}
