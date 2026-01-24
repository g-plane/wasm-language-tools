use super::Diagnostic;
use crate::{data_set::INSTR_NAMES, helpers};
use rowan::ast::support;
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "unknown-instr";

pub fn check(node: &SyntaxNode) -> Option<Diagnostic> {
    let token = support::token(node, SyntaxKind::INSTR_NAME)?;
    let instr_name = token.text();
    if INSTR_NAMES.contains(&instr_name) {
        None
    } else {
        let message = if let Some(guess) = helpers::fuzzy_search(INSTR_NAMES, instr_name) {
            format!("unknown instruction `{instr_name}`, do you mean `{guess}`?")
        } else {
            format!("unknown instruction `{instr_name}`")
        };
        Some(Diagnostic {
            range: token.text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message,
            ..Default::default()
        })
    }
}
