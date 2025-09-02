use crate::{
    LanguageService,
    binder::{Symbol, SymbolKey, SymbolTable},
    document::Document,
    helpers,
    types_analyzer::{
        CompositeType, DefType, FieldType, OperandType, RefType, ValType, get_def_types,
        resolve_br_types,
    },
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, Union2};
use rowan::ast::{AstNode, support};
use wat_syntax::{SyntaxKind, SyntaxNode, ast::Immediate};

const DIAGNOSTIC_CODE: &str = "type-misuse";

pub fn check(
    service: &LanguageService,
    diagnostics: &mut Vec<Diagnostic>,
    document: Document,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    module_id: u32,
    node: &SyntaxNode,
) -> Option<()> {
    let def_types = get_def_types(service, document);
    let instr_name = support::token(node, SyntaxKind::INSTR_NAME)?;
    let instr_name = instr_name.text();
    match instr_name.split_once('.') {
        Some(("struct", _)) => {
            if let Some(diagnostic) = check_type_matches(
                "struct",
                &node.first_child()?,
                service,
                document,
                line_index,
                symbol_table,
                def_types,
            ) {
                diagnostics.push(diagnostic);
            }
        }
        Some(("array", "copy")) => {
            let mut children = node.children();
            let dst = children.next()?;
            let dst_symbol = symbol_table.find_def(SymbolKey::new(&dst))?;
            let dst_type = match &def_types
                .iter()
                .find(|def_type| def_type.key == dst_symbol.key)?
                .comp
            {
                CompositeType::Func(..) => {
                    diagnostics.push(build_diagnostic(
                        "array", "func", &dst, dst_symbol, service, document, line_index,
                    ));
                    None
                }
                CompositeType::Struct(..) => {
                    diagnostics.push(build_diagnostic(
                        "array", "struct", &dst, dst_symbol, service, document, line_index,
                    ));
                    None
                }
                CompositeType::Array(field_type) => field_type.as_ref(),
            };

            let src = children.next()?;
            let src_symbol = symbol_table.find_def(SymbolKey::new(&src))?;
            let src_type = match &def_types
                .iter()
                .find(|def_type| def_type.key == src_symbol.key)?
                .comp
            {
                CompositeType::Func(..) => {
                    diagnostics.push(build_diagnostic(
                        "array", "func", &src, src_symbol, service, document, line_index,
                    ));
                    None
                }
                CompositeType::Struct(..) => {
                    diagnostics.push(build_diagnostic(
                        "array", "struct", &src, src_symbol, service, document, line_index,
                    ));
                    None
                }
                CompositeType::Array(field_type) => field_type.as_ref(),
            };

            match (dst_type, src_type) {
                (Some(FieldType { storage: dst, .. }), Some(FieldType { storage: src, .. }))
                    if !src.matches(dst, service, document, module_id) =>
                {
                    diagnostics.push(Diagnostic {
                        range: helpers::rowan_range_to_lsp_range(line_index, node.text_range()),
                        severity: Some(DiagnosticSeverity::Error),
                        source: Some("wat".into()),
                        code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                        message: format!(
                            "destination array type `{}` doesn't match source array type `{}`",
                            dst_symbol.idx.render(service),
                            src_symbol.idx.render(service),
                        ),
                        related_information: Some(vec![
                            DiagnosticRelatedInformation {
                                location: Location {
                                    uri: document.uri(service).raw(service),
                                    range: helpers::rowan_range_to_lsp_range(
                                        line_index,
                                        dst_symbol.key.text_range(),
                                    ),
                                },
                                message: "destination array type defined here".into(),
                            },
                            DiagnosticRelatedInformation {
                                location: Location {
                                    uri: document.uri(service).raw(service),
                                    range: helpers::rowan_range_to_lsp_range(
                                        line_index,
                                        src_symbol.key.text_range(),
                                    ),
                                },
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
            if let Some(diagnostic) = check_type_matches(
                "array",
                &node.first_child()?,
                service,
                document,
                line_index,
                symbol_table,
                def_types,
            ) {
                diagnostics.push(diagnostic);
            }
        }
        _ => match instr_name {
            "call_ref" | "return_call_ref" => {
                if let Some(diagnostic) = check_type_matches(
                    "func",
                    &node.first_child()?,
                    service,
                    document,
                    line_index,
                    symbol_table,
                    def_types,
                ) {
                    diagnostics.push(diagnostic);
                }
            }
            "br_on_cast" => {
                let mut immediates = support::children::<Immediate>(node);
                let label = immediates.next()?;
                let label_types = resolve_br_types(service, document, symbol_table, &label);
                let rt_label =
                    if let Some(OperandType::Val(ValType::Ref(rt_label))) = label_types.last() {
                        rt_label
                    } else {
                        diagnostics.push(Diagnostic {
                            range: helpers::rowan_range_to_lsp_range(
                                line_index,
                                label.syntax().text_range(),
                            ),
                            severity: Some(DiagnosticSeverity::Error),
                            source: Some("wat".into()),
                            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                            message: "the last type of this label must be a ref type".into(),
                            ..Default::default()
                        });
                        return None;
                    };
                let rt1_node = immediates.next()?;
                let rt1 = RefType::from_green(&rt1_node.ref_type()?.syntax().green(), service)?;
                let rt2_node = immediates.next()?;
                let rt2 = RefType::from_green(&rt2_node.ref_type()?.syntax().green(), service)?;
                if !rt2.matches(&rt1, service, document, module_id) {
                    diagnostics.push(Diagnostic {
                        range: helpers::rowan_range_to_lsp_range(
                            line_index,
                            rt2_node.syntax().text_range(),
                        ),
                        severity: Some(DiagnosticSeverity::Error),
                        source: Some("wat".into()),
                        code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                        message: format!(
                            "ref type `{}` doesn't match the ref type `{}`",
                            rt2.render(service),
                            rt1.render(service),
                        ),
                        related_information: Some(vec![DiagnosticRelatedInformation {
                            location: Location {
                                uri: document.uri(service).raw(service),
                                range: helpers::rowan_range_to_lsp_range(
                                    line_index,
                                    rt1_node.syntax().text_range(),
                                ),
                            },
                            message: "should match this ref type".into(),
                        }]),
                        ..Default::default()
                    });
                }
                if !rt2.matches(rt_label, service, document, module_id) {
                    diagnostics.push(Diagnostic {
                        range: helpers::rowan_range_to_lsp_range(
                            line_index,
                            rt2_node.syntax().text_range(),
                        ),
                        severity: Some(DiagnosticSeverity::Error),
                        source: Some("wat".into()),
                        code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                        message: format!(
                            "ref type `{}` doesn't match the ref type `{}`",
                            rt2.render(service),
                            rt_label.render(service),
                        ),
                        related_information: Some(vec![DiagnosticRelatedInformation {
                            location: Location {
                                uri: document.uri(service).raw(service),
                                range: helpers::rowan_range_to_lsp_range(
                                    line_index,
                                    label.syntax().text_range(),
                                ),
                            },
                            message:
                                "should match the last ref type in the result type of this label"
                                    .into(),
                        }]),
                        ..Default::default()
                    });
                }
            }
            "br_on_cast_fail" => {
                let mut immediates = support::children::<Immediate>(node);
                let label = immediates.next()?;
                let label_types = resolve_br_types(service, document, symbol_table, &label);
                let rt_label =
                    if let Some(OperandType::Val(ValType::Ref(rt_label))) = label_types.last() {
                        rt_label
                    } else {
                        diagnostics.push(Diagnostic {
                            range: helpers::rowan_range_to_lsp_range(
                                line_index,
                                label.syntax().text_range(),
                            ),
                            severity: Some(DiagnosticSeverity::Error),
                            source: Some("wat".into()),
                            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                            message: "the last type of this label must be a ref type".into(),
                            ..Default::default()
                        });
                        return None;
                    };
                let rt1_node = immediates.next()?;
                let rt1 = RefType::from_green(&rt1_node.ref_type()?.syntax().green(), service)?;
                let rt2_node = immediates.next()?;
                let rt2 = RefType::from_green(&rt2_node.ref_type()?.syntax().green(), service)?;
                if !rt2.matches(&rt1, service, document, module_id) {
                    diagnostics.push(Diagnostic {
                        range: helpers::rowan_range_to_lsp_range(
                            line_index,
                            rt2_node.syntax().text_range(),
                        ),
                        severity: Some(DiagnosticSeverity::Error),
                        source: Some("wat".into()),
                        code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                        message: format!(
                            "ref type `{}` doesn't match the ref type `{}`",
                            rt2.render(service),
                            rt1.render(service),
                        ),
                        related_information: Some(vec![DiagnosticRelatedInformation {
                            location: Location {
                                uri: document.uri(service).raw(service),
                                range: helpers::rowan_range_to_lsp_range(
                                    line_index,
                                    rt1_node.syntax().text_range(),
                                ),
                            },
                            message: "should match this ref type".into(),
                        }]),
                        ..Default::default()
                    });
                }
                let rt_diff = rt1.diff(&rt2);
                if !rt_diff.matches(rt_label, service, document, module_id) {
                    diagnostics.push(Diagnostic {
                        range: helpers::rowan_range_to_lsp_range(line_index, node.text_range()),
                        severity: Some(DiagnosticSeverity::Error),
                        source: Some("wat".into()),
                        code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                        message: format!(
                            "type difference between given two ref types `{}` doesn't match the ref type `{}`",
                            rt_diff.render(service),
                            rt_label.render(service),
                        ),
                        related_information: Some(vec![DiagnosticRelatedInformation {
                            location: Location {
                                uri: document.uri(service).raw(service),
                                range: helpers::rowan_range_to_lsp_range(
                                    line_index,
                                    label.syntax().text_range(),
                                ),
                            },
                            message: "should match the last ref type in the result type of this label".into(),
                        }]),
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
    expected_kind: &'static str,
    ref_node: &SyntaxNode,
    service: &LanguageService,
    document: Document,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    def_types: &[DefType],
) -> Option<Diagnostic> {
    let def_symbol = symbol_table.find_def(SymbolKey::new(ref_node))?;
    let def_type = def_types
        .iter()
        .find(|def_type| def_type.key == def_symbol.key)?;
    let kind = match def_type.comp {
        CompositeType::Func(..) => "func",
        CompositeType::Struct(..) => "struct",
        CompositeType::Array(..) => "array",
    };
    if kind == expected_kind {
        None
    } else {
        Some(build_diagnostic(
            expected_kind,
            kind,
            ref_node,
            def_symbol,
            service,
            document,
            line_index,
        ))
    }
}

fn build_diagnostic(
    expected_kind: &'static str,
    actual_kind: &str,
    ref_node: &SyntaxNode,
    def_symbol: &Symbol,
    service: &LanguageService,
    document: Document,
    line_index: &LineIndex,
) -> Diagnostic {
    debug_assert!(matches!(expected_kind, "func" | "struct" | "array"));
    debug_assert!(matches!(actual_kind, "func" | "struct" | "array"));
    Diagnostic {
        range: helpers::rowan_range_to_lsp_range(line_index, ref_node.text_range()),
        severity: Some(DiagnosticSeverity::Error),
        source: Some("wat".into()),
        code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
        message: format!(
            "expected type is {expected_kind}, but type of `{}` is {actual_kind}",
            def_symbol.idx.render(service)
        ),
        related_information: Some(vec![DiagnosticRelatedInformation {
            location: Location {
                uri: document.uri(service).raw(service),
                range: helpers::rowan_range_to_lsp_range(line_index, def_symbol.key.text_range()),
            },
            message: format!("{actual_kind} type defined here"),
        }]),
        ..Default::default()
    }
}
