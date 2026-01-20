use crate::{
    binder::{SymbolKey, SymbolTable},
    helpers,
    uri::InternUri,
};
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use rowan::{TextRange, ast::support};
use rustc_hash::{FxBuildHasher, FxHashMap};
use wat_syntax::{SyntaxKind, SyntaxNode};

pub fn act(
    db: &dyn salsa::Database,
    uri: InternUri,
    line_index: &LineIndex,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) -> Option<CodeAction> {
    let index = helpers::ast::extract_index_from_export(node)?;
    let def_node = symbol_table
        .resolved
        .get(&SymbolKey::new(&index))?
        .try_to_node(root)?;

    let mut changes = FxHashMap::with_capacity_and_hasher(1, FxBuildHasher);
    changes.insert(
        uri.raw(db),
        vec![
            TextEdit {
                range: helpers::rowan_range_to_lsp_range(line_index, node.text_range()),
                new_text: "".into(),
            },
            TextEdit {
                range: helpers::rowan_range_to_lsp_range(
                    line_index,
                    TextRange::empty(
                        support::token(&def_node, SyntaxKind::IDENT)
                            .or_else(|| support::token(&def_node, SyntaxKind::KEYWORD))?
                            .text_range()
                            .end(),
                    ),
                ),
                new_text: format!(
                    " (export {})",
                    node.first_child_by_kind(&|kind| kind == SyntaxKind::NAME)?
                        .text(),
                ),
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
