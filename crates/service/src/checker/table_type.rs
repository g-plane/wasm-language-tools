use super::Diagnostic;
use crate::{
    helpers,
    types_analyzer::{self, ValType},
};
use std::num::IntErrorKind;
use wat_syntax::{AmberNode, AmberToken, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "table-type";

pub fn check(diagnostics: &mut Vec<Diagnostic>, node: AmberNode) -> Option<()> {
    let addr_type = types_analyzer::extract_addr_type(node.green());
    let upper_bound = match addr_type {
        ValType::I32 => u32::MAX as u64,
        ValType::I64 => u64::MAX,
        _ => return None,
    };

    let limits = node.children_by_kind(SyntaxKind::LIMITS).next()?;
    let mut uints = limits.tokens_by_kind(SyntaxKind::UNSIGNED_INT);
    let min_token = uints.next()?;
    let min = match helpers::parse_u64(min_token.text()) {
        Ok(min) => {
            if min > upper_bound {
                diagnostics.push(report_overflow(min_token, upper_bound));
            }
            min
        }
        Err(error) if error.kind() == &IntErrorKind::PosOverflow => {
            diagnostics.push(report_overflow(min_token, upper_bound));
            u64::MAX
        }
        Err(_) => return None,
    };
    if let Some(max_token) = uints.next() {
        let max = match helpers::parse_u64(max_token.text()) {
            Ok(max) => {
                if max > upper_bound {
                    diagnostics.push(report_overflow(max_token, upper_bound));
                }
                max
            }
            Err(error) if error.kind() == &IntErrorKind::PosOverflow => {
                diagnostics.push(report_overflow(max_token, upper_bound));
                u64::MAX
            }
            Err(_) => return None,
        };
        if max < min {
            diagnostics.push(Diagnostic {
                range: max_token.text_range(),
                code: DIAGNOSTIC_CODE.into(),
                message: "maximum size must be greater than minimum size".into(),
                ..Default::default()
            });
        }
    }
    Some(())
}

fn report_overflow(token: AmberToken, upper_bound: u64) -> Diagnostic {
    Diagnostic {
        range: token.text_range(),
        code: DIAGNOSTIC_CODE.into(),
        message: format!("table limit can't be greater than {upper_bound}"),
        ..Default::default()
    }
}
