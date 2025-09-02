use crate::{helpers, uri::InternUri, LanguageService};
use itertools::Itertools;
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use rowan::ast::AstNode;
use rustc_hash::FxBuildHasher;
use std::collections::HashMap;
use wat_syntax::{ast::PlainInstr, SyntaxNode};

pub fn act(
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    node: &SyntaxNode,
) -> Option<CodeAction> {
    let instr = PlainInstr::cast(node.clone())?;
    if instr.instr_name()?.text() != "br_if" {
        return None;
    }

    let indent = " ".repeat(line_index.line_col(node.text_range().start()).col as usize);
    let new_text = if instr.l_paren_token().is_some() {
        format!(
            "(if{}\n{indent}  (then\n{indent}    (br{})))",
            instr
                .instrs()
                .map(|instr| format!("\n{indent}  {}", instr.syntax()))
                .join(""),
            instr
                .immediates()
                .map(|immediate| format!(" {}", immediate.syntax()))
                .join(" "),
        )
    } else {
        format!(
            "if\n{indent}  br{}\n{indent}end",
            instr
                .immediates()
                .map(|immediate| format!(" {}", immediate.syntax()))
                .join(" "),
        )
    };

    let mut changes = HashMap::with_capacity_and_hasher(1, FxBuildHasher);
    changes.insert(
        uri.raw(service),
        vec![TextEdit {
            range: helpers::rowan_range_to_lsp_range(line_index, node.text_range()),
            new_text,
        }],
    );
    Some(CodeAction {
        title: "Convert `br_if` to `if` with `br`".into(),
        kind: Some(CodeActionKind::RefactorRewrite),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        ..Default::default()
    })
}
