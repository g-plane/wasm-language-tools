use crate::{helpers, types_analyzer::RefType, uri::InternUri};
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use rowan::ast::support;
use rustc_hash::{FxBuildHasher, FxHashMap};
use wat_syntax::{SyntaxKind, SyntaxNode};

pub fn act(db: &dyn salsa::Database, uri: InternUri, line_index: &LineIndex, node: &SyntaxNode) -> Option<CodeAction> {
    if !RefType::from_green(&node.green(), db)?.nullable {
        return None;
    }

    let token = node
        .first_child_by_kind(&|kind| kind == SyntaxKind::HEAP_TYPE)
        .and_then(|heap_ty| support::token(&heap_ty, SyntaxKind::TYPE_KEYWORD))?;
    let ref_type = match token.text() {
        "any" => "anyref",
        "eq" => "eqref",
        "i31" => "i31ref",
        "struct" => "structref",
        "array" => "arrayref",
        "none" => "nullref",
        "func" => "funcref",
        "nofunc" => "nullfuncref",
        "exn" => "exnref",
        "noexn" => "nullexnref",
        "extern" => "externref",
        "noextern" => "nullexternref",
        _ => return None,
    };

    let mut changes = FxHashMap::with_capacity_and_hasher(1, FxBuildHasher);
    changes.insert(
        uri.raw(db),
        vec![TextEdit {
            range: helpers::rowan_range_to_lsp_range(line_index, node.text_range()),
            new_text: ref_type.into(),
        }],
    );
    Some(CodeAction {
        title: format!("Simplify to `{ref_type}`"),
        kind: Some(CodeActionKind::RefactorRewrite),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        ..Default::default()
    })
}
