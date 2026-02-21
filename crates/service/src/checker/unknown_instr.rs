use super::Diagnostic;
use crate::data_set::INSTR_OP_CODES;
use wat_syntax::AmberToken;

const DIAGNOSTIC_CODE: &str = "unknown-instr";

pub fn check(instr_name: AmberToken) -> Option<Diagnostic> {
    let name = instr_name.text();
    if INSTR_OP_CODES.contains_key(name) {
        None
    } else {
        Some(Diagnostic {
            range: instr_name.text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message: format!("unknown instruction `{name}`"),
            ..Default::default()
        })
    }
}
