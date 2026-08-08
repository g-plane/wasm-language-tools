use crate::{
    binder::SymbolTable,
    helpers::{self, LineIndexExt},
};
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use rustc_hash::{FxBuildHasher, FxHashMap};
use wat_syntax::{AmberNode, SyntaxKind, TextRange};

pub fn act(uri: &str, line_index: &LineIndex, symbol_table: &SymbolTable, node: AmberNode) -> Option<CodeAction> {
    let index = helpers::syntax::extract_index_from_export(node)?;
    let def_node = symbol_table.find_def(index.into())?.amber();

    let mut changes = FxHashMap::with_capacity_and_hasher(1, FxBuildHasher);
    changes.insert(
        uri.to_owned(),
        vec![
            TextEdit {
                range: line_index.convert(node.text_range())?,
                new_text: "".into(),
            },
            TextEdit {
                range: line_index.convert(TextRange::empty(
                    def_node
                        .tokens_by_kind(SyntaxKind::IDENT)
                        .next()
                        .or_else(|| def_node.tokens_by_kind(SyntaxKind::KEYWORD).next())?
                        .text_range()
                        .end(),
                ))?,
                new_text: format!(" (export {})", node.children_by_kind(SyntaxKind::NAME).next()?.green()),
            },
        ],
    );
    Some(CodeAction {
        title: "Inline export".into(),
        kind: Some(CodeActionKind::RefactorInline),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        ..Default::default()
    })
}
