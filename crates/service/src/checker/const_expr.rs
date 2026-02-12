use super::Diagnostic;
use crate::data_set::CONST_INSTRS;
use wat_syntax::{
    SyntaxKind, SyntaxNode, TextRange,
    ast::{AstNode, Instr, support},
};

const DIAGNOSTIC_CODE: &str = "const-expr";

pub fn check(node: &SyntaxNode) -> Option<Diagnostic> {
    let mut first = None;
    let mut last = None;
    let mut is_const = true;
    node.children()
        .filter(|child| Instr::can_cast(child.kind()))
        .for_each(|instr| {
            if first.is_none() {
                first = Some(instr.clone());
            }
            last = Some(instr.clone());
            let mut descendants = instr.descendants();
            while let Some(node) = descendants.next() {
                match node.kind() {
                    SyntaxKind::BLOCK_BLOCK
                    | SyntaxKind::BLOCK_LOOP
                    | SyntaxKind::BLOCK_IF
                    | SyntaxKind::BLOCK_TRY_TABLE => {
                        is_const = false;
                        break;
                    }
                    SyntaxKind::PLAIN_INSTR => {
                        if support::token(&node, SyntaxKind::INSTR_NAME)
                            .is_some_and(|instr_name| !CONST_INSTRS.contains(&instr_name.text()))
                        {
                            is_const = false;
                            break;
                        }
                    }
                    _ => {
                        descendants.skip_subtree();
                    }
                }
            }
        });
    if !is_const
        && let Some(first) = first
        && let Some(last) = last
    {
        Some(Diagnostic {
            range: TextRange::cover(first.text_range(), last.text_range()),
            code: DIAGNOSTIC_CODE.into(),
            message: "expression must be constant".into(),
            ..Default::default()
        })
    } else {
        None
    }
}
