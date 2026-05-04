use super::Diagnostic;
use crate::helpers;
use std::num::IntErrorKind;
use wat_syntax::{AmberNode, AmberToken, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "lane";

pub fn check(diagnostics: &mut Vec<Diagnostic>, node: AmberNode, instr_name: AmberToken) -> Option<()> {
    match instr_name.text().split_once('.')? {
        ("v128", right) => {
            let n = right
                .strip_prefix("load")
                .or_else(|| right.strip_prefix("store"))?
                .strip_suffix("_lane")?
                .parse::<u32>()
                .ok()?;
            let max = 128 / n;
            let mut immediates = node.children_by_kind(SyntaxKind::IMMEDIATE);
            let second = immediates.nth(1);
            if let Some(diagnostic) = immediates
                .next()
                .or(second)
                .and_then(|immediate| check_immediate(immediate, max))
            {
                diagnostics.push(diagnostic);
            }
        }
        ("i8x16", "extract_lane_s" | "extract_lane_u" | "replace_lane") => {
            if let Some(diagnostic) = node
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| check_immediate(immediate, 16))
            {
                diagnostics.push(diagnostic);
            }
        }
        ("i16x8", "extract_lane_s" | "extract_lane_u" | "replace_lane") => {
            if let Some(diagnostic) = node
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| check_immediate(immediate, 8))
            {
                diagnostics.push(diagnostic);
            }
        }
        ("i32x4" | "f32x4", "extract_lane" | "replace_lane") => {
            if let Some(diagnostic) = node
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| check_immediate(immediate, 4))
            {
                diagnostics.push(diagnostic);
            }
        }
        ("i64x2" | "f64x2", "extract_lane" | "replace_lane") => {
            if let Some(diagnostic) = node
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| check_immediate(immediate, 2))
            {
                diagnostics.push(diagnostic);
            }
        }
        ("i8x16", "shuffle") => {
            diagnostics.extend(
                node.children_by_kind(SyntaxKind::IMMEDIATE)
                    .filter_map(|immediate| check_immediate(immediate, 32)),
            );
        }
        _ => {}
    }
    None
}

fn check_immediate(immediate: AmberNode, max: u32) -> Option<Diagnostic> {
    match helpers::parse_u32(immediate.tokens_by_kind(SyntaxKind::INT).next()?.text()) {
        Ok(laneidx) => {
            if laneidx < max {
                None
            } else {
                Some(Diagnostic {
                    range: immediate.text_range(),
                    code: DIAGNOSTIC_CODE.into(),
                    message: format!("lane index must be less than {max}"),
                    ..Default::default()
                })
            }
        }
        Err(error) => Some(Diagnostic {
            range: immediate.text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message: if error.kind() == &IntErrorKind::PosOverflow {
                format!("lane index must be less than {max}")
            } else {
                "invalid lane index".into()
            },
            ..Default::default()
        }),
    }
}
