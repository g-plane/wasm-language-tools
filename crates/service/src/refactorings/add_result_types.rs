use crate::{helpers::LineIndexExt, uri::InternUri};
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionContext, CodeActionKind, TextEdit, Union2, WorkspaceEdit};
use rustc_hash::FxBuildHasher;
use std::{collections::HashMap, ops::ControlFlow};
use wat_syntax::{NodeOrToken, SyntaxKind, SyntaxNode, TextRange, ast::support};

pub fn act(
    db: &dyn salsa::Database,
    uri: InternUri,
    line_index: &LineIndex,
    node: &SyntaxNode,
    context: &CodeActionContext,
) -> Option<CodeAction> {
    let end = line_index.convert(node.text_range().end());
    let (types, diagnostic) = context
        .diagnostics
        .iter()
        .find_map(|diagnostic| match &diagnostic.code {
            Some(Union2::B(code)) if code == "type-check" && diagnostic.range.end == end => diagnostic
                .data
                .as_ref()
                .and_then(|data| serde_json::from_value::<Vec<String>>(data.clone()).ok())
                .map(|types| (types, diagnostic)),
            _ => None,
        })?;
    let end = match node.kind() {
        SyntaxKind::MODULE_FIELD_FUNC => {
            let (ControlFlow::Continue(range) | ControlFlow::Break(range)) =
                node.children_with_tokens()
                    .try_fold(None, |range, node_or_token| match node_or_token {
                        NodeOrToken::Node(node) => {
                            if matches!(
                                node.kind(),
                                SyntaxKind::EXPORT | SyntaxKind::IMPORT | SyntaxKind::TYPE_USE
                            ) {
                                ControlFlow::Continue(Some(node.text_range()))
                            } else {
                                ControlFlow::Break(range)
                            }
                        }
                        NodeOrToken::Token(token) => {
                            if matches!(token.kind(), SyntaxKind::KEYWORD | SyntaxKind::IDENT) {
                                ControlFlow::Continue(Some(token.text_range()))
                            } else {
                                ControlFlow::Continue(range)
                            }
                        }
                    });
            range
        }
        SyntaxKind::BLOCK_BLOCK | SyntaxKind::BLOCK_LOOP | SyntaxKind::BLOCK_TRY_TABLE => node
            .children_by_kind(SyntaxKind::TYPE_USE)
            .next()
            .map(|child| child.text_range())
            .or_else(|| {
                support::token(node, SyntaxKind::IDENT)
                    .or_else(|| support::token(node, SyntaxKind::KEYWORD))
                    .map(|token| token.text_range())
            }),
        SyntaxKind::BLOCK_IF_THEN => {
            let parent = node.parent()?;
            parent
                .children_by_kind(SyntaxKind::TYPE_USE)
                .next()
                .map(|child| child.text_range())
                .or_else(|| {
                    support::token(&parent, SyntaxKind::IDENT)
                        .or_else(|| support::token(&parent, SyntaxKind::KEYWORD))
                        .map(|token| token.text_range())
                })
        }
        _ => None,
    }?
    .end();

    let mut changes = HashMap::with_capacity_and_hasher(1, FxBuildHasher);
    changes.insert(
        uri.raw(db),
        vec![TextEdit {
            range: line_index.convert(TextRange::empty(end)),
            new_text: format!(" (result {})", types.join(" ")),
        }],
    );
    Some(CodeAction {
        title: if let Some(ident) = support::token(node, SyntaxKind::IDENT) {
            format!("Add result types to `{}`", ident.text())
        } else {
            "Add result types".into()
        },
        kind: Some(CodeActionKind::QuickFix),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        diagnostics: Some(vec![diagnostic.clone()]),
        ..Default::default()
    })
}
