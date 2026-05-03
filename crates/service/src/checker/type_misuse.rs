use super::{Diagnostic, DiagnosticCtx, RelatedInformation};
use crate::{
    binder::{Symbol, SymbolKey, SymbolKind},
    helpers,
    idx::Idx,
    types_analyzer::{
        CompositeType, FieldType, HeapType, InstrSigResolverCtx, NamedSig, OperandType, RefType, Sig, ValType,
        extract_elem_ref_type, extract_table_ref_type, find_comp_type_by_idx, perform_types_till, resolve_br_types,
    },
};
use wat_syntax::{AmberNode, AmberToken, SyntaxKind, SyntaxNodePtr};

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
                CompositeType::Cont(..) => {
                    diagnostics.push(build_diagnostic("array", "cont", dst, dst_symbol, ctx.db));
                    None
                }
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
                CompositeType::Cont(..) => {
                    diagnostics.push(build_diagnostic("array", "cont", src, src_symbol, ctx.db));
                    None
                }
            };

            match (dst_type, src_type) {
                (Some(FieldType { storage: dst, .. }), Some(FieldType { storage: src, .. }))
                    if !src.matches(dst, ctx.db, ctx.document, ctx.module_id) =>
                {
                    diagnostics.push(Diagnostic {
                        range: node.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!(
                            "source array type `{}` doesn't match destination array type `{}`",
                            src_symbol.idx.render(ctx.db),
                            dst_symbol.idx.render(ctx.db),
                        ),
                        related_information: Some(vec![
                            RelatedInformation {
                                range: dst_symbol.key.text_range(),
                                message: format!(
                                    "destination array type `{}` defined here",
                                    dst_symbol.idx.render(ctx.db),
                                ),
                            },
                            RelatedInformation {
                                range: src_symbol.key.text_range(),
                                message: format!("source array type `{}` defined here", src_symbol.idx.render(ctx.db)),
                            },
                        ]),
                        ..Default::default()
                    });
                }
                _ => {}
            }
        }
        Some(("array", "new_elem" | "init_elem")) => {
            let array = immediates.next()?.to_ptr();
            let array_symbol = ctx.symbol_table.find_def(array.into())?;
            let array_type = match &ctx.def_types.get(&array_symbol.key)?.comp {
                CompositeType::Func(..) => {
                    diagnostics.push(build_diagnostic("array", "func", array, array_symbol, ctx.db));
                    None
                }
                CompositeType::Struct(..) => {
                    diagnostics.push(build_diagnostic("array", "struct", array, array_symbol, ctx.db));
                    None
                }
                CompositeType::Array(field_type) => field_type.as_ref(),
                CompositeType::Cont(..) => {
                    diagnostics.push(build_diagnostic("array", "cont", array, array_symbol, ctx.db));
                    None
                }
            }?
            .clone()
            .into();

            let elem_symbol = ctx.symbol_table.find_def(immediates.next()?.to_ptr().into())?;
            let elem_type = extract_elem_ref_type(ctx.db, &elem_symbol.green)?;

            if !ValType::Ref(elem_type.clone()).matches(&array_type, ctx.db, ctx.document, ctx.module_id) {
                diagnostics.push(Diagnostic {
                    range: node.text_range(),
                    code: DIAGNOSTIC_CODE.into(),
                    message: format!(
                        "ref type `{}` of element segment `{}` doesn't match ref type `{}` of array type `{}`",
                        elem_type.render(ctx.db),
                        elem_symbol.idx.render(ctx.db),
                        array_type.render(ctx.db),
                        array_symbol.idx.render(ctx.db),
                    ),
                    related_information: Some(vec![
                        RelatedInformation {
                            range: array_symbol.key.text_range(),
                            message: format!("array type `{}` defined here", array_symbol.idx.render(ctx.db)),
                        },
                        RelatedInformation {
                            range: elem_symbol.key.text_range(),
                            message: format!("element segment `{}` defined here", elem_symbol.idx.render(ctx.db)),
                        },
                    ]),
                    ..Default::default()
                });
            }
        }
        Some(("array", _)) => {
            if let Some(diagnostic) = check_type_matches(ctx, "array", immediates.next()?.to_ptr()) {
                diagnostics.push(diagnostic);
            }
        }
        Some(("ref", "test" | "cast")) => {
            if let Some(diagnostic) = immediates.next().and_then(|immediate| check_cast(ctx, node, immediate)) {
                diagnostics.push(diagnostic);
            }
        }
        Some(("cont", "new")) => {
            if let Some(diagnostic) = check_type_matches(ctx, "cont", immediates.next()?.to_ptr()) {
                diagnostics.push(diagnostic);
            }
        }
        Some(("cont", "bind")) => {
            let module = SymbolKey::new(ctx.module);
            let fst = immediates.next()?.to_ptr();
            let fst_symbol = ctx.symbol_table.find_def(fst.into())?;
            let fst_sig = match &ctx.def_types.get(&fst_symbol.key)?.comp {
                CompositeType::Func(..) => {
                    diagnostics.push(build_diagnostic("cont", "func", fst, fst_symbol, ctx.db));
                    None
                }
                CompositeType::Struct(..) => {
                    diagnostics.push(build_diagnostic("cont", "struct", fst, fst_symbol, ctx.db));
                    None
                }
                CompositeType::Array(..) => {
                    diagnostics.push(build_diagnostic("cont", "array", fst, fst_symbol, ctx.db));
                    None
                }
                CompositeType::Cont(HeapType::Type(idx)) => {
                    find_comp_type_by_idx(ctx.symbol_table, ctx.def_types, *idx, module)
                        .and_then(CompositeType::as_func)
                }
                CompositeType::Cont(..) => None,
            };

            let snd = immediates.next()?.to_ptr();
            let snd_symbol = ctx.symbol_table.find_def(snd.into())?;
            let snd_sig = match &ctx.def_types.get(&snd_symbol.key)?.comp {
                CompositeType::Func(..) => {
                    diagnostics.push(build_diagnostic("cont", "func", snd, snd_symbol, ctx.db));
                    None
                }
                CompositeType::Struct(..) => {
                    diagnostics.push(build_diagnostic("cont", "struct", snd, snd_symbol, ctx.db));
                    None
                }
                CompositeType::Array(..) => {
                    diagnostics.push(build_diagnostic("cont", "array", snd, snd_symbol, ctx.db));
                    None
                }
                CompositeType::Cont(HeapType::Type(idx)) => {
                    find_comp_type_by_idx(ctx.symbol_table, ctx.def_types, *idx, module)
                        .and_then(CompositeType::as_func)
                }
                CompositeType::Cont(..) => None,
            };

            if let (Some(fst_sig), Some(snd_sig)) = (fst_sig, snd_sig) {
                if let Some(params) = fst_sig
                    .params
                    .len()
                    .checked_sub(snd_sig.params.len())
                    .and_then(|i| fst_sig.params.get(i..))
                {
                    let partial = NamedSig {
                        params: params.to_owned(),
                        results: fst_sig.results.clone(),
                    };
                    if !partial.matches(snd_sig, ctx.db, ctx.document, ctx.module_id) {
                        diagnostics.push(Diagnostic {
                            range: node.text_range(),
                            code: DIAGNOSTIC_CODE.into(),
                            message: format!(
                                "cont type `{}` must match cont type `{}` in continuation types",
                                fst_symbol.idx.render(ctx.db),
                                snd_symbol.idx.render(ctx.db),
                            ),
                            related_information: Some(vec![
                                RelatedInformation {
                                    range: fst_symbol.key.text_range(),
                                    message: format!(
                                        "cont type `{}` defined with: {}",
                                        fst_symbol.idx.render(ctx.db),
                                        fst_sig.render_compact(ctx.db),
                                    ),
                                },
                                RelatedInformation {
                                    range: snd_symbol.key.text_range(),
                                    message: format!(
                                        "cont type `{}` defined with: {}",
                                        snd_symbol.idx.render(ctx.db),
                                        snd_sig.render_compact(ctx.db),
                                    ),
                                },
                            ]),
                            ..Default::default()
                        });
                    }
                } else {
                    diagnostics.push(Diagnostic {
                        range: node.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!(
                            "cont type `{}` must match cont type `{}` in continuation arguments",
                            fst_symbol.idx.render(ctx.db),
                            snd_symbol.idx.render(ctx.db),
                        ),
                        related_information: Some(vec![
                            RelatedInformation {
                                range: fst_symbol.key.text_range(),
                                message: format!(
                                    "cont type `{}` defined with: {}",
                                    fst_symbol.idx.render(ctx.db),
                                    fst_sig.render_compact(ctx.db),
                                ),
                            },
                            RelatedInformation {
                                range: snd_symbol.key.text_range(),
                                message: format!(
                                    "cont type `{}` defined with: {}",
                                    snd_symbol.idx.render(ctx.db),
                                    snd_sig.render_compact(ctx.db),
                                ),
                            },
                        ]),
                        ..Default::default()
                    });
                }
            }
        }
        Some(("table", "copy")) => {
            let dst = immediates.next()?.to_ptr();
            let dst_symbol = ctx.symbol_table.find_def(dst.into())?;
            let dst_type = extract_table_ref_type(ctx.db, &dst_symbol.green)?;

            let src = immediates.next()?.to_ptr();
            let src_symbol = ctx.symbol_table.find_def(src.into())?;
            let src_type = extract_table_ref_type(ctx.db, &src_symbol.green)?;

            if !src_type.matches(&dst_type, ctx.db, ctx.document, ctx.module_id) {
                diagnostics.push(Diagnostic {
                    range: node.text_range(),
                    code: DIAGNOSTIC_CODE.into(),
                    message: format!(
                        "ref type `{}` of source table `{}` doesn't match ref type `{}` of destination table `{}`",
                        dst_type.render(ctx.db),
                        dst_symbol.idx.render(ctx.db),
                        src_type.render(ctx.db),
                        src_symbol.idx.render(ctx.db),
                    ),
                    related_information: Some(vec![
                        RelatedInformation {
                            range: dst_symbol.key.text_range(),
                            message: format!("destination table `{}` defined here", dst_symbol.idx.render(ctx.db)),
                        },
                        RelatedInformation {
                            range: src_symbol.key.text_range(),
                            message: format!("source table `{}` defined here", src_symbol.idx.render(ctx.db)),
                        },
                    ]),
                    ..Default::default()
                });
            }
        }
        Some(("table", "init")) => {
            let symbol = ctx.symbol_table.find_def(immediates.next()?.to_ptr().into())?;
            let (table_symbol, table_type, elem_symbol, elem_type) = if symbol.kind == SymbolKind::TableDef {
                let table_type = extract_table_ref_type(ctx.db, &symbol.green)?;
                let elem_symbol = ctx.symbol_table.find_def(immediates.next()?.to_ptr().into())?;
                let elem_type = extract_elem_ref_type(ctx.db, &elem_symbol.green)?;
                (symbol, table_type, elem_symbol, elem_type)
            } else {
                let table_symbol = ctx.symbol_table.find_def_by_idx(
                    Idx {
                        num: Some(0),
                        name: None,
                    },
                    SymbolKind::TableDef,
                    SymbolKey::new(ctx.module),
                )?;
                let table_type = extract_table_ref_type(ctx.db, &table_symbol.green)?;
                let elem_type = extract_elem_ref_type(ctx.db, &symbol.green)?;
                (table_symbol, table_type, symbol, elem_type)
            };
            if !elem_type.matches(&table_type, ctx.db, ctx.document, ctx.module_id) {
                diagnostics.push(Diagnostic {
                    range: node.text_range(),
                    code: DIAGNOSTIC_CODE.into(),
                    message: format!(
                        "ref type `{}` of element segment `{}` doesn't match ref type `{}` of table `{}`",
                        elem_type.render(ctx.db),
                        elem_symbol.idx.render(ctx.db),
                        table_type.render(ctx.db),
                        table_symbol.idx.render(ctx.db),
                    ),
                    related_information: Some(vec![
                        RelatedInformation {
                            range: table_symbol.key.text_range(),
                            message: format!("table `{}` defined here", table_symbol.idx.render(ctx.db)),
                        },
                        RelatedInformation {
                            range: elem_symbol.key.text_range(),
                            message: format!("element segment `{}` defined here", elem_symbol.idx.render(ctx.db)),
                        },
                    ]),
                    ..Default::default()
                });
            }
        }
        _ => match instr_name.text() {
            "call_ref" => {
                if let Some(diagnostic) = check_type_matches(ctx, "func", immediates.next()?.to_ptr()) {
                    diagnostics.push(diagnostic);
                }
            }
            "br_on_null" => {
                if let Some(outer_block) = node
                    .to_ptr()
                    .to_node(ctx.module)
                    .and_then(|node| helpers::syntax::find_outer_block_for_types(&node))
                    && let Some((stack, _)) = perform_types_till(
                        node,
                        &outer_block,
                        &InstrSigResolverCtx {
                            db: ctx.db,
                            document: ctx.document,
                            symbol_table: ctx.symbol_table,
                            def_types: ctx.def_types,
                            module: ctx.module,
                            module_id: ctx.module_id,
                            bump: ctx.bump,
                        },
                    )
                {
                    const BASE_MSG: &str = "first type from stack top must be ref type";
                    match stack.last() {
                        Some(OperandType::Val(ValType::Ref(..))) => {}
                        Some(ty) => {
                            diagnostics.push(Diagnostic {
                                range: node.text_range(),
                                code: DIAGNOSTIC_CODE.into(),
                                message: format!("{BASE_MSG} but found `{}`", ty.render(ctx.db)),
                                ..Default::default()
                            });
                        }
                        None => {
                            diagnostics.push(Diagnostic {
                                range: node.text_range(),
                                code: DIAGNOSTIC_CODE.into(),
                                message: BASE_MSG.into(),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
            "br_on_non_null" => {
                let immediate = immediates.next()?.to_ptr();
                let last = resolve_br_types(ctx.db, ctx.document, ctx.symbol_table, immediate.into())
                    .and_then(|mut types| types.next_back());
                if !matches!(last, Some(OperandType::Val(ValType::Ref(..))))
                    && let Some(symbol) = ctx.symbol_table.symbols.get(&SymbolKey::from(immediate))
                {
                    diagnostics.push(Diagnostic {
                        range: immediate.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!(
                            "the last type of label `{}` must be ref type",
                            symbol.idx.render(ctx.db),
                        ),
                        ..Default::default()
                    });
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
                if let Some(diagnostic) = check_cast(ctx, node, rt1_node) {
                    diagnostics.push(diagnostic);
                }
                if let Some(diagnostic) = check_cast(ctx, node, rt2_node) {
                    diagnostics.push(diagnostic);
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
                if let Some(diagnostic) = check_cast(ctx, node, rt1_node) {
                    diagnostics.push(diagnostic);
                }
                if let Some(diagnostic) = check_cast(ctx, node, rt2_node) {
                    diagnostics.push(diagnostic);
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
                        .map(|func| Sig::from_func(ctx.db, ctx.document, func.amber()))
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
                        &Sig::from_type_use(ctx.db, ctx.document, type_use).results,
                    )
                {
                    diagnostics.push(diagnostic);
                }
            }
            "throw" => {
                if let Some(immediate) = immediates.next()
                    && let Some(symbol) = ctx.symbol_table.find_def(immediate.to_ptr().into())
                    && !Sig::from_func(ctx.db, ctx.document, symbol.amber()).results.is_empty()
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
            "resume" | "resume_throw" | "resume_throw_ref" => {
                let ct = immediates.next()?.to_ptr();
                let ct_symbol = ctx.symbol_table.find_def(ct.into())?;
                match &ctx.def_types.get(&ct_symbol.key)?.comp {
                    CompositeType::Func(..) => {
                        diagnostics.push(build_diagnostic("cont", "func", ct, ct_symbol, ctx.db));
                    }
                    CompositeType::Struct(..) => {
                        diagnostics.push(build_diagnostic("cont", "struct", ct, ct_symbol, ctx.db));
                    }
                    CompositeType::Array(..) => {
                        diagnostics.push(build_diagnostic("cont", "array", ct, ct_symbol, ctx.db));
                    }
                    CompositeType::Cont(HeapType::Type(idx)) => {
                        let ct_sig =
                            find_comp_type_by_idx(ctx.symbol_table, ctx.def_types, *idx, SymbolKey::new(ctx.module))?
                                .as_func()?;
                        diagnostics.extend(
                            immediates.filter_map(|immediate| check_on_clause(ctx, immediate, &ct_sig.results)),
                        );
                    }
                    CompositeType::Cont(..) => {}
                }
            }
            "switch" => {
                let module = SymbolKey::new(ctx.module);
                let ct = immediates.next()?.to_ptr();
                let ct_ref_symbol = ctx.symbol_table.symbols.get(&SymbolKey::from(ct))?;
                let ct_def_symbol = ctx.symbol_table.find_def(ct_ref_symbol.key)?;
                let ct_sig = match &ctx.def_types.get(&ct_def_symbol.key)?.comp {
                    CompositeType::Func(..) => {
                        diagnostics.push(build_diagnostic("cont", "func", ct, ct_def_symbol, ctx.db));
                        None
                    }
                    CompositeType::Struct(..) => {
                        diagnostics.push(build_diagnostic("cont", "struct", ct, ct_def_symbol, ctx.db));
                        None
                    }
                    CompositeType::Array(..) => {
                        diagnostics.push(build_diagnostic("cont", "array", ct, ct_def_symbol, ctx.db));
                        None
                    }
                    CompositeType::Cont(HeapType::Type(idx)) => {
                        find_comp_type_by_idx(ctx.symbol_table, ctx.def_types, *idx, module)
                            .and_then(CompositeType::as_func)
                    }
                    CompositeType::Cont(..) => None,
                }?;

                let tag_ref_symbol = ctx
                    .symbol_table
                    .symbols
                    .get(&SymbolKey::from(immediates.next()?.to_ptr()))?;
                let tag_def_symbol = ctx.symbol_table.find_def(tag_ref_symbol.key)?;
                let tag_sig = Sig::from_func(ctx.db, ctx.document, tag_def_symbol.amber());
                if !tag_sig.params.is_empty() {
                    diagnostics.push(Diagnostic {
                        range: tag_ref_symbol.key.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!(
                            "param types of tag `{}` must be empty when used in `switch`",
                            tag_ref_symbol.idx.render(ctx.db),
                        ),
                        related_information: Some(vec![RelatedInformation {
                            range: tag_def_symbol.key.text_range(),
                            message: format!("tag `{}` defined here", tag_ref_symbol.idx.render(ctx.db)),
                        }]),
                        ..Default::default()
                    });
                }
                if ct_sig.results.len() != tag_sig.results.len()
                    || ct_sig
                        .results
                        .iter()
                        .zip(&tag_sig.results)
                        .any(|(a, b)| !a.matches(b, ctx.db, ctx.document, ctx.module_id))
                {
                    diagnostics.push(Diagnostic {
                        range: tag_ref_symbol.key.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!(
                            "result types of cont type `{}` must match result types of tag `{}`",
                            ct_ref_symbol.idx.render(ctx.db),
                            tag_ref_symbol.idx.render(ctx.db),
                        ),
                        related_information: Some(vec![
                            RelatedInformation {
                                range: ct_def_symbol.key.text_range(),
                                message: format!("cont type `{}` defined here", ct_ref_symbol.idx.render(ctx.db)),
                            },
                            RelatedInformation {
                                range: tag_def_symbol.key.text_range(),
                                message: format!("tag `{}` defined here", tag_ref_symbol.idx.render(ctx.db)),
                            },
                        ]),
                        ..Default::default()
                    });
                }

                if let Some((
                    ValType::Ref(RefType {
                        heap_ty: HeapType::Type(cont_idx),
                        ..
                    }),
                    _,
                )) = ct_sig.params.last()
                    && let CompositeType::Cont(HeapType::Type(ft_idx)) =
                        find_comp_type_by_idx(ctx.symbol_table, ctx.def_types, *cont_idx, module)?
                    && let CompositeType::Func(last_ct_sig) =
                        find_comp_type_by_idx(ctx.symbol_table, ctx.def_types, *ft_idx, module)?
                {
                    if tag_sig.results.len() != last_ct_sig.results.len()
                        || tag_sig
                            .results
                            .iter()
                            .zip(&last_ct_sig.results)
                            .any(|(a, b)| !a.matches(b, ctx.db, ctx.document, ctx.module_id))
                    {
                        diagnostics.push(Diagnostic {
                            range: tag_ref_symbol.key.text_range(),
                            code: DIAGNOSTIC_CODE.into(),
                            message: format!(
                                "result types of tag `{}` must match result types of cont type `{}`",
                                tag_ref_symbol.idx.render(ctx.db),
                                cont_idx.render(ctx.db),
                            ),
                            related_information: ctx
                                .symbol_table
                                .find_def_by_idx(*cont_idx, SymbolKind::Type, module)
                                .map(|cont_def_symbol| {
                                    vec![
                                        RelatedInformation {
                                            range: tag_def_symbol.key.text_range(),
                                            message: format!(
                                                "tag `{}` defined here",
                                                tag_ref_symbol.idx.render(ctx.db)
                                            ),
                                        },
                                        RelatedInformation {
                                            range: cont_def_symbol.key.text_range(),
                                            message: format!("cont type `{}` defined here", cont_idx.render(ctx.db)),
                                        },
                                    ]
                                }),
                            ..Default::default()
                        });
                    }
                } else {
                    diagnostics.push(Diagnostic {
                        range: ct_ref_symbol.key.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!(
                            "last param type of cont type `{}` must be continuation reference type",
                            ct_ref_symbol.idx.render(ctx.db)
                        ),
                        related_information: Some(vec![RelatedInformation {
                            range: ct_def_symbol.key.text_range(),
                            message: format!("cont type `{}` defined here", ct_ref_symbol.idx.render(ctx.db)),
                        }]),
                        ..Default::default()
                    });
                }
            }
            "select" => {
                if immediates.next().is_none()
                    && let Some(outer_block) = node
                        .to_ptr()
                        .to_node(ctx.module)
                        .and_then(|node| helpers::syntax::find_outer_block_for_types(&node))
                    && let Some((stack, _)) = perform_types_till(
                        node,
                        &outer_block,
                        &InstrSigResolverCtx {
                            db: ctx.db,
                            document: ctx.document,
                            symbol_table: ctx.symbol_table,
                            def_types: ctx.def_types,
                            module: ctx.module,
                            module_id: ctx.module_id,
                            bump: ctx.bump,
                        },
                    )
                {
                    const BASE_MSG: &str = "second type from stack top must be number type or vector type";
                    match stack.len().checked_sub(2).and_then(|i| stack.get(i)) {
                        Some(OperandType::Val(
                            ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 | ValType::V128,
                        )) => {}
                        Some(ty) => {
                            diagnostics.push(Diagnostic {
                                range: node.text_range(),
                                code: DIAGNOSTIC_CODE.into(),
                                message: format!("{BASE_MSG} but found `{}`", ty.render(ctx.db)),
                                ..Default::default()
                            });
                        }
                        None => {
                            diagnostics.push(Diagnostic {
                                range: node.text_range(),
                                code: DIAGNOSTIC_CODE.into(),
                                message: BASE_MSG.into(),
                                ..Default::default()
                            });
                        }
                    }
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
        CompositeType::Cont(..) => "cont",
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
            .and_then(|symbol| extract_table_ref_type(ctx.db, &symbol.green))
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
    let expected = Sig::from_func(ctx.db, ctx.document, func.amber()).results;
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

fn check_on_clause(ctx: &DiagnosticCtx, immediate: AmberNode, ct_results: &[ValType]) -> Option<Diagnostic> {
    let module = SymbolKey::new(ctx.module);
    // Though `resume_throw` will run this check for the second immediate which is tagidx,
    // it won't report diagnostic because there're no `ON_CLAUSE` children.
    let on_clause = immediate.children_by_kind(SyntaxKind::ON_CLAUSE).next()?;
    let mut indexes = on_clause.children_by_kind(SyntaxKind::INDEX);
    let tag_ref_symbol = ctx
        .symbol_table
        .symbols
        .get(&SymbolKey::from(indexes.next()?.to_ptr()))?;
    let tag_def_symbol = ctx.symbol_table.find_def(tag_ref_symbol.key)?;
    let tag_sig = Sig::from_func(ctx.db, ctx.document, tag_def_symbol.amber());
    if let Some(label_index) = indexes.next() {
        let block_symbol = ctx.symbol_table.find_def(label_index.to_ptr().into())?;
        let block_sig = Sig::from_func(ctx.db, ctx.document, block_symbol.amber());
        if let Some((
            ValType::Ref(RefType {
                heap_ty: HeapType::Type(cont_idx),
                ..
            }),
            block_results_rest,
        )) = block_sig.results.split_last()
            && let cont_in_block = ctx.symbol_table.find_def_by_idx(*cont_idx, SymbolKind::Type, module)?
            && let CompositeType::Cont(HeapType::Type(ft_idx)) = ctx.def_types.get(&cont_in_block.key)?.comp
        {
            if tag_sig.params.len() != block_results_rest.len()
                || tag_sig
                    .params
                    .iter()
                    .zip(block_results_rest)
                    .any(|(a, b)| !a.matches(b, ctx.db, ctx.document, ctx.module_id))
            {
                return Some(Diagnostic {
                    range: label_index.text_range(),
                    code: DIAGNOSTIC_CODE.into(),
                    message: format!(
                        "param types of tag `{}` must match result types of label `{}`",
                        tag_ref_symbol.idx.render(ctx.db),
                        label_index.green(),
                    ),
                    related_information: block_symbol.amber().children_by_kind(SyntaxKind::TYPE_USE).next().map(
                        |type_use| {
                            vec![
                                RelatedInformation {
                                    range: tag_def_symbol.key.text_range(),
                                    message: format!("tag `{}` defined here", tag_ref_symbol.idx.render(ctx.db)),
                                },
                                RelatedInformation {
                                    range: type_use.text_range(),
                                    message: format!("type of label `{}` defined here", label_index.green()),
                                },
                            ]
                        },
                    ),
                    ..Default::default()
                });
            }
            let ct_sig = find_comp_type_by_idx(ctx.symbol_table, ctx.def_types, ft_idx, module)?.as_func()?;
            let tmp = NamedSig {
                params: tag_sig.results.iter().map(|ty| (ty.clone(), None)).collect(),
                results: ct_results.to_owned(),
            };
            if tmp.matches(ct_sig, ctx.db, ctx.document, ctx.module_id) {
                None
            } else {
                Some(Diagnostic {
                    range: label_index.text_range(),
                    code: DIAGNOSTIC_CODE.into(),
                    message: format!(
                        "result types of tag `{}` must match result types of cont type `{}`",
                        tag_ref_symbol.idx.render(ctx.db),
                        cont_idx.render(ctx.db),
                    ),
                    related_information: Some(vec![
                        RelatedInformation {
                            range: tag_def_symbol.key.text_range(),
                            message: format!("tag `{}` defined here", tag_ref_symbol.idx.render(ctx.db)),
                        },
                        RelatedInformation {
                            range: cont_in_block.key.text_range(),
                            message: format!("cont type `{}` defined here", cont_idx.render(ctx.db)),
                        },
                    ]),
                    ..Default::default()
                })
            }
        } else {
            Some(Diagnostic {
                range: label_index.text_range(),
                code: DIAGNOSTIC_CODE.into(),
                message: format!(
                    "last result type of label `{}` must be continuation reference type",
                    label_index.green()
                ),
                related_information: block_symbol.amber().children_by_kind(SyntaxKind::TYPE_USE).next().map(
                    |type_use| {
                        vec![RelatedInformation {
                            range: type_use.text_range(),
                            message: format!("type of label `{}` declared here", label_index.green()),
                        }]
                    },
                ),
                ..Default::default()
            })
        }
    } else if on_clause.tokens_by_kind(SyntaxKind::MODIFIER_KEYWORD).next().is_none() || tag_sig.params.is_empty() {
        None
    } else {
        Some(Diagnostic {
            range: tag_ref_symbol.key.text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message: format!(
                "param types of tag `{}` must be empty",
                tag_ref_symbol.idx.render(ctx.db),
            ),
            related_information: Some(vec![RelatedInformation {
                range: tag_def_symbol.key.text_range(),
                message: format!("tag `{}` defined here", tag_ref_symbol.idx.render(ctx.db)),
            }]),
            ..Default::default()
        })
    }
}

fn check_cast(ctx: &DiagnosticCtx, instr: AmberNode, immediate: AmberNode) -> Option<Diagnostic> {
    let ref_type = RefType::from_green(immediate.children_by_kind(SyntaxKind::REF_TYPE).next()?.green(), ctx.db)?;
    let is_cont = match ref_type.heap_ty {
        HeapType::Cont | HeapType::NoCont => true,
        HeapType::Type(idx) => matches!(
            find_comp_type_by_idx(ctx.symbol_table, ctx.def_types, idx, SymbolKey::new(ctx.module))?,
            CompositeType::Cont(..)
        ),
        _ => false,
    };
    if is_cont {
        Some(Diagnostic {
            range: instr.text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message: format!("cannot cast to continuation type `{}`", ref_type.render(ctx.db)),
            ..Default::default()
        })
    } else {
        None
    }
}

fn build_diagnostic(
    expected_kind: &'static str,
    actual_kind: &str,
    ptr: SyntaxNodePtr,
    def_symbol: &Symbol,
    db: &dyn salsa::Database,
) -> Diagnostic {
    debug_assert!(matches!(expected_kind, "func" | "struct" | "array" | "cont"));
    debug_assert!(matches!(actual_kind, "func" | "struct" | "array" | "cont"));
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
