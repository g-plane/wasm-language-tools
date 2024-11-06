use crate::helpers;
use line_index::LineIndex;
use lsp_types::{Diagnostic, DiagnosticSeverity};
use rowan::ast::{
    support::{children, token},
    AstNode,
};
use wat_syntax::{ast::Operand, SyntaxKind, SyntaxNode};

pub fn check(diags: &mut Vec<Diagnostic>, line_index: &LineIndex, node: &SyntaxNode) {
    let Some(instr_name) = token(node, SyntaxKind::INSTR_NAME) else {
        return;
    };
    let mut operands = children::<Operand>(node);

    macro_rules! check_operand {
        ($kind:pat, $msg:literal) => {
            if let Some(operand) = operands
                .next()
                .and_then(|operand| operand.syntax().first_token())
            {
                if !matches!(operand.kind(), $kind) {
                    diags.push(Diagnostic {
                        range: helpers::rowan_range_to_lsp_range(line_index, operand.text_range()),
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("wat".into()),
                        message: $msg.into(),
                        ..Default::default()
                    });
                }
            }
        };
    }

    match instr_name.text() {
        "call" | "local.get" | "local.set" | "local.tee" | "global.get" | "global.set"
        | "table.get" | "table.set" | "ref.func" | "memory.init" | "data.drop" | "elem.drop"
        | "table.grow" | "table.size" | "table.fill" => {
            check_operand!(
                SyntaxKind::IDENT | SyntaxKind::INT,
                "expected identifier or unsigned integer"
            );
        }
        "i32.const" | "i64.const" | "v128.const" => {
            check_operand!(SyntaxKind::INT, "expected integer");
        }
        "f32.const" | "f64.const" => {
            check_operand!(SyntaxKind::FLOAT, "expected floating-point number");
        }
        "table.init" | "table.copy" => {
            check_operand!(
                SyntaxKind::IDENT | SyntaxKind::INT,
                "expected identifier or unsigned integer"
            );
            check_operand!(
                SyntaxKind::IDENT | SyntaxKind::INT,
                "expected identifier or unsigned integer"
            );
        }
        "i32.load" | "i64.load" | "f32.load" | "f64.load" | "i32.load8_s" | "i32.load8_u"
        | "i32.load16_s" | "i32.load16_u" | "i64.load8_s" | "i64.load8_u" | "i64.load16_s"
        | "i64.load16_u" | "i64.load32_s" | "i64.load32_u" | "i32.store" | "i64.store"
        | "f32.store" | "f64.store" | "i32.store8" | "i32.store16" | "i64.store8"
        | "i64.store16" | "i64.store32" | "v128.load" | "v128.load8x8_s" | "v128.load8x8_u"
        | "v128.load16x4_s" | "v128.load16x4_u" | "v128.load32x2_s" | "v128.load32x2_u"
        | "v128.load8_splat" | "v128.load16_splat" | "v128.load32_splat" | "v128.load64_splat"
        | "v128.store" => {
            check_operand!(SyntaxKind::MEM_ARG, "expected memory argument");
        }
        "i8x16.shuffle"
        | "i8x16.extract_lane_s"
        | "i8x16.extract_lane_u"
        | "i8x16.replace_lane"
        | "i16x8.extract_lane_s"
        | "i16x8.extract_lane_u"
        | "i16x8.replace_lane"
        | "i32x4.extract_lane"
        | "i32x4.replace_lane"
        | "i64x2.extract_lane"
        | "i64x2.replace_lane"
        | "f32x4.extract_lane"
        | "f32x4.replace_lane"
        | "f64x2.extract_lane"
        | "f64x2.replace_lane" => {
            check_operand!(
                SyntaxKind::IDENT | SyntaxKind::INT,
                "expected identifier or unsigned integer"
            );
        }
        "v128.load8_lane" | "v128.load16_lane" | "v128.load32_lane" | "v128.load64_lane"
        | "v128.store8_lane" | "v128.store16_lane" | "v128.store32_lane" | "v128.store64_lane" => {
            check_operand!(SyntaxKind::MEM_ARG, "expected memory argument");
            check_operand!(
                SyntaxKind::IDENT | SyntaxKind::INT,
                "expected identifier or unsigned integer"
            );
        }
        _ => {}
    }
}
