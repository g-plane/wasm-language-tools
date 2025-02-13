use crate::{
    binder::{SymbolKey, SymbolTable},
    helpers,
    idx::IdentsCtx,
    uri::{InternUri, UrisCtx},
    LanguageService,
};
use line_index::LineIndex;
use lsp_types::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use std::collections::HashMap;
use wat_syntax::SyntaxNode;

pub fn act(
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) -> Option<CodeAction> {
    let ref_key = SymbolKey::new(node);
    let ref_idx = &symbol_table
        .symbols
        .iter()
        .find(|symbol| symbol.key == ref_key)?
        .idx;
    let def_idx = if let Some(mut defs) = symbol_table.find_defs(ref_key) {
        &defs.next()?.idx
    } else if let Some(def) = symbol_table.find_param_or_local_def(ref_key) {
        &def.idx
    } else {
        &symbol_table
            .blocks
            .iter()
            .find(|block| block.ref_key == ref_key)?
            .def_idx
    };
    let def_num = def_idx.num?;
    let def_name = def_idx.name?;
    let (new_text, title) = if ref_idx.name.is_some() {
        (
            def_num.to_string(),
            "Convert identifier to numeric idx".into(),
        )
    } else {
        (
            service.lookup_ident(def_name).to_string(),
            "Convert numeric idx to identifier".into(),
        )
    };

    #[expect(clippy::mutable_key_type)]
    let mut changes = HashMap::with_capacity(1);
    changes.insert(
        service.lookup_uri(uri),
        vec![TextEdit {
            range: helpers::rowan_range_to_lsp_range(line_index, node.text_range()),
            new_text,
        }],
    );
    Some(CodeAction {
        title,
        kind: Some(CodeActionKind::REFACTOR_REWRITE),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        ..Default::default()
    })
}
