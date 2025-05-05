use crate::helpers;
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use rowan::ast::support;
use std::num::{IntErrorKind, ParseIntError};
use wat_syntax::{ast::Limits, SyntaxNode, SyntaxToken};

const DIAGNOSTIC_CODE: &str = "mem-type";

const K: u32 = 2u32.pow(16);

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    line_index: &LineIndex,
    node: &SyntaxNode,
) -> Option<()> {
    let limits = support::child::<Limits>(node)?;
    let min_token = limits.min()?;
    let min = match parse_u32(min_token.text()) {
        Ok(min) => {
            if min > K {
                diagnostics.push(report_overflow(&min_token, line_index));
            }
            min
        }
        Err(error) if error.kind() == &IntErrorKind::PosOverflow => {
            diagnostics.push(report_overflow(&min_token, line_index));
            K
        }
        Err(_) => return None,
    };
    if let Some(max_token) = limits.max() {
        let max = match parse_u32(max_token.text()) {
            Ok(max) => {
                if max > K {
                    diagnostics.push(report_overflow(&max_token, line_index));
                }
                max
            }
            Err(error) if error.kind() == &IntErrorKind::PosOverflow => {
                diagnostics.push(report_overflow(&max_token, line_index));
                K
            }
            Err(_) => return None,
        };
        if max < min {
            diagnostics.push(Diagnostic {
                range: helpers::rowan_range_to_lsp_range(line_index, max_token.text_range()),
                severity: Some(DiagnosticSeverity::Error),
                source: Some("wat".into()),
                code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                message: "max value must be greater than min value".into(),
                ..Default::default()
            });
        }
    }
    Some(())
}

fn report_overflow(token: &SyntaxToken, line_index: &LineIndex) -> Diagnostic {
    Diagnostic {
        range: helpers::rowan_range_to_lsp_range(line_index, token.text_range()),
        severity: Some(DiagnosticSeverity::Error),
        source: Some("wat".into()),
        code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
        message: format!("value can't be greater than {K}"),
        ..Default::default()
    }
}

fn parse_u32(s: &str) -> Result<u32, ParseIntError> {
    let s = s.replace('_', "");
    if let Some(s) = s.strip_prefix("0x") {
        u32::from_str_radix(s, 16)
    } else {
        s.parse()
    }
}
