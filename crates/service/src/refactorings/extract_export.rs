use crate::{
    binder::{SymbolKey, SymbolTable},
    helpers::LineIndexExt,
    uri::InternUri,
};
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use rustc_hash::{FxBuildHasher, FxHashMap};
use wat_syntax::{SyntaxKind, SyntaxNode, TextRange};

pub fn act(
    db: &dyn salsa::Database,
    uri: InternUri,
    line_index: &LineIndex,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) -> Option<CodeAction> {
    let parent = node.parent()?;
    let def_symbol = symbol_table.symbols.get(&SymbolKey::new(&parent))?;
    let last_module_field = def_symbol.region.try_to_node(root)?.last_child()?;

    let mut changes = FxHashMap::with_capacity_and_hasher(1, FxBuildHasher);
    changes.insert(
        uri.raw(db),
        vec![
            TextEdit {
                range: line_index.convert(node.text_range()),
                new_text: "".into(),
            },
            TextEdit {
                range: line_index.convert(TextRange::empty(last_module_field.text_range().end())),
                new_text: format!(
                    "\n  (export {} ({} {}))",
                    node.children_by_kind(SyntaxKind::NAME).next()?,
                    def_symbol.kind,
                    def_symbol.idx.render(db),
                ),
            },
        ],
    );
    Some(CodeAction {
        title: "Extract export as a module field".into(),
        kind: Some(CodeActionKind::RefactorExtract),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        ..Default::default()
    })
}
