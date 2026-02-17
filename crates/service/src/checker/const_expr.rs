use super::Diagnostic;
use crate::data_set::CONST_INSTRS;
use wat_syntax::{
    AmberNode, SyntaxKind, SyntaxNode, TextRange,
    ast::{AstNode, Instr},
};

const DIAGNOSTIC_CODE: &str = "const-expr";

pub fn check(node: &SyntaxNode) -> Option<Diagnostic> {
    let mut first = None;
    let mut last = None;
    let mut is_const = true;
    node.amber().children_by_kind(Instr::can_cast).for_each(|instr| {
        if first.is_none() {
            first = Some(instr);
        }
        last = Some(instr);
        is_const &= check_instr(instr);
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

fn check_instr(instr: AmberNode) -> bool {
    instr
        .tokens_by_kind(SyntaxKind::INSTR_NAME)
        .next()
        .is_some_and(|instr_name| CONST_INSTRS.contains(&instr_name.text()))
        && instr.children_by_kind(Instr::can_cast).all(check_instr)
}
