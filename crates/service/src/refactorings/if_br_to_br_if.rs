use crate::{helpers::LineIndexExt, uri::InternUri};
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use rowan::{GreenToken, NodeOrToken, ast::AstNode};
use rustc_hash::FxBuildHasher;
use std::collections::HashMap;
use wat_syntax::{
    SyntaxKind, SyntaxNode,
    ast::{BlockIf, Instr},
};

pub fn act(db: &dyn salsa::Database, uri: InternUri, line_index: &LineIndex, node: &SyntaxNode) -> Option<CodeAction> {
    let block_if = BlockIf::cast(node.clone())?;
    if block_if.else_block().is_some() {
        return None;
    }
    let mut then_instrs = block_if.then_block()?.instrs();
    let first_instr = then_instrs
        .next()
        .and_then(|instr| match instr {
            Instr::Plain(plain) => Some(plain),
            _ => None,
        })?
        .clone_subtree();
    if then_instrs.next().is_some() {
        return None;
    }
    let instr_name = first_instr.instr_name()?;
    if instr_name.text() != "br" {
        return None;
    }

    let br_if = instr_name.replace_with(GreenToken::new(SyntaxKind::INSTR_NAME.into(), "br_if"));
    let new_text = if block_if.l_paren_token().is_some() {
        let i = if let Some(token) = first_instr.r_paren_token() {
            token.index()
        } else {
            first_instr.syntax().last_child()?.index()
        };
        br_if
            .splice_children(
                i..i,
                block_if.instrs().flat_map(|instr| {
                    [
                        NodeOrToken::Token(GreenToken::new(SyntaxKind::WHITESPACE.into(), " ")),
                        NodeOrToken::Node(instr.syntax().green().into()),
                    ]
                }),
            )
            .to_string()
    } else {
        br_if.to_string()
    };

    let mut changes = HashMap::with_capacity_and_hasher(1, FxBuildHasher);
    changes.insert(
        uri.raw(db),
        vec![TextEdit {
            range: line_index.convert(node.text_range()),
            new_text,
        }],
    );
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
