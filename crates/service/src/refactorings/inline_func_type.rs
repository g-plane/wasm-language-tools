use crate::{
    LanguageService,
    binder::{SymbolKey, SymbolTable},
    helpers,
    uri::InternUri,
};
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionKind, Range, TextEdit, WorkspaceEdit};
use rowan::ast::AstNode;
use rustc_hash::FxBuildHasher;
use std::collections::HashMap;
use wat_syntax::{
    SyntaxNode,
    ast::{CompType, TypeDef, TypeUse},
};

pub fn act(
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) -> Option<CodeAction> {
    let type_use = TypeUse::cast(node.clone())?;
    if type_use.params().count() > 0 || type_use.results().count() > 0 {
        return None;
    }

    let index = type_use.index()?;
    let index = index.syntax();
    let type_def = symbol_table.find_def(SymbolKey::new(index))?;
    let CompType::Func(func_type) = TypeDef::cast(type_def.key.to_node(root))?
        .sub_type()?
        .comp_type()?
    else {
        return None;
    };
    func_type.syntax().first_child()?; // skip empty func type
    let mut new_text = String::with_capacity(8);
    for node in func_type.syntax().children() {
        new_text.push(' ');
        new_text.push_str(&node.to_string());
    }

    let end = helpers::rowan_pos_to_lsp_pos(line_index, node.text_range().end());
    let mut changes = HashMap::with_capacity_and_hasher(1, FxBuildHasher);
    changes.insert(
        uri.raw(service),
        vec![TextEdit {
            range: Range { start: end, end },
            new_text,
        }],
    );
    Some(CodeAction {
        title: format!("Inline func type `{index}`"),
        kind: Some(CodeActionKind::RefactorInline),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        ..Default::default()
    })
}
