use crate::{helpers, uri::InternUri};
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use rowan::NodeOrToken;
use rustc_hash::{FxBuildHasher, FxHashMap};
use wat_syntax::{SyntaxKind, SyntaxNode};

pub fn act(
    db: &dyn salsa::Database,
    uri: InternUri,
    line_index: &LineIndex,
    node: &SyntaxNode,
) -> Option<CodeAction> {
    let kind = match node.kind() {
        SyntaxKind::PARAM => "param",
        SyntaxKind::RESULT => "result",
        SyntaxKind::LOCAL => "local",
        SyntaxKind::FIELD => "field",
        _ => return None,
    };
    if node
        .children_with_tokens()
        .all(|node_or_token| match node_or_token {
            NodeOrToken::Node(..) => false,
            NodeOrToken::Token(token) => matches!(
                token.kind(),
                SyntaxKind::KEYWORD
                    | SyntaxKind::L_PAREN
                    | SyntaxKind::R_PAREN
                    | SyntaxKind::WHITESPACE
                    | SyntaxKind::LINE_COMMENT
                    | SyntaxKind::BLOCK_COMMENT
            ),
        })
    {
        let mut changes = FxHashMap::with_capacity_and_hasher(1, FxBuildHasher);
        changes.insert(
            uri.raw(db),
            vec![TextEdit {
                range: helpers::rowan_range_to_lsp_range(line_index, node.text_range()),
                new_text: "".into(),
            }],
        );
        Some(CodeAction {
            title: format!("Remove empty {kind}"),
            kind: Some(CodeActionKind::Refactor),
            edit: Some(WorkspaceEdit {
                changes: Some(changes),
                ..Default::default()
            }),
            ..Default::default()
        })
    } else {
        None
    }
}
