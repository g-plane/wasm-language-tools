use super::{Diagnostic, FastPlainInstr, RelatedInformation};
use crate::{
    binder::{Symbol, SymbolKey, SymbolTable},
    document::Document,
    types_analyzer::{
        CompositeType, DefTypes, FieldType, HeapType, OperandType, RefType, ValType, get_def_types, get_func_sig,
        get_type_use_sig, resolve_br_types,
    },
};
use rowan::{
    Language, NodeOrToken,
    ast::{AstNode, support},
};
use wat_syntax::{SyntaxKind, SyntaxNode, SyntaxNodePtr, WatLanguage, ast::Immediate};

const DIAGNOSTIC_CODE: &str = "type-misuse";

pub fn check(
    db: &dyn salsa::Database,
    diagnostics: &mut Vec<Diagnostic>,
    document: Document,
    symbol_table: &SymbolTable,
    module_id: u32,
    node: &SyntaxNode,
    instr: &FastPlainInstr,
) -> Option<()> {
    let def_types = get_def_types(db, document);
    match instr.name.split_once('.') {
        Some(("struct", _)) => {
            if let Some(diagnostic) =
                check_type_matches("struct", *instr.immediates.first()?, db, symbol_table, def_types)
            {
                diagnostics.push(diagnostic);
            }
        }
        Some(("array", "copy")) => {
            let dst = *instr.immediates.first()?;
            let dst_symbol = symbol_table.find_def(dst.into())?;
            let dst_type = match &def_types.get(&dst_symbol.key)?.comp {
                CompositeType::Func(..) => {
                    diagnostics.push(build_diagnostic("array", "func", dst, dst_symbol, db));
                    None
                }
                CompositeType::Struct(..) => {
                    diagnostics.push(build_diagnostic("array", "struct", dst, dst_symbol, db));
                    None
                }
                CompositeType::Array(field_type) => field_type.as_ref(),
            };

            let src = *instr.immediates.get(1)?;
            let src_symbol = symbol_table.find_def(src.into())?;
            let src_type = match &def_types.get(&src_symbol.key)?.comp {
                CompositeType::Func(..) => {
                    diagnostics.push(build_diagnostic("array", "func", src, src_symbol, db));
                    None
                }
                CompositeType::Struct(..) => {
                    diagnostics.push(build_diagnostic("array", "struct", src, src_symbol, db));
                    None
                }
                CompositeType::Array(field_type) => field_type.as_ref(),
            };

            match (dst_type, src_type) {
                (Some(FieldType { storage: dst, .. }), Some(FieldType { storage: src, .. }))
                    if !src.matches(dst, db, document, module_id) =>
                {
                    diagnostics.push(Diagnostic {
                        range: instr.ptr.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!(
                            "destination array type `{}` doesn't match source array type `{}`",
                            dst_symbol.idx.render(db),
                            src_symbol.idx.render(db),
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
            if let Some(diagnostic) =
                check_type_matches("array", *instr.immediates.first()?, db, symbol_table, def_types)
            {
                diagnostics.push(diagnostic);
            }
        }
        _ => match instr.name {
            "call_ref" => {
                if let Some(diagnostic) =
                    check_type_matches("func", *instr.immediates.first()?, db, symbol_table, def_types)
                {
                    diagnostics.push(diagnostic);
                }
            }
            "br_on_cast" => {
                let mut immediates = support::children::<Immediate>(node);
                let label = immediates.next()?;
                let label_types = resolve_br_types(db, document, symbol_table, SymbolKey::new(label.syntax()));
                let rt_label = if let Some(OperandType::Val(ValType::Ref(rt_label))) = label_types.last() {
                    rt_label
                } else {
                    diagnostics.push(Diagnostic {
                        range: label.syntax().text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: "the last type of this label must be a ref type".into(),
                        ..Default::default()
                    });
                    return None;
                };
                let rt1_node = immediates.next()?;
                let rt1 = RefType::from_green(&rt1_node.ref_type()?.syntax().green(), db)?;
                let rt2_node = immediates.next()?;
                let rt2 = RefType::from_green(&rt2_node.ref_type()?.syntax().green(), db)?;
                if !rt2.matches(&rt1, db, document, module_id) {
                    diagnostics.push(Diagnostic {
                        range: rt2_node.syntax().text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!(
                            "ref type `{}` doesn't match the ref type `{}`",
                            rt2.render(db),
                            rt1.render(db),
                        ),
                        related_information: Some(vec![RelatedInformation {
                            range: rt1_node.syntax().text_range(),
                            message: "should match this ref type".into(),
                        }]),
                        ..Default::default()
                    });
                }
                if !rt2.matches(rt_label, db, document, module_id) {
                    diagnostics.push(Diagnostic {
                        range: rt2_node.syntax().text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!(
                            "ref type `{}` doesn't match the ref type `{}`",
                            rt2.render(db),
                            rt_label.render(db),
                        ),
                        related_information: Some(vec![RelatedInformation {
                            range: label.syntax().text_range(),
                            message: "should match the last ref type in the result type of this label".into(),
                        }]),
                        ..Default::default()
                    });
                }
            }
            "br_on_cast_fail" => {
                let mut immediates = support::children::<Immediate>(node);
                let label = immediates.next()?;
                let label_types = resolve_br_types(db, document, symbol_table, SymbolKey::new(label.syntax()));
                let rt_label = if let Some(OperandType::Val(ValType::Ref(rt_label))) = label_types.last() {
                    rt_label
                } else {
                    diagnostics.push(Diagnostic {
                        range: label.syntax().text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: "the last type of this label must be a ref type".into(),
                        ..Default::default()
                    });
                    return None;
                };
                let rt1_node = immediates.next()?;
                let rt1 = RefType::from_green(&rt1_node.ref_type()?.syntax().green(), db)?;
                let rt2_node = immediates.next()?;
                let rt2 = RefType::from_green(&rt2_node.ref_type()?.syntax().green(), db)?;
                if !rt2.matches(&rt1, db, document, module_id) {
                    diagnostics.push(Diagnostic {
                        range: rt2_node.syntax().text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!(
                            "ref type `{}` doesn't match the ref type `{}`",
                            rt2.render(db),
                            rt1.render(db),
                        ),
                        related_information: Some(vec![RelatedInformation {
                            range: rt1_node.syntax().text_range(),
                            message: "should match this ref type".into(),
                        }]),
                        ..Default::default()
                    });
                }
                let rt_diff = rt1.diff(&rt2);
                if !rt_diff.matches(rt_label, db, document, module_id) {
                    diagnostics.push(Diagnostic {
                        range: node.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!(
                            "type difference between given two ref types `{}` doesn't match the ref type `{}`",
                            rt_diff.render(db),
                            rt_label.render(db),
                        ),
                        related_information: Some(vec![RelatedInformation {
                            range: label.syntax().text_range(),
                            message: "should match the last ref type in the result type of this label".into(),
                        }]),
                        ..Default::default()
                    });
                }
            }
            "call_indirect" => {
                if let Some(diagnostic) = check_table_ref_type(db, document, symbol_table, module_id, node) {
                    diagnostics.push(diagnostic);
                }
            }
            "return_call" => {
                if let Some(immediate) = node.first_child_by_kind(&|kind| kind == SyntaxKind::IMMEDIATE)
                    && let Some(diagnostic) = symbol_table
                        .find_def(SymbolKey::new(&immediate))
                        .map(|func| get_func_sig(db, document, func.key, &func.green))
                        .and_then(|sig| {
                            check_return_call_result_type(db, document, module_id, node, &immediate, &sig.results)
                        })
                {
                    diagnostics.push(diagnostic);
                }
            }
            "return_call_ref" => {
                if let Some(diagnostic) =
                    check_type_matches("func", *instr.immediates.first()?, db, symbol_table, def_types)
                {
                    diagnostics.push(diagnostic);
                }
                if let Some(immediate) = node.first_child_by_kind(&|kind| kind == SyntaxKind::IMMEDIATE)
                    && let Some(diagnostic) = symbol_table
                        .resolved
                        .get(&SymbolKey::new(&immediate))
                        .and_then(|key| def_types.get(key))
                        .and_then(|def_type| def_type.comp.as_func())
                        .and_then(|sig| {
                            check_return_call_result_type(db, document, module_id, node, &immediate, &sig.results)
                        })
                {
                    diagnostics.push(diagnostic);
                }
            }
            "return_call_indirect" => {
                if let Some(diagnostic) = check_table_ref_type(db, document, symbol_table, module_id, node) {
                    diagnostics.push(diagnostic);
                }
                if let Some(type_use) = node
                    .children()
                    .find_map(|immediate| immediate.first_child_by_kind(&|kind| kind == SyntaxKind::TYPE_USE))
                    && let Some(diagnostic) = check_return_call_result_type(
                        db,
                        document,
                        module_id,
                        node,
                        &type_use,
                        &get_type_use_sig(db, document, SyntaxNodePtr::new(&type_use), &type_use.green()).results,
                    )
                {
                    diagnostics.push(diagnostic);
                }
            }
            _ => {}
        },
    }
    Some(())
}

fn check_type_matches(
    expected_kind: &'static str,
    immediate: SyntaxNodePtr,
    db: &dyn salsa::Database,
    symbol_table: &SymbolTable,
    def_types: &DefTypes,
) -> Option<Diagnostic> {
    let def_symbol = symbol_table.find_def(immediate.into())?;
    let def_type = def_types.get(&def_symbol.key)?;
    let kind = match def_type.comp {
        CompositeType::Func(..) => "func",
        CompositeType::Struct(..) => "struct",
        CompositeType::Array(..) => "array",
    };
    if kind == expected_kind {
        None
    } else {
        Some(build_diagnostic(expected_kind, kind, immediate, def_symbol, db))
    }
}

fn check_table_ref_type(
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
    module_id: u32,
    node: &SyntaxNode,
) -> Option<Diagnostic> {
    if let Some(ref_key) = support::children::<Immediate>(node)
        .find(|immediate| {
            matches!(
                immediate.syntax().first_token().map(|token| token.kind()),
                Some(SyntaxKind::INT | SyntaxKind::UNSIGNED_INT | SyntaxKind::IDENT)
            )
        })
        .map(|immediate| SymbolKey::new(immediate.syntax()))
        && let Some(ref_symbol) = symbol_table.symbols.get(&ref_key)
        && symbol_table
            .find_def(ref_key)
            .and_then(|symbol| {
                symbol.green.children().find_map(|child| {
                    if let NodeOrToken::Node(node) = child {
                        match WatLanguage::kind_from_raw(node.kind()) {
                            SyntaxKind::REF_TYPE => Some(node),
                            SyntaxKind::TABLE_TYPE => node.children().find_map(|child| {
                                if let NodeOrToken::Node(node) = child
                                    && node.kind() == SyntaxKind::REF_TYPE.into()
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
            .and_then(|green| RefType::from_green(green, db))
            .is_some_and(|ty| {
                !ty.matches(
                    &RefType {
                        heap_ty: HeapType::Func,
                        nullable: true,
                    },
                    db,
                    document,
                    module_id,
                )
            })
    {
        Some(Diagnostic {
            range: ref_key.text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message: format!(
                "ref type of table `{}` must match `(ref null func)`",
                ref_symbol.idx.render(db),
            ),
            ..Default::default()
        })
    } else {
        None
    }
}

fn check_return_call_result_type(
    db: &dyn salsa::Database,
    document: Document,
    module_id: u32,
    instr: &SyntaxNode,
    reported_node: &SyntaxNode,
    actual: &[ValType],
) -> Option<Diagnostic> {
    let func = instr
        .ancestors()
        .find(|ancestor| ancestor.kind() == SyntaxKind::MODULE_FIELD_FUNC)?;
    let expected = get_func_sig(db, document, SymbolKey::new(&func), &func.green()).results;
    if actual.len() == expected.len()
        && actual
            .iter()
            .zip(expected.iter())
            .all(|(actual, expected)| actual.matches(expected, db, document, module_id))
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
