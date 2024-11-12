use crate::{files::FilesCtx, helpers, InternUri, LanguageService};
use line_index::LineIndex;
use lsp_types::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use rowan::TextRange;
use std::collections::HashMap;
use wat_syntax::{SyntaxKind, SyntaxNode};

pub fn act(
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    node: &SyntaxNode,
    range: TextRange,
) -> Option<CodeAction> {
    let params = node
        .children()
        .skip_while(|child| !is_joinable_param(child, range))
        .take_while(|child| is_joinable_param(child, range))
        .collect::<Vec<_>>();
    let [first_node, ..] = &params[..] else {
        return None;
    };
    let types = params
        .iter()
        .flat_map(|param| {
            param
                .children()
                .filter(|child| child.kind() == SyntaxKind::VAL_TYPE)
        })
        .collect::<Vec<_>>();
    let [first, rest @ ..] = &types[..] else {
        return None;
    };
    if rest.is_empty() {
        return None;
    }

    let mut new_text = rest
        .iter()
        .fold(format!("(param {first}"), |mut new_text, ty| {
            new_text.push(' ');
            new_text.push_str(&ty.to_string());
            new_text
        });
    new_text.push(')');

    #[expect(clippy::mutable_key_type)]
    let mut changes = HashMap::with_capacity(1);
    changes.insert(
        service.lookup_uri(uri),
        vec![TextEdit {
            range: helpers::rowan_range_to_lsp_range(
                line_index,
                TextRange::new(
                    first_node.text_range().start(),
                    params.last().unwrap_or(first_node).text_range().end(),
                ),
            ),
            new_text,
        }],
    );
    Some(CodeAction {
        title: "Join parameters".into(),
        kind: Some(CodeActionKind::REFACTOR_REWRITE),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        ..Default::default()
    })
}

fn is_joinable_param(node: &SyntaxNode, range: TextRange) -> bool {
    node.kind() == SyntaxKind::PARAM
        && range.contains_range(node.text_range())
        && !node
            .children_with_tokens()
            .any(|it| it.kind() == SyntaxKind::IDENT)
}
