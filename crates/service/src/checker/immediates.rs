use super::{Diagnostic, FastPlainInstr};
use std::iter::Peekable;
use wat_syntax::{AmberNode, NodeOrToken, SyntaxKind, SyntaxKindMatch};

const DIAGNOSTIC_CODE: &str = "immediates";

const INDEX: [SyntaxKind; 2] = [SyntaxKind::IDENT, SyntaxKind::INT];

pub fn check(diagnostics: &mut Vec<Diagnostic>, node: AmberNode, instr: &FastPlainInstr) {
    let mut immediates = node
        .children()
        .filter(|child| child.kind() == SyntaxKind::IMMEDIATE)
        .peekable();

    match instr.name.text() {
        "call" | "local.get" | "local.set" | "local.tee" | "global.get" | "global.set" | "ref.func" | "data.drop"
        | "elem.drop" | "br" | "br_if" | "struct.new" | "struct.new_default" | "array.new" | "array.new_default"
        | "array.get" | "array.get_u" | "array.get_s" | "array.set" | "array.fill" | "br_on_null"
        | "br_on_non_null" | "call_ref" | "return_call" | "return_call_ref" | "throw" => {
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                instr,
            );
        }
        "i32.const" | "i64.const" => {
            check_immediate::<true>(diagnostics, &mut immediates, SyntaxKind::INT, "integer", instr);
        }
        "f32.const" | "f64.const" => {
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                [SyntaxKind::FLOAT, SyntaxKind::INT],
                "floating-point number",
                instr,
            );
        }
        "select" => {
            if let Some(node) = immediates.next() {
                'a: {
                    let Some(type_use) = node.green().children().find_map(|node_or_token| match node_or_token {
                        NodeOrToken::Node(node) if node.kind() == SyntaxKind::TYPE_USE => Some(node),
                        _ => None,
                    }) else {
                        diagnostics.push(Diagnostic {
                            range: node.text_range(),
                            code: DIAGNOSTIC_CODE.into(),
                            message: "expected result type".into(),
                            ..Default::default()
                        });
                        break 'a;
                    };
                    let mut children = type_use.children().filter_map(NodeOrToken::into_node);
                    if children.next().is_some_and(|child| {
                        child.kind() == SyntaxKind::RESULT
                            && child.children().filter_map(NodeOrToken::into_node).count() == 1
                    }) && children.next().is_none()
                    {
                        break 'a;
                    }
                    diagnostics.push(Diagnostic {
                        range: node.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
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
                instr,
            );
            diagnostics.extend(
                immediates
                    .filter(|immediate| {
                        !immediate.green().children().next().is_some_and(|node_or_token| {
                            matches!(node_or_token.kind(), SyntaxKind::IDENT | SyntaxKind::INT)
                        })
                    })
                    .map(|immediate| Diagnostic {
                        range: immediate.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
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
                instr,
            );
            check_immediate::<false>(diagnostics, &mut immediates, SyntaxKind::TYPE_USE, "type use", instr);
        }
        "struct.get" | "struct.get_u" | "struct.get_s" | "struct.set" | "array.new_data" | "array.new_elem"
        | "array.copy" | "array.init_data" | "array.init_elem" => {
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                instr,
            );
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                instr,
            );
        }
        "i32.load" | "i64.load" | "f32.load" | "f64.load" | "i32.load8_s" | "i32.load8_u" | "i32.load16_s"
        | "i32.load16_u" | "i64.load8_s" | "i64.load8_u" | "i64.load16_s" | "i64.load16_u" | "i64.load32_s"
        | "i64.load32_u" | "i32.store" | "i64.store" | "f32.store" | "f64.store" | "i32.store8" | "i32.store16"
        | "i64.store8" | "i64.store16" | "i64.store32" | "v128.load" | "v128.load8x8_s" | "v128.load8x8_u"
        | "v128.load16x4_s" | "v128.load16x4_u" | "v128.load32x2_s" | "v128.load32x2_u" | "v128.load8_splat"
        | "v128.load16_splat" | "v128.load32_splat" | "v128.load64_splat" | "v128.load32_zero" | "v128.load64_zero"
        | "v128.store" => {
            check_immediate::<false>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                instr,
            );
            check_immediate::<false>(
                diagnostics,
                &mut immediates,
                SyntaxKind::MEM_ARG,
                "memory argument",
                instr,
            );
        }
        "memory.size" | "memory.grow" | "memory.fill" | "table.get" | "table.set" | "table.grow" | "table.size"
        | "table.fill" => {
            check_immediate::<false>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                instr,
            );
        }
        "memory.copy" | "table.copy" => {
            check_immediate::<false>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                instr,
            );
            check_immediate::<false>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                instr,
            );
        }
        "memory.init" | "table.init" => {
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                instr,
            );
            check_immediate::<false>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                instr,
            );
        }
        "v128.const" => {
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                SyntaxKind::SHAPE_DESCRIPTOR,
                "shape descriptor",
                instr,
            );
            if let Some((allow_float, expected_count)) = node
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| {
                    immediate
                        .children_with_tokens()
                        .next()
                        .and_then(NodeOrToken::into_token)
                })
                .filter(|token| token.kind() == SyntaxKind::SHAPE_DESCRIPTOR)
                .as_ref()
                .and_then(|token| token.text().split_once('x'))
                .and_then(|(ty, count)| count.parse::<usize>().ok().map(|count| (ty.starts_with('f'), count)))
            {
                let actual_count = immediates.clone().count();
                if actual_count != expected_count {
                    diagnostics.push(Diagnostic {
                        range: node.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!(
                            "expected {expected_count} {} in `v128.const`",
                            if allow_float {
                                "floating-point numbers"
                            } else {
                                "integers"
                            },
                        ),
                        ..Default::default()
                    });
                }
                if allow_float {
                    for _ in 0..actual_count {
                        check_immediate::<true>(
                            diagnostics,
                            &mut immediates,
                            [SyntaxKind::FLOAT, SyntaxKind::INT],
                            "floating-point number",
                            instr,
                        );
                    }
                } else {
                    for _ in 0..actual_count {
                        check_immediate::<true>(diagnostics, &mut immediates, SyntaxKind::INT, "integer", instr);
                    }
                }
            }
        }
        "i8x16.extract_lane_s"
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
                instr,
            );
        }
        "i8x16.shuffle" => {
            let immediates_count = immediates.clone().count();
            if immediates_count != 16 {
                diagnostics.push(Diagnostic {
                    range: node.text_range(),
                    code: DIAGNOSTIC_CODE.into(),
                    message: format!("expected 16 lane indices in `i8x16.shuffle`, found {immediates_count}"),
                    ..Default::default()
                });
            }
            immediates.for_each(|immediate| {
                if immediate
                    .green()
                    .children()
                    .next()
                    .and_then(NodeOrToken::into_token)
                    .and_then(|token| token.text().parse::<u8>().ok())
                    .is_some_and(|idx| idx >= 32)
                {
                    diagnostics.push(Diagnostic {
                        range: immediate.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: "laneidx must be smaller than 32".into(),
                        ..Default::default()
                    });
                }
            });
            return;
        }
        "v128.load8_lane" | "v128.load16_lane" | "v128.load32_lane" | "v128.load64_lane" | "v128.store8_lane"
        | "v128.store16_lane" | "v128.store32_lane" | "v128.store64_lane" => {
            check_immediate::<false>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                instr,
            );
            check_immediate::<false>(
                diagnostics,
                &mut immediates,
                SyntaxKind::MEM_ARG,
                "memory argument",
                instr,
            );
            check_immediate::<false>(diagnostics, &mut immediates, SyntaxKind::INT, "unsigned integer", instr);
        }
        "ref.null" => {
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                [SyntaxKind::HEAP_TYPE, SyntaxKind::IDENT, SyntaxKind::INT],
                "heap type",
                instr,
            );
        }
        "ref.test" | "ref.cast" => {
            check_immediate::<true>(diagnostics, &mut immediates, SyntaxKind::REF_TYPE, "ref type", instr);
        }
        "array.new_fixed" => {
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                instr,
            );
            check_immediate::<true>(diagnostics, &mut immediates, SyntaxKind::INT, "unsigned integer", instr);
        }
        "br_on_cast" | "br_on_cast_fail" => {
            check_immediate::<true>(
                diagnostics,
                &mut immediates,
                INDEX,
                "identifier or unsigned integer",
                instr,
            );
            check_immediate::<true>(diagnostics, &mut immediates, SyntaxKind::REF_TYPE, "ref type", instr);
            check_immediate::<true>(diagnostics, &mut immediates, SyntaxKind::REF_TYPE, "ref type", instr);
        }
        "memory.atomic.notify"
        | "memory.atomic.wait32"
        | "memory.atomic.wait64"
        | "i32.atomic.load"
        | "i64.atomic.load"
        | "i32.atomic.load8_u"
        | "i32.atomic.load16_u"
        | "i64.atomic.load8_u"
        | "i64.atomic.load16_u"
        | "i64.atomic.load32_u"
        | "i32.atomic.store"
        | "i64.atomic.store"
        | "i32.atomic.store8"
        | "i32.atomic.store16"
        | "i64.atomic.store8"
        | "i64.atomic.store16"
        | "i64.atomic.store32"
        | "i32.atomic.rmw.add"
        | "i64.atomic.rmw.add"
        | "i32.atomic.rmw8.add_u"
        | "i32.atomic.rmw16.add_u"
        | "i64.atomic.rmw8.add_u"
        | "i64.atomic.rmw16.add_u"
        | "i64.atomic.rmw32.add_u"
        | "i32.atomic.rmw.sub"
        | "i64.atomic.rmw.sub"
        | "i32.atomic.rmw8.sub_u"
        | "i32.atomic.rmw16.sub_u"
        | "i64.atomic.rmw8.sub_u"
        | "i64.atomic.rmw16.sub_u"
        | "i64.atomic.rmw32.sub_u"
        | "i32.atomic.rmw.and"
        | "i64.atomic.rmw.and"
        | "i32.atomic.rmw8.and_u"
        | "i32.atomic.rmw16.and_u"
        | "i64.atomic.rmw8.and_u"
        | "i64.atomic.rmw16.and_u"
        | "i64.atomic.rmw32.and_u"
        | "i32.atomic.rmw.or"
        | "i64.atomic.rmw.or"
        | "i32.atomic.rmw8.or_u"
        | "i32.atomic.rmw16.or_u"
        | "i64.atomic.rmw8.or_u"
        | "i64.atomic.rmw16.or_u"
        | "i64.atomic.rmw32.or_u"
        | "i32.atomic.rmw.xor"
        | "i64.atomic.rmw.xor"
        | "i32.atomic.rmw8.xor_u"
        | "i32.atomic.rmw16.xor_u"
        | "i64.atomic.rmw8.xor_u"
        | "i64.atomic.rmw16.xor_u"
        | "i64.atomic.rmw32.xor_u"
        | "i32.atomic.rmw.xchg"
        | "i64.atomic.rmw.xchg"
        | "i32.atomic.rmw8.xchg_u"
        | "i32.atomic.rmw16.xchg_u"
        | "i64.atomic.rmw8.xchg_u"
        | "i64.atomic.rmw16.xchg_u"
        | "i64.atomic.rmw32.xchg_u"
        | "i32.atomic.rmw.cmpxchg"
        | "i64.atomic.rmw.cmpxchg"
        | "i32.atomic.rmw8.cmpxchg_u"
        | "i32.atomic.rmw16.cmpxchg_u"
        | "i64.atomic.rmw8.cmpxchg_u"
        | "i64.atomic.rmw16.cmpxchg_u"
        | "i64.atomic.rmw32.cmpxchg_u" => {
            check_immediate::<false>(
                diagnostics,
                &mut immediates,
                SyntaxKind::MEM_ARG,
                "memory argument",
                instr,
            );
        }
        _ => {}
    }
    diagnostics.extend(immediates.map(|immediate| Diagnostic {
        range: immediate.text_range(),
        code: DIAGNOSTIC_CODE.into(),
        message: "unexpected immediate".into(),
        ..Default::default()
    }));
}

fn check_immediate<'a, const REQUIRED: bool>(
    diagnostics: &mut Vec<Diagnostic>,
    immediates: &mut Peekable<impl Iterator<Item = AmberNode<'a>>>,
    expected: impl SyntaxKindMatch,
    description: &'static str,
    instr: &FastPlainInstr,
) {
    let immediate = immediates.peek().and_then(|immediate| {
        immediate
            .green()
            .children()
            .next()
            .map(|node_or_token| (node_or_token.kind(), immediate.text_range()))
    });
    if let Some((kind, range)) = immediate {
        if expected.matches(kind) {
            immediates.next();
        } else if REQUIRED {
            diagnostics.push(Diagnostic {
                range,
                code: DIAGNOSTIC_CODE.into(),
                message: format!("expected {description}"),
                ..Default::default()
            });
            immediates.next();
        }
    } else if REQUIRED {
        diagnostics.push(Diagnostic {
            range: instr.name.text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message: format!("missing {description}"),
            ..Default::default()
        });
    }
}
