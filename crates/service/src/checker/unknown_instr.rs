use super::{Diagnostic, FastPlainInstr};
use crate::data_set::INSTR_OP_CODES;

const DIAGNOSTIC_CODE: &str = "unknown-instr";

pub fn check(instr: &FastPlainInstr) -> Option<Diagnostic> {
    if INSTR_OP_CODES.contains_key(instr.name) {
        None
    } else {
        Some(Diagnostic {
            range: instr.name_range,
            code: DIAGNOSTIC_CODE.into(),
            message: format!("unknown instruction `{}`", instr.name),
            ..Default::default()
        })
    }
}
