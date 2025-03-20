use crate::{
    helpers,
    uri::{InternUri, UrisCtx},
    LanguageService,
};
use line_index::LineIndex;
use lspt::{
    CodeAction, CodeActionContext, CodeActionKind, Diagnostic, TextEdit, Union2, WorkspaceEdit,
};
use rowan::{Direction, TextRange};
use rustc_hash::FxBuildHasher;
use std::collections::HashMap;
use wat_syntax::{SyntaxElement, SyntaxKind, SyntaxNode};

pub fn act(
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    node: &SyntaxNode,
    context: &CodeActionContext,
) -> Option<Vec<CodeAction>> {
    let node_lsp_range = helpers::rowan_range_to_lsp_range(line_index, node.text_range());
    let diagnostic = context
        .diagnostics
        .iter()
        .find(|diagnostic| match &diagnostic.code {
            Some(Union2::B(code)) => code == "packing" && diagnostic.range == node_lsp_range,
            _ => false,
        })?;
    let instr_name =
        node.siblings_with_tokens(Direction::Prev)
            .find_map(|element| match element {
                SyntaxElement::Token(token) if token.kind() == SyntaxKind::INSTR_NAME => {
                    Some(token)
                }
                _ => None,
            })?;

    let range = instr_name.text_range();
    match instr_name.text() {
        "struct.get" => Some(
            ["struct.get_s", "struct.get_u"]
                .iter()
                .map(|new_text| build_action(new_text, range, diagnostic, service, uri, line_index))
                .collect(),
        ),
        "struct.get_s" | "struct.get_u" => Some(vec![build_action(
            "struct.get",
            range,
            diagnostic,
            service,
            uri,
            line_index,
        )]),
        "array.get" => Some(
            ["array.get_s", "array.get_u"]
                .iter()
                .map(|new_text| build_action(new_text, range, diagnostic, service, uri, line_index))
                .collect(),
        ),
        "array.get_s" | "array.get_u" => Some(vec![build_action(
            "array.get",
            range,
            diagnostic,
            service,
            uri,
            line_index,
        )]),
        _ => None,
    }
}

fn build_action(
    new_text: &str,
    range: TextRange,
    diagnostic: &Diagnostic,
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
) -> CodeAction {
    let text_edits = vec![TextEdit {
        range: helpers::rowan_range_to_lsp_range(line_index, range),
        new_text: new_text.into(),
    }];
    let mut changes = HashMap::with_capacity_and_hasher(1, FxBuildHasher);
    changes.insert(service.lookup_uri(uri), text_edits);
    CodeAction {
        title: format!("Replace instruction with `{new_text}`"),
        kind: Some(CodeActionKind::QuickFix),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        is_preferred: Some(true),
        diagnostics: Some(vec![diagnostic.clone()]),
        ..Default::default()
    }
}
