use crate::{
    binder::{SymbolKey, SymbolTable},
    helpers,
    uri::{InternUri, UrisCtx},
    LanguageService,
};
use line_index::LineIndex;
use lsp_types::{CodeAction, CodeActionKind, Range, TextEdit, WorkspaceEdit};
use rowan::ast::AstNode;
use std::collections::HashMap;
use wat_syntax::{
    ast::{CompType, ModuleFieldType, TypeUse},
    SyntaxNode,
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
    let type_def = symbol_table.find_defs(SymbolKey::new(index))?.next()?;
    let CompType::Func(func_type) = ModuleFieldType::cast(type_def.key.to_node(root))?
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
    #[expect(clippy::mutable_key_type)]
    let mut changes = HashMap::with_capacity(1);
    changes.insert(
        service.lookup_uri(uri),
        vec![TextEdit {
            range: Range { start: end, end },
            new_text,
        }],
    );
    Some(CodeAction {
        title: format!("Inline func type `{index}`"),
        kind: Some(CodeActionKind::REFACTOR_INLINE),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        ..Default::default()
    })
}
