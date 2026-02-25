use super::{Diagnostic, DiagnosticCtx, RelatedInformation};
use crate::{
    binder::{Symbol, SymbolKind},
    types_analyzer::{
        CompositeType, FieldType, HeapType, OperandType, RefType, ValType, get_func_sig, get_type_use_sig,
        resolve_br_types,
    },
};
use wat_syntax::{AmberNode, AmberToken, NodeOrToken, SyntaxKind, SyntaxNodePtr};

const DIAGNOSTIC_CODE: &str = "type-misuse";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    ctx: &DiagnosticCtx,
    node: AmberNode,
    instr_name: AmberToken,
) -> Option<()> {
    let mut immediates = node.children_by_kind(SyntaxKind::IMMEDIATE);
    match instr_name.text().split_once('.') {
        Some(("struct", _)) => {
            if let Some(diagnostic) = check_type_matches(ctx, "struct", immediates.next()?.to_ptr()) {
                diagnostics.push(diagnostic);
            }
        }
        Some(("array", "copy")) => {
            let dst = immediates.next()?.to_ptr();
            let dst_symbol = ctx.symbol_table.find_def(dst.into())?;
            let dst_type = match &ctx.def_types.get(&dst_symbol.key)?.comp {
                CompositeType::Func(..) => {
                    diagnostics.push(build_diagnostic("array", "func", dst, dst_symbol, ctx.db));
                    None
                }
                CompositeType::Struct(..) => {
                    diagnostics.push(build_diagnostic("array", "struct", dst, dst_symbol, ctx.db));
                    None
                }
                CompositeType::Array(field_type) => field_type.as_ref(),
            };

            let src = immediates.next()?.to_ptr();
            let src_symbol = ctx.symbol_table.find_def(src.into())?;
            let src_type = match &ctx.def_types.get(&src_symbol.key)?.comp {
                CompositeType::Func(..) => {
                    diagnostics.push(build_diagnostic("array", "func", src, src_symbol, ctx.db));
                    None
                }
                CompositeType::Struct(..) => {
                    diagnostics.push(build_diagnostic("array", "struct", src, src_symbol, ctx.db));
                    None
                }
                CompositeType::Array(field_type) => field_type.as_ref(),
            };

            match (dst_type, src_type) {
                (Some(FieldType { storage: dst, .. }), Some(FieldType { storage: src, .. }))
                    if !src.matches(dst, ctx.db, ctx.document, ctx.module_id) =>
                {
                    diagnostics.push(Diagnostic {
                        range: node.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!(
                            "destination array type `{}` doesn't match source array type `{}`",
                            dst_symbol.idx.render(ctx.db),
                            src_symbol.idx.render(ctx.db),
                        ),
                        related_information: Some(vec![
                            RelatedInformation {
                                range: dst_symbol.key.text_range(),
                                message: "destination array type defined here".into(),
                            },
                            RelatedInformation {
                                range: src_symbol.key.text_range(),
                                message: "source array type defined here".into(),
                            },
                        ]),
                        ..Default::default()
                    });
                }
                _ => {}
            }
        }
        Some(("array", _)) => {
            if let Some(diagnostic) = check_type_matches(ctx, "array", immediates.next()?.to_ptr()) {
                diagnostics.push(diagnostic);
            }
        }
        _ => match instr_name.text() {
            "call_ref" => {
                if let Some(diagnostic) = check_type_matches(ctx, "func", immediates.next()?.to_ptr()) {
                    diagnostics.push(diagnostic);
                }
            }
            "br_on_cast" => {
                let label = immediates.next()?;
                let rt_label_type = resolve_br_types(ctx.db, ctx.document, ctx.symbol_table, label.to_ptr().into())
                    .and_then(|mut types| types.next_back());
                let rt_label = if let Some(OperandType::Val(ValType::Ref(rt_label))) = rt_label_type {
                    rt_label
                } else {
                    diagnostics.push(Diagnostic {
                        range: label.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: "the last type of this label must be a ref type".into(),
                        ..Default::default()
                    });
                    return None;
                };
                let rt1_node = immediates.next()?;
                let rt1 = RefType::from_green(rt1_node.children_by_kind(SyntaxKind::REF_TYPE).next()?.green(), ctx.db)?;
                let rt2_node = immediates.next()?;
                let rt2 = RefType::from_green(rt2_node.children_by_kind(SyntaxKind::REF_TYPE).next()?.green(), ctx.db)?;
                if !rt2.matches(&rt1, ctx.db, ctx.document, ctx.module_id) {
                    diagnostics.push(Diagnostic {
                        range: rt2_node.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!(
                            "ref type `{}` doesn't match the ref type `{}`",
                            rt2.render(ctx.db),
                            rt1.render(ctx.db),
                        ),
                        related_information: Some(vec![RelatedInformation {
                            range: rt1_node.text_range(),
                            message: "should match this ref type".into(),
                        }]),
                        ..Default::default()
                    });
                }
                if !rt2.matches(&rt_label, ctx.db, ctx.document, ctx.module_id) {
                    diagnostics.push(Diagnostic {
                        range: rt2_node.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!(
                            "ref type `{}` doesn't match the ref type `{}`",
                            rt2.render(ctx.db),
                            rt_label.render(ctx.db),
                        ),
                        related_information: Some(vec![RelatedInformation {
                            range: label.text_range(),
                            message: "should match the last ref type in the result type of this label".into(),
                        }]),
                        ..Default::default()
                    });
                }
            }
            "br_on_cast_fail" => {
                let label = immediates.next()?;
                let rt_label_type = resolve_br_types(ctx.db, ctx.document, ctx.symbol_table, label.to_ptr().into())
                    .and_then(|mut types| types.next_back());
                let rt_label = if let Some(OperandType::Val(ValType::Ref(rt_label))) = rt_label_type {
                    rt_label
                } else {
                    diagnostics.push(Diagnostic {
                        range: label.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: "the last type of this label must be a ref type".into(),
                        ..Default::default()
                    });
                    return None;
                };
                let rt1_node = immediates.next()?;
                let rt1 = RefType::from_green(rt1_node.children_by_kind(SyntaxKind::REF_TYPE).next()?.green(), ctx.db)?;
                let rt2_node = immediates.next()?;
                let rt2 = RefType::from_green(rt2_node.children_by_kind(SyntaxKind::REF_TYPE).next()?.green(), ctx.db)?;
                if !rt2.matches(&rt1, ctx.db, ctx.document, ctx.module_id) {
                    diagnostics.push(Diagnostic {
                        range: rt2_node.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!(
                            "ref type `{}` doesn't match the ref type `{}`",
                            rt2.render(ctx.db),
                            rt1.render(ctx.db),
                        ),
                        related_information: Some(vec![RelatedInformation {
                            range: rt1_node.text_range(),
                            message: "should match this ref type".into(),
                        }]),
                        ..Default::default()
                    });
                }
                let rt_diff = rt1.diff(&rt2);
                if !rt_diff.matches(&rt_label, ctx.db, ctx.document, ctx.module_id) {
                    diagnostics.push(Diagnostic {
                        range: node.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!(
                            "type difference between given two ref types `{}` doesn't match the ref type `{}`",
                            rt_diff.render(ctx.db),
                            rt_label.render(ctx.db),
                        ),
                        related_information: Some(vec![RelatedInformation {
                            range: label.text_range(),
                            message: "should match the last ref type in the result type of this label".into(),
                        }]),
                        ..Default::default()
                    });
                }
            }
            "call_indirect" => {
                if let Some(diagnostic) = check_table_ref_type(ctx, node) {
                    diagnostics.push(diagnostic);
                }
            }
            "return_call" => {
                if let Some(immediate) = node.children_by_kind(SyntaxKind::IMMEDIATE).next()
                    && let Some(diagnostic) = ctx
                        .symbol_table
                        .find_def(immediate.to_ptr().into())
                        .map(|func| get_func_sig(ctx.db, ctx.document, func.key, &func.green))
                        .and_then(|sig| check_return_call_result_type(ctx, node, immediate, &sig.results))
                {
                    diagnostics.push(diagnostic);
                }
            }
            "return_call_ref" => {
                let immediate = immediates.next()?;
                if let Some(diagnostic) = check_type_matches(ctx, "func", immediate.to_ptr()) {
                    diagnostics.push(diagnostic);
                }
                if let Some(diagnostic) = ctx
                    .symbol_table
                    .resolved
                    .get(&immediate.to_ptr().into())
                    .and_then(|key| ctx.def_types.get(key))
                    .and_then(|def_type| def_type.comp.as_func())
                    .and_then(|sig| check_return_call_result_type(ctx, node, immediate, &sig.results))
                {
                    diagnostics.push(diagnostic);
                }
            }
            "return_call_indirect" => {
                if let Some(diagnostic) = check_table_ref_type(ctx, node) {
                    diagnostics.push(diagnostic);
                }
                if let Some(type_use) = node
                    .children()
                    .find_map(|immediate| immediate.children_by_kind(SyntaxKind::TYPE_USE).next())
                    && let Some(diagnostic) = check_return_call_result_type(
                        ctx,
                        node,
                        type_use,
                        &get_type_use_sig(ctx.db, ctx.document, type_use.to_ptr(), type_use.green()).results,
                    )
                {
                    diagnostics.push(diagnostic);
                }
            }
            "throw" => {
                if let Some(immediate) = immediates.next()
                    && let Some(symbol) = ctx.symbol_table.find_def(immediate.to_ptr().into())
                    && !get_func_sig(ctx.db, ctx.document, symbol.key, &symbol.green)
                        .results
                        .is_empty()
                {
                    diagnostics.push(Diagnostic {
                        range: immediate.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!(
                            "result types of exception tag `{}` must be empty",
                            symbol.idx.render(ctx.db)
                        ),
                        related_information: ctx.symbol_table.def_poi.get(&symbol.key).map(|range| {
                            vec![RelatedInformation {
                                range: *range,
                                message: format!("tag `{}` defined here", symbol.idx.render(ctx.db)),
                            }]
                        }),
                        ..Default::default()
                    });
                }
            }
            _ => {}
        },
    }
    Some(())
}

fn check_type_matches(
    ctx: &DiagnosticCtx,
    expected_kind: &'static str,
    immediate: SyntaxNodePtr,
) -> Option<Diagnostic> {
    let def_symbol = ctx.symbol_table.find_def(immediate.into())?;
    let def_type = ctx.def_types.get(&def_symbol.key)?;
    let kind = match def_type.comp {
        CompositeType::Func(..) => "func",
        CompositeType::Struct(..) => "struct",
        CompositeType::Array(..) => "array",
    };
    if kind == expected_kind {
        None
    } else {
        Some(build_diagnostic(expected_kind, kind, immediate, def_symbol, ctx.db))
    }
}

fn check_table_ref_type(ctx: &DiagnosticCtx, node: AmberNode) -> Option<Diagnostic> {
    if let Some(ref_key) = node
        .children_by_kind(SyntaxKind::IMMEDIATE)
        .find(|immediate| {
            immediate
                .tokens_by_kind([SyntaxKind::INT, SyntaxKind::UNSIGNED_INT, SyntaxKind::IDENT])
                .next()
                .is_some()
        })
        .map(|immediate| immediate.to_ptr().into())
        && let Some(ref_symbol) = ctx.symbol_table.symbols.get(&ref_key)
        && ctx
            .symbol_table
            .find_def(ref_key)
            .and_then(|symbol| {
                symbol.green.children().find_map(|child| {
                    if let NodeOrToken::Node(node) = child {
                        match node.kind() {
                            SyntaxKind::REF_TYPE => Some(node),
                            SyntaxKind::TABLE_TYPE => node.children().find_map(|child| {
                                if let NodeOrToken::Node(node) = child
                                    && node.kind() == SyntaxKind::REF_TYPE
                                {
                                    Some(node)
                                } else {
                                    None
                                }
                            }),
                            _ => None,
                        }
                    } else {
                        None
                    }
                })
            })
            .and_then(|green| RefType::from_green(green, ctx.db))
            .is_some_and(|ty| {
                !ty.matches(
                    &RefType {
                        heap_ty: HeapType::Func,
                        nullable: true,
                    },
                    ctx.db,
                    ctx.document,
                    ctx.module_id,
                )
            })
    {
        Some(Diagnostic {
            range: ref_key.text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message: format!(
                "ref type of table `{}` must match `(ref null func)`",
                ref_symbol.idx.render(ctx.db),
            ),
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_return_call_result_type(
    ctx: &DiagnosticCtx,
    instr: AmberNode,
    reported_node: AmberNode,
    actual: &[ValType],
) -> Option<Diagnostic> {
    let func =
        ctx.symbol_table.symbols.values().find(|symbol| {
            symbol.kind == SymbolKind::Func && symbol.key.text_range().contains_range(instr.text_range())
        })?;
    let expected = get_func_sig(ctx.db, ctx.document, func.key, &func.green).results;
    if actual.len() == expected.len()
        && actual
            .iter()
            .zip(expected.iter())
            .all(|(actual, expected)| actual.matches(expected, ctx.db, ctx.document, ctx.module_id))
    {
        None
    } else {
        Some(Diagnostic {
            range: reported_node.text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message: "this result type must match the result type of current function".into(),
            ..Default::default()
        })
    }
}

fn build_diagnostic(
    expected_kind: &'static str,
    actual_kind: &str,
    ptr: SyntaxNodePtr,
    def_symbol: &Symbol,
    db: &dyn salsa::Database,
) -> Diagnostic {
    debug_assert!(matches!(expected_kind, "func" | "struct" | "array"));
    debug_assert!(matches!(actual_kind, "func" | "struct" | "array"));
    Diagnostic {
        range: ptr.text_range(),
        code: DIAGNOSTIC_CODE.into(),
        message: format!(
            "expected type is {expected_kind}, but type of `{}` is {actual_kind}",
            def_symbol.idx.render(db)
        ),
        related_information: Some(vec![RelatedInformation {
            range: def_symbol.key.text_range(),
            message: format!("{actual_kind} type defined here"),
        }]),
        ..Default::default()
    }
}
