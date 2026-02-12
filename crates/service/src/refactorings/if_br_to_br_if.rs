use crate::{helpers::LineIndexExt, uri::InternUri};
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use rustc_hash::FxBuildHasher;
use std::collections::HashMap;
use wat_syntax::{
    SyntaxKind, SyntaxNode,
    ast::{AstNode, BlockIf, Instr},
};

pub fn act(db: &dyn salsa::Database, uri: InternUri, line_index: &LineIndex, node: &SyntaxNode) -> Option<CodeAction> {
    let block_if = BlockIf::cast(node.clone())?;
    if block_if.else_block().is_some() {
        return None;
    }
    let then_block = block_if.then_block()?;
    let mut then_instrs = then_block.instrs();
    let first_instr = then_instrs.next().and_then(|instr| match instr {
        Instr::Plain(plain) => Some(plain),
        _ => None,
    })?;
    if then_instrs.next().is_some() {
        return None;
    }
    let instr_name = first_instr.instr_name()?;
    if instr_name.text() != "br" {
        return None;
    }
    let mut first_instr_children = first_instr.syntax().children();
    let first_immediate = first_instr_children
        .next()
        .filter(|child| child.kind() == SyntaxKind::IMMEDIATE)?;
    if first_instr_children.next().is_some() {
        return None;
    }

    let mut text_edits = Vec::with_capacity(1);
    if let Some(l_paren) = block_if.l_paren_token() {
        if let Some(keyword) = block_if.keyword() {
            text_edits.push(TextEdit {
                range: line_index.convert(l_paren.text_range().cover(keyword.text_range())),
                new_text: "".into(),
            });
        }
        text_edits.push(TextEdit {
            range: line_index.convert(then_block.syntax().text_range()),
            new_text: format!("(br_if {first_immediate})"),
        });
        if let Some(r_paren) = block_if.r_paren_token() {
            text_edits.push(TextEdit {
                range: line_index.convert(r_paren.text_range()),
                new_text: "".into(),
            });
        }
    } else {
        text_edits.push(TextEdit {
            range: line_index.convert(node.text_range()),
            new_text: format!("br_if {first_immediate}"),
        });
    };

    let mut changes = HashMap::with_capacity_and_hasher(1, FxBuildHasher);
    changes.insert(uri.raw(db), text_edits);
    Some(CodeAction {
        title: "Convert `if` with `br` to `br_if`".into(),
        kind: Some(CodeActionKind::RefactorRewrite),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        ..Default::default()
    })
}
