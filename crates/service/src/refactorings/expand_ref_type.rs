use crate::{LanguageService, helpers, uri::InternUri};
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use rowan::ast::support;
use rustc_hash::{FxBuildHasher, FxHashMap};
use wat_syntax::{SyntaxKind, SyntaxNode};

pub fn act(
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    node: &SyntaxNode,
) -> Option<CodeAction> {
    let token = support::token(node, SyntaxKind::TYPE_KEYWORD)?;
    let type_keyword = token.text();
    let heap_ty = match type_keyword {
        "anyref" => "any",
        "eqref" => "eq",
        "i31ref" => "i31",
        "structref" => "struct",
        "arrayref" => "array",
        "nullref" => "none",
        "funcref" => "func",
        "nullfuncref" => "nofunc",
        "exnref" => "exn",
        "nullexnref" => "noexn",
        "externref" => "extern",
        "nullexternref" => "noextern",
        _ => return None,
    };

    let mut changes = FxHashMap::with_capacity_and_hasher(1, FxBuildHasher);
    changes.insert(
        uri.raw(service),
        vec![TextEdit {
            range: helpers::rowan_range_to_lsp_range(line_index, token.text_range()),
            new_text: format!("(ref null {heap_ty})"),
        }],
    );
    Some(CodeAction {
        title: format!("Expand `{type_keyword}`"),
        kind: Some(CodeActionKind::RefactorRewrite),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        ..Default::default()
    })
}
