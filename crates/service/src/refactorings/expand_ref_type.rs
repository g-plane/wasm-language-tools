use crate::helpers::LineIndexExt;
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use rustc_hash::{FxBuildHasher, FxHashMap};
use wat_syntax::{AmberNode, SyntaxKind};

pub fn act(uri: &str, line_index: &LineIndex, node: AmberNode) -> Option<CodeAction> {
    let token = node.tokens_by_kind(SyntaxKind::TYPE_KEYWORD).next()?;
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
        uri.to_owned(),
        vec![TextEdit {
            range: line_index.convert(token.text_range())?,
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
