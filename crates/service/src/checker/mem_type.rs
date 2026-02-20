use super::Diagnostic;
use crate::helpers;
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
    let upper_bound = if page_size == 1 {
        u32::MAX
    } else {
        (2u64.pow(32) / page_size as u64) as u32
    };

    let mut uints = limits.tokens_by_kind(SyntaxKind::UNSIGNED_INT);
    let min_token = uints.next()?;
    let min = match helpers::parse_u32(min_token.text()) {
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
        let max = match helpers::parse_u32(max_token.text()) {
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

fn report_overflow(token: AmberToken, upper_bound: u32) -> Diagnostic {
    Diagnostic {
        range: token.text_range(),
        code: DIAGNOSTIC_CODE.into(),
        message: format!("memory size can't be greater than {upper_bound}"),
        ..Default::default()
    }
}
