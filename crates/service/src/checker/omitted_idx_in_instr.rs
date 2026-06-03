use super::Diagnostic;
use crate::config::LintLevel;
use lspt::DiagnosticSeverity;
use wat_syntax::{AmberNode, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "omitted-idx-in-instr";

pub fn check(lint_level: LintLevel, node: AmberNode) -> Option<Diagnostic> {
    let severity = match lint_level {
        LintLevel::Allow => return None,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };
    let instr_name_token = node.tokens_by_kind(SyntaxKind::INSTR_NAME).next()?;
    let instr_name = instr_name_token.text();
    match instr_name {
        "memory.size" | "memory.grow" | "memory.fill" | "i32.load" | "i64.load" | "f32.load" | "f64.load"
        | "i32.load8_s" | "i32.load8_u" | "i32.load16_s" | "i32.load16_u" | "i64.load8_s" | "i64.load8_u"
        | "i64.load16_s" | "i64.load16_u" | "i64.load32_s" | "i64.load32_u" | "i32.store" | "i64.store"
        | "f32.store" | "f64.store" | "i32.store8" | "i32.store16" | "i64.store8" | "i64.store16" | "i64.store32"
        | "v128.load" | "v128.load8x8_s" | "v128.load8x8_u" | "v128.load16x4_s" | "v128.load16x4_u"
        | "v128.load32x2_s" | "v128.load32x2_u" | "v128.load8_splat" | "v128.load16_splat" | "v128.load32_splat"
        | "v128.load64_splat" | "v128.load32_zero" | "v128.load64_zero" | "v128.store" | "v128.load8_lane"
        | "v128.load16_lane" | "v128.load32_lane" | "v128.load64_lane" | "v128.store8_lane" | "v128.store16_lane"
        | "v128.store32_lane" | "v128.store64_lane" => {
            if node
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .is_none_or(|immediate| immediate.children_by_kind(SyntaxKind::MEM_ARG).next().is_some())
            {
                Some(Diagnostic {
                    range: instr_name_token.text_range(),
                    code: DIAGNOSTIC_CODE.into(),
                    message: format!("explicit memory idx for `{instr_name}` is required"),
                    severity,
                    ..Default::default()
                })
            } else {
                None
            }
        }
        "memory.init" | "memory.copy" => {
            if node.children_by_kind(SyntaxKind::IMMEDIATE).count() < 2 {
                Some(Diagnostic {
                    range: instr_name_token.text_range(),
                    code: DIAGNOSTIC_CODE.into(),
                    message: format!("explicit memory idx for `{instr_name}` is required"),
                    severity,
                    ..Default::default()
                })
            } else {
                None
            }
        }
        "table.get" | "table.set" | "table.size" | "table.grow" | "table.fill" => {
            if node.children_by_kind(SyntaxKind::IMMEDIATE).next().is_none() {
                Some(Diagnostic {
                    range: instr_name_token.text_range(),
                    code: DIAGNOSTIC_CODE.into(),
                    message: format!("explicit table idx for `{instr_name}` is required"),
                    severity,
                    ..Default::default()
                })
            } else {
                None
            }
        }
        "table.init" | "table.copy" => {
            if node.children_by_kind(SyntaxKind::IMMEDIATE).count() < 2 {
                Some(Diagnostic {
                    range: instr_name_token.text_range(),
                    code: DIAGNOSTIC_CODE.into(),
                    message: format!("explicit table idx for `{instr_name}` is required"),
                    severity,
                    ..Default::default()
                })
            } else {
                None
            }
        }
        _ => None,
    }
}
