use crate::{helpers::LineIndexExt, uri::InternUri};
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use rustc_hash::FxBuildHasher;
use std::{collections::HashMap, fmt::Write};
use wat_syntax::{
    SyntaxKind, SyntaxNode,
    ast::{AstNode, ExternType},
};

pub fn act(db: &dyn salsa::Database, uri: InternUri, line_index: &LineIndex, node: &SyntaxNode) -> Option<CodeAction> {
    let module_name = node.children_by_kind(SyntaxKind::MODULE_NAME).next()?;
    let extern_type = node.children_by_kind(ExternType::can_cast).next();
    let imports = node
        .children_by_kind(SyntaxKind::IMPORT_ITEM)
        .filter_map(|import_item| {
            let name = import_item.children_by_kind(SyntaxKind::NAME).next()?;
            let mut new_text = format!("(import {module_name} {name}");
            if let Some(extern_type) = import_item
                .children_by_kind(ExternType::can_cast)
                .next()
                .as_ref()
                .or(extern_type.as_ref())
            {
                let _ = write!(&mut new_text, " {extern_type}");
            }
            new_text.push(')');
            Some(new_text)
        })
        .collect::<Vec<_>>();
    if imports.is_empty() {
        None
    } else {
        let mut changes = HashMap::with_capacity_and_hasher(1, FxBuildHasher);
        changes.insert(
            uri.raw(db),
            vec![TextEdit {
                range: line_index.convert(node.text_range()),
                new_text: imports.join("\n  "),
            }],
        );
        Some(CodeAction {
            title: "Expand compact import".into(),
            kind: Some(CodeActionKind::RefactorRewrite),
            edit: Some(WorkspaceEdit {
                changes: Some(changes),
                ..Default::default()
            }),
            ..Default::default()
        })
    }
}
