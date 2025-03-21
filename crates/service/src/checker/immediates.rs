use crate::helpers;
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use rowan::ast::{support::token, AstNode};
use std::iter::Peekable;
use wat_syntax::{ast::Immediate, SyntaxKind, SyntaxNode, SyntaxToken};

const DIAGNOSTIC_CODE: &str = "immediates";

const INDEX: [SyntaxKind; 2] = [SyntaxKind::IDENT, SyntaxKind::INT];

pub fn check(diagnostics: &mut Vec<Diagnostic>, line_index: &LineIndex, node: &SyntaxNode) {
    let Some(instr_name) = token(node, SyntaxKind::INSTR_NAME) else {
        return;
    };
    let mut immediates = node
        .children()
        .filter(|child| child.kind() == SyntaxKind::IMMEDIATE)
        .peekable();

    match instr_name.text() {
        "call" | "local.get" | "local.set" | "local.tee" | "global.get" | "global.set"
        | "table.get" | "table.set" | "ref.func" | "data.drop" | "elem.drop" | "table.grow"
        | "table.size" | "table.fill" | "br" | "br_if" | "struct.new" | "struct.new_default"
        | "array.new" | "array.new_default" | "array.get" | "array.get_u" | "array.get_s"
        | "array.set" | "array.fill" | "br_on_null" | "br_on_non_null" | "call_ref"
        | "return_call" | "return_call_ref" => {
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                &instr_name,
                line_index,
            );
        }
        "i32.const" | "i64.const" | "v128.const" => {
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                SyntaxKind::INT,
                "integer",
                &instr_name,
                line_index,
            );
        }
        "f32.const" | "f64.const" => {
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                [SyntaxKind::FLOAT, SyntaxKind::INT],
                "floating-point number",
                &instr_name,
                line_index,
            );
        }
        "select" => {
            if let Some(immediate) = immediates.next() {
                let range = immediate.text_range();
                'a: {
                    let Some(type_use) =
                        Immediate::cast(immediate).and_then(|immediate| immediate.type_use())
                    else {
                        diagnostics.push(Diagnostic {
                            range: helpers::rowan_range_to_lsp_range(line_index, range),
                            severity: Some(DiagnosticSeverity::Error),
                            source: Some("wat".into()),
                            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
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
                    diagnostics.push(Diagnostic {
                        range: helpers::rowan_range_to_lsp_range(line_index, range),
                        severity: Some(DiagnosticSeverity::Error),
                        source: Some("wat".into()),
                        code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                        message: "there must be exactly one result type".into(),
                        ..Default::default()
                    });
                }
            }
        }
        "br_table" => {
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                &instr_name,
                line_index,
            );
            diagnostics.extend(
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
                        severity: Some(DiagnosticSeverity::Error),
                        source: Some("wat".into()),
                        code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                        message: "expected identifier or unsigned integer".into(),
                        ..Default::default()
                    }),
            );
            return;
        }
        "call_indirect" | "return_call_indirect" => {
            check_immediate::<false>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                &instr_name,
                line_index,
            );
            check_immediate::<false>(
                diagnostics,
                &mut immediates,
                SyntaxKind::TYPE_USE,
                "type use",
                &instr_name,
                line_index,
            );
        }
        "table.init" | "table.copy" | "struct.get" | "struct.get_u" | "struct.get_s"
        | "struct.set" | "array.new_data" | "array.new_elem" | "array.copy" | "array.init_data"
        | "array.init_elem" => {
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                &instr_name,
                line_index,
            );
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                &instr_name,
                line_index,
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
            check_immediate::<false>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                &instr_name,
                line_index,
            );
            check_immediate::<false>(
                diagnostics,
                &mut immediates,
                SyntaxKind::MEM_ARG,
                "memory argument",
                &instr_name,
                line_index,
            );
        }
        "memory.size" | "memory.grow" | "memory.fill" => {
            check_immediate::<false>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                &instr_name,
                line_index,
            );
        }
        "memory.copy" => {
            check_immediate::<false>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                &instr_name,
                line_index,
            );
            check_immediate::<false>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                &instr_name,
                line_index,
            );
        }
        "memory.init" => {
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                &instr_name,
                line_index,
            );
            check_immediate::<false>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                &instr_name,
                line_index,
            );
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
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                &instr_name,
                line_index,
            );
        }
        "v128.load8_lane" | "v128.load16_lane" | "v128.load32_lane" | "v128.load64_lane"
        | "v128.store8_lane" | "v128.store16_lane" | "v128.store32_lane" | "v128.store64_lane" => {
            check_immediate::<false>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                &instr_name,
                line_index,
            );
            check_immediate::<false>(
                diagnostics,
                &mut immediates,
                SyntaxKind::MEM_ARG,
                "memory argument",
                &instr_name,
                line_index,
            );
            check_immediate::<false>(
                diagnostics,
                &mut immediates,
                SyntaxKind::INT,
                "unsigned integer",
                &instr_name,
                line_index,
            );
        }
        "ref.null" => {
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                [SyntaxKind::HEAP_TYPE, SyntaxKind::IDENT, SyntaxKind::INT],
                "heap type",
                &instr_name,
                line_index,
            );
        }
        "ref.test" | "ref.cast" => {
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                SyntaxKind::REF_TYPE,
                "ref type",
                &instr_name,
                line_index,
            );
        }
        "array.new_fixed" => {
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                &instr_name,
                line_index,
            );
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                SyntaxKind::INT,
                "unsigned integer",
                &instr_name,
                line_index,
            );
        }
        "br_on_cast" | "br_on_cast_fail" => {
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                &instr_name,
                line_index,
            );
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                SyntaxKind::REF_TYPE,
                "ref type",
                &instr_name,
                line_index,
            );
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                SyntaxKind::REF_TYPE,
                "ref type",
                &instr_name,
                line_index,
            );
        }
        _ => {}
    }
    diagnostics.extend(immediates.map(|immediate| Diagnostic {
        range: helpers::rowan_range_to_lsp_range(line_index, immediate.text_range()),
        severity: Some(DiagnosticSeverity::Error),
        source: Some("wat".into()),
        code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
        message: "unexpected immediate".into(),
        ..Default::default()
    }));
}

fn check_immediate<const REQUIRED: bool>(
    diagnostics: &mut Vec<Diagnostic>,
    immediates: &mut Peekable<impl Iterator<Item = SyntaxNode>>,
    expected: impl SyntaxKindCmp,
    description: &'static str,
    instr_name: &SyntaxToken,
    line_index: &LineIndex,
) {
    let immediate = immediates
        .peek()
        .and_then(|immediate| immediate.first_child_or_token());
    if let Some(immediate) = immediate {
        if expected.cmp(immediate.kind()) {
            immediates.next();
        } else if REQUIRED {
            diagnostics.push(Diagnostic {
                range: helpers::rowan_range_to_lsp_range(line_index, immediate.text_range()),
                severity: Some(DiagnosticSeverity::Error),
                source: Some("wat".into()),
                code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                message: format!("expected {description}"),
                ..Default::default()
            });
            immediates.next();
        }
    } else if REQUIRED {
        diagnostics.push(Diagnostic {
            range: helpers::rowan_range_to_lsp_range(line_index, instr_name.text_range()),
            severity: Some(DiagnosticSeverity::Error),
            source: Some("wat".into()),
            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
            message: format!("missing {description}"),
            ..Default::default()
        });
    }
}

trait SyntaxKindCmp {
    fn cmp(self, other: SyntaxKind) -> bool;
}
impl SyntaxKindCmp for SyntaxKind {
    fn cmp(self, other: SyntaxKind) -> bool {
        self == other
    }
}
impl<const N: usize> SyntaxKindCmp for [SyntaxKind; N] {
    fn cmp(self, other: SyntaxKind) -> bool {
        self.into_iter().any(|kind| kind == other)
    }
}
