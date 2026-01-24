use super::Diagnostic;
use crate::data_set::CONST_INSTRS;
use rowan::{TextRange, ast::AstNode};
use wat_syntax::{SyntaxNode, ast::Instr};

const DIAGNOSTIC_CODE: &str = "const-expr";

pub fn check(node: &SyntaxNode) -> Option<Diagnostic> {
    let first = node.first_child_by_kind(&Instr::can_cast)?;
    let last = node.children().filter(|child| Instr::can_cast(child.kind())).last()?;
    if node.descendants().filter_map(Instr::cast).all(|instr| {
        if let Instr::Plain(plain) = instr {
            plain
                .instr_name()
                .is_some_and(|instr_name| CONST_INSTRS.contains(&instr_name.text()))
        } else {
            false
        }
    }) {
        None
    } else {
        Some(Diagnostic {
            range: TextRange::cover(first.text_range(), last.text_range()),
            code: DIAGNOSTIC_CODE.into(),
            message: "expression must be constant".into(),
            ..Default::default()
        })
    }
}
