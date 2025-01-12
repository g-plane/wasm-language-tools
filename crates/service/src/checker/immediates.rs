use crate::helpers;
use line_index::LineIndex;
use lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString};
use rowan::ast::{support::token, AstNode};
use wat_syntax::{ast::Immediate, SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "immediates";

pub fn check(diags: &mut Vec<Diagnostic>, line_index: &LineIndex, node: &SyntaxNode) {
    let Some(instr_name) = token(node, SyntaxKind::INSTR_NAME) else {
        return;
    };
    let mut immediates = node
        .children()
        .filter(|child| child.kind() == SyntaxKind::IMMEDIATE);

    macro_rules! check_immediate {
        ($kind:pat, $syntax:literal, $required:literal) => {
            let immediate = immediates
                .next()
                .and_then(|immediate| immediate.first_child_or_token());
            if let Some(immediate) = immediate {
                if !matches!(immediate.kind(), $kind) {
                    diags.push(Diagnostic {
                        range: helpers::rowan_range_to_lsp_range(
                            line_index,
                            immediate.text_range(),
                        ),
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("wat".into()),
                        code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
                        message: format!("expected {}", $syntax),
                        ..Default::default()
                    });
                }
            } else if $required {
                diags.push(Diagnostic {
                    range: helpers::rowan_range_to_lsp_range(line_index, instr_name.text_range()),
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("wat".into()),
                    code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
                    message: format!("missing {}", $syntax),
                    ..Default::default()
                });
            }
        };
    }

    match instr_name.text() {
        "call" | "local.get" | "local.set" | "local.tee" | "global.get" | "global.set"
        | "table.get" | "table.set" | "ref.func" | "memory.init" | "data.drop" | "elem.drop"
        | "table.grow" | "table.size" | "table.fill" | "br" | "br_if" => {
            check_immediate!(
                SyntaxKind::IDENT | SyntaxKind::INT,
                "identifier or unsigned integer",
                true
            );
        }
        "i32.const" | "i64.const" | "v128.const" => {
            check_immediate!(SyntaxKind::INT, "integer", true);
        }
        "f32.const" | "f64.const" => {
            check_immediate!(
                SyntaxKind::FLOAT | SyntaxKind::INT,
                "floating-point number",
                true
            );
        }
        "select" => {
            if let Some(immediate) = immediates.next() {
                let range = immediate.text_range();
                'a: {
                    let Some(type_use) =
                        Immediate::cast(immediate).and_then(|immediate| immediate.type_use())
                    else {
                        diags.push(Diagnostic {
                            range: helpers::rowan_range_to_lsp_range(line_index, range),
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("wat".into()),
                            code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
                            message: "expected result type".into(),
                            ..Default::default()
                        });
                        break 'a;
                    };
                    let mut children = type_use.syntax().children();
                    if children.next().is_some_and(|child| {
                        child.kind() == SyntaxKind::RESULT && child.children().count() == 1
                    }) && children.next().is_none()
                    {
                        break 'a;
                    }
                    diags.push(Diagnostic {
                        range: helpers::rowan_range_to_lsp_range(line_index, range),
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("wat".into()),
                        code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
                        message: "there must be exactly one result type".into(),
                        ..Default::default()
                    });
                }
            }
        }
        "br_table" => {
            diags.extend(
                immediates
                    .filter(|immediate| {
                        !immediate.first_token().is_some_and(|token| {
                            matches!(token.kind(), SyntaxKind::IDENT | SyntaxKind::INT)
                        })
                    })
                    .map(|immediate| Diagnostic {
                        range: helpers::rowan_range_to_lsp_range(
                            line_index,
                            immediate.text_range(),
                        ),
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("wat".into()),
                        code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
                        message: "expected identifier or unsigned integer".into(),
                        ..Default::default()
                    }),
            );
            return;
        }
        "table.init" | "table.copy" => {
            check_immediate!(
                SyntaxKind::IDENT | SyntaxKind::INT,
                "identifier or unsigned integer",
                true
            );
            check_immediate!(
                SyntaxKind::IDENT | SyntaxKind::INT,
                "identifier or unsigned integer",
                true
            );
        }
        "i32.load" | "i64.load" | "f32.load" | "f64.load" | "i32.load8_s" | "i32.load8_u"
        | "i32.load16_s" | "i32.load16_u" | "i64.load8_s" | "i64.load8_u" | "i64.load16_s"
        | "i64.load16_u" | "i64.load32_s" | "i64.load32_u" | "i32.store" | "i64.store"
        | "f32.store" | "f64.store" | "i32.store8" | "i32.store16" | "i64.store8"
        | "i64.store16" | "i64.store32" | "v128.load" | "v128.load8x8_s" | "v128.load8x8_u"
        | "v128.load16x4_s" | "v128.load16x4_u" | "v128.load32x2_s" | "v128.load32x2_u"
        | "v128.load8_splat" | "v128.load16_splat" | "v128.load32_splat" | "v128.load64_splat"
        | "v128.load32_zero" | "v128.load64_zero" | "v128.store" => {
            check_immediate!(SyntaxKind::MEM_ARG, "memory argument", true);
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
            check_immediate!(
                SyntaxKind::IDENT | SyntaxKind::INT,
                "identifier or unsigned integer",
                true
            );
        }
        "v128.load8_lane" | "v128.load16_lane" | "v128.load32_lane" | "v128.load64_lane"
        | "v128.store8_lane" | "v128.store16_lane" | "v128.store32_lane" | "v128.store64_lane" => {
            check_immediate!(SyntaxKind::MEM_ARG, "memory argument", true);
            check_immediate!(
                SyntaxKind::IDENT | SyntaxKind::INT,
                "identifier or unsigned integer",
                true
            );
        }
        "ref.null" => {
            check_immediate!(SyntaxKind::HEAP_TYPE, "heap type", true);
        }
        _ => {}
    }
    diags.extend(immediates.map(|immediate| Diagnostic {
        range: helpers::rowan_range_to_lsp_range(line_index, immediate.text_range()),
        severity: Some(DiagnosticSeverity::ERROR),
        source: Some("wat".into()),
        code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
        message: "unexpected immediate".into(),
        ..Default::default()
    }));
}
