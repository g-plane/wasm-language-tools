use crate::{binder::SymbolKey, document::Document};
use rowan::{Direction, NodeOrToken};
use rustc_hash::FxHashMap;
use wat_syntax::SyntaxKind;

#[salsa::tracked(returns(ref))]
pub(crate) fn get_deprecation(db: &dyn salsa::Database, document: Document) -> FxHashMap<SymbolKey, Option<String>> {
    let root = document.root_tree(db);
    root.children()
        .flat_map(|module| module.children())
        .filter(|node| {
            matches!(
                node.kind(),
                SyntaxKind::MODULE_FIELD_FUNC
                    | SyntaxKind::MODULE_FIELD_GLOBAL
                    | SyntaxKind::MODULE_FIELD_MEMORY
                    | SyntaxKind::MODULE_FIELD_TABLE
                    | SyntaxKind::MODULE_FIELD_TAG
                    | SyntaxKind::TYPE_DEF
                    | SyntaxKind::MODULE_FIELD_IMPORT
                    | SyntaxKind::MODULE_FIELD_DATA
            )
        })
        .chain(
            root.children()
                .flat_map(|module| module.children())
                .filter(|node| node.kind() == SyntaxKind::REC_TYPE)
                .flat_map(|node| node.children()),
        )
        .filter_map(|node| {
            let key = if node.kind() == SyntaxKind::MODULE_FIELD_IMPORT {
                SymbolKey::new(&node.first_child_by_kind(&|kind| {
                    matches!(
                        kind,
                        SyntaxKind::EXTERN_TYPE_FUNC
                            | SyntaxKind::EXTERN_TYPE_GLOBAL
                            | SyntaxKind::EXTERN_TYPE_MEMORY
                            | SyntaxKind::EXTERN_TYPE_TABLE
                            | SyntaxKind::EXTERN_TYPE_TAG
                    )
                })?)
            } else {
                SymbolKey::new(&node)
            };
            node.siblings_with_tokens(Direction::Prev)
                .skip(1)
                .map_while(|node_or_token| match node_or_token {
                    NodeOrToken::Token(token) if token.kind().is_trivia() => Some(token),
                    _ => None,
                })
                .find(|token| {
                    token.kind() == SyntaxKind::ANNOT_START
                        && token
                            .text()
                            .strip_prefix("(@")
                            .map(|s| s.strip_prefix('"').and_then(|s| s.strip_suffix('"')).unwrap_or(s))
                            .is_some_and(|name| name == "deprecated")
                })
                .map(|annot_start| {
                    let reason = annot_start
                        .siblings_with_tokens(Direction::Next)
                        .map_while(|node_or_token| match node_or_token {
                            NodeOrToken::Token(token) if token.kind().is_trivia() => Some(token),
                            _ => None,
                        })
                        .find_map(|token| {
                            if token.kind() == SyntaxKind::ANNOT_ELEM {
                                token
                                    .text()
                                    .strip_prefix('"')
                                    .and_then(|s| s.strip_suffix('"'))
                                    .map(String::from)
                            } else {
                                None
                            }
                        });
                    (key, reason)
                })
        })
        .collect()
}
