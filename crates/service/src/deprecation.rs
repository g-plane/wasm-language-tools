use crate::{binder::SymbolKey, document::Document};
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
                    | SyntaxKind::MODULE_FIELD_ELEM
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
                node.amber()
                    .children_by_kind(|kind| {
                        matches!(
                            kind,
                            SyntaxKind::EXTERN_TYPE_FUNC
                                | SyntaxKind::EXTERN_TYPE_GLOBAL
                                | SyntaxKind::EXTERN_TYPE_MEMORY
                                | SyntaxKind::EXTERN_TYPE_TABLE
                                | SyntaxKind::EXTERN_TYPE_TAG
                        )
                    })
                    .next()?
                    .to_ptr()
                    .into()
            } else {
                SymbolKey::new(&node)
            };
            node.prev_consecutive_tokens()
                .take_while(|token| token.kind().is_trivia())
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
                        .next_consecutive_tokens()
                        .take_while(|token| token.kind().is_trivia())
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
