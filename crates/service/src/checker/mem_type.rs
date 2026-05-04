use super::Diagnostic;
use crate::{
    helpers,
    types_analyzer::{self, ValType},
};
use std::num::IntErrorKind;
use wat_syntax::{AmberNode, AmberToken, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "mem-type";

pub fn check(diagnostics: &mut Vec<Diagnostic>, node: AmberNode) -> Option<()> {
    let limits = node.children_by_kind(SyntaxKind::LIMITS).next()?;
    let page_size = if let Some(token) = node
        .children_by_kind(SyntaxKind::MEM_PAGE_SIZE)
        .next()
        .and_then(|page_size| page_size.tokens_by_kind(SyntaxKind::UNSIGNED_INT).next())
        && let Ok(page_size) = helpers::parse_u32(token.text())
    {
        if page_size == 1 || page_size == 65536 {
            page_size
        } else {
            diagnostics.push(Diagnostic {
                range: token.text_range(),
                code: DIAGNOSTIC_CODE.into(),
                message: "memory page size must be 1 or 65536".into(),
                ..Default::default()
            });
            1
        }
    } else {
        1
    };
    let upper_bound = match types_analyzer::extract_addr_type(node.green()) {
        ValType::I32 => {
            if page_size == 1 {
                u32::MAX as u64
            } else {
                2u64.pow(32) / page_size as u64
            }
        }
        ValType::I64 => {
            if page_size == 1 {
                u64::MAX
            } else {
                (2u128.pow(64) / page_size as u128) as u64
            }
        }
        _ => return None,
    };

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
            upper_bound
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
                upper_bound
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
    } else if let Some(token) = node
        .tokens_by_kind(SyntaxKind::KEYWORD)
        .find(|token| token.text() == "shared")
    {
        diagnostics.push(Diagnostic {
            range: token.text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message: "shared memory must have a maximum size".into(),
            ..Default::default()
        });
    }
    Some(())
}

fn report_overflow(token: AmberToken, upper_bound: u64) -> Diagnostic {
    Diagnostic {
        range: token.text_range(),
        code: DIAGNOSTIC_CODE.into(),
        message: format!("memory size can't be greater than {upper_bound}"),
        ..Default::default()
    }
}
