use crate::{
    binder::{SymbolKey, SymbolTable},
    helpers,
    uri::InternUri,
};
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use rustc_hash::FxBuildHasher;
use std::collections::HashMap;
use wat_syntax::SyntaxNode;

pub fn act(
    db: &dyn salsa::Database,
    uri: InternUri,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) -> Option<CodeAction> {
    let ref_key = SymbolKey::new(node);
    let ref_idx = symbol_table.symbols.get(&ref_key)?.idx;
    let def_idx = symbol_table.find_def(ref_key)?.idx;
    let def_num = def_idx.num?;
    let def_name = def_idx.name?;
    let (new_text, title) = if ref_idx.name.is_some() {
        (
            def_num.to_string(),
            "Convert identifier to numeric idx".into(),
        )
    } else {
        (
            def_name.ident(db).to_string(),
            "Convert numeric idx to identifier".into(),
        )
    };

    let mut changes = HashMap::with_capacity_and_hasher(1, FxBuildHasher);
    changes.insert(
        uri.raw(db),
        vec![TextEdit {
            range: helpers::rowan_range_to_lsp_range(line_index, node.text_range()),
            new_text,
        }],
    );
    Some(CodeAction {
        title,
        kind: Some(CodeActionKind::RefactorRewrite),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        ..Default::default()
    })
}
