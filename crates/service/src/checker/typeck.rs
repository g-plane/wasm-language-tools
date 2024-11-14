use crate::{
    binder::{SymbolItemKind, SymbolTable},
    data_set::{self, OperandType},
    files::FilesCtx,
    helpers,
    types_analyzer::TypesAnalyzerCtx,
    InternUri, LanguageService,
};
use line_index::LineIndex;
use lsp_types::{Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location};
use rowan::{
    ast::{
        support::{children, token},
        AstNode,
    },
    TextRange,
};
use wat_syntax::{
    ast::{Instr, PlainInstr},
    SyntaxKind, SyntaxNode,
};

pub fn check_folded(
    diags: &mut Vec<Diagnostic>,
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    node: SyntaxNode,
    symbol_table: &SymbolTable,
) {
    let Some(instr) = PlainInstr::cast(node) else {
        return;
    };
    let Some(instr_name) = instr.instr_name() else {
        return;
    };
    if instr.l_paren_token().is_none() {
        return;
    }
    let is_call = instr_name.text() == "call";
    let meta = data_set::INSTR_METAS.get(instr_name.text());

    let skipped_count = if is_call {
        1
    } else {
        meta.map(|meta| meta.operands_count).unwrap_or_default()
    };
    let received_types =
        instr
            .operands()
            .skip(skipped_count)
            .fold(vec![], |mut received, operand| {
                if let Some(instr) = operand.instr() {
                    if let Some(types) = resolve_type(service, uri, symbol_table, &instr) {
                        received.extend(types.into_iter().map(|ty| (ty, operand.clone())));
                    }
                } else if meta.is_some() {
                    diags.push(Diagnostic {
                        range: helpers::rowan_range_to_lsp_range(
                            line_index,
                            operand.syntax().text_range(),
                        ),
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("wat".into()),
                        message: "expected instr".into(),
                        ..Default::default()
                    });
                }
                received
            });
    let Some(params) = resolve_expected_types(service, uri, symbol_table, &instr, meta) else {
        return;
    };

    let expected_count = params.len() + skipped_count;
    let received_count = received_types.len()
        + instr
            .operands()
            .filter(|operand| operand.instr().is_none())
            .count();
    if expected_count != received_count {
        diags.push(Diagnostic {
            range: helpers::rowan_range_to_lsp_range(line_index, instr.syntax().text_range()),
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("wat".into()),
            message: build_incorrect_operands_count_msg(expected_count, received_count),
            ..Default::default()
        });
    }

    let type_mismatches = params.iter().zip(received_types.iter()).filter_map(|pair| {
        if let ((OperandType::Val(expected), related), (OperandType::Val(received), operand)) = pair
        {
            if expected == received {
                None
            } else {
                Some(Diagnostic {
                    range: helpers::rowan_range_to_lsp_range(
                        line_index,
                        operand.syntax().text_range(),
                    ),
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("wat".into()),
                    message: format!("expected type `{expected}`, found `{received}`"),
                    related_information: related.as_ref().map(|(range, message)| {
                        vec![DiagnosticRelatedInformation {
                            location: Location {
                                uri: service.lookup_uri(uri),
                                range: helpers::rowan_range_to_lsp_range(line_index, *range),
                            },
                            message: message.clone(),
                        }]
                    }),
                    ..Default::default()
                })
            }
        } else {
            None
        }
    });
    diags.extend(type_mismatches);
}

pub fn check_stacked(
    diags: &mut Vec<Diagnostic>,
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    node: &SyntaxNode,
    symbol_table: &SymbolTable,
) {
    if node
        .children()
        .filter(|child| {
            matches!(
                child.kind(),
                SyntaxKind::PLAIN_INSTR
                    | SyntaxKind::BLOCK_BLOCK
                    | SyntaxKind::BLOCK_IF
                    | SyntaxKind::BLOCK_LOOP
            )
        })
        .all(|child| token(&child, SyntaxKind::L_PAREN).is_some())
    {
        return;
    }

    let mut types_stack = Vec::<(_, Instr)>::with_capacity(2);
    children::<Instr>(node).for_each(|instr| {
        if let Instr::Plain(plain_instr) = &instr {
            let Some(instr_name) = plain_instr.instr_name() else {
                return;
            };
            let meta = data_set::INSTR_METAS.get(instr_name.text());
            let Some(params) =
                resolve_expected_types(service, uri, symbol_table, plain_instr, meta)
            else {
                return;
            };
            let expected_count = params.len();
            let pop_count = if let Some(count) = types_stack.len().checked_sub(expected_count) {
                count
            } else {
                diags.push(Diagnostic {
                    range: helpers::rowan_range_to_lsp_range(
                        line_index,
                        instr.syntax().text_range(),
                    ),
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("wat".into()),
                    message: build_incorrect_operands_count_msg(expected_count, types_stack.len()),
                    ..Default::default()
                });
                0
            };
            let type_mismatches = params
                .iter()
                .zip(types_stack.drain(pop_count..))
                .filter_map(|pair| {
                    if let (
                        (OperandType::Val(expected), related),
                        (OperandType::Val(received), related_instr),
                    ) = pair
                    {
                        if expected == &received {
                            None
                        } else {
                            Some(Diagnostic {
                                range: helpers::rowan_range_to_lsp_range(
                                    line_index,
                                    related_instr.syntax().text_range(),
                                ),
                                severity: Some(DiagnosticSeverity::ERROR),
                                source: Some("wat".into()),
                                message: format!("expected type `{expected}`, found `{received}`"),
                                related_information: related.as_ref().map(|(range, message)| {
                                    vec![DiagnosticRelatedInformation {
                                        location: Location {
                                            uri: service.lookup_uri(uri),
                                            range: helpers::rowan_range_to_lsp_range(
                                                line_index, *range,
                                            ),
                                        },
                                        message: message.clone(),
                                    }]
                                }),
                                ..Default::default()
                            })
                        }
                    } else {
                        None
                    }
                });
            diags.extend(type_mismatches);

            if let Some(types) = resolve_type(service, uri, symbol_table, &instr) {
                types_stack.extend(types.into_iter().map(|ty| (ty, instr.clone())));
            }
        }
    });
}

fn resolve_type(
    service: &LanguageService,
    uri: InternUri,
    symbol_table: &SymbolTable,
    instr: &Instr,
) -> Option<Vec<OperandType>> {
    match instr {
        Instr::Block(..) => None,
        Instr::Plain(plain_instr) => {
            let instr_name = plain_instr.instr_name()?;
            match instr_name.text() {
                "call" => {
                    let idx = plain_instr.operands().next()?;
                    symbol_table
                        .find_func_defs(&idx.syntax().clone().into())
                        .into_iter()
                        .flatten()
                        .next()
                        .and_then(|func| service.get_func_sig(uri, func.clone().into()))
                        .map(|sig| {
                            sig.results
                                .iter()
                                .map(|ty| OperandType::Val(ty.clone()))
                                .collect()
                        })
                }
                "local.get" => {
                    let idx = plain_instr.operands().next()?;
                    symbol_table
                        .find_param_or_local_def(&idx.syntax().clone().into())
                        .and_then(|symbol| service.extract_type(symbol.green.clone()))
                        .map(|ty| vec![OperandType::Val(ty)])
                }
                "global.get" => {
                    let idx = plain_instr.operands().next()?;
                    symbol_table
                        .find_global_defs(&idx.syntax().clone().into())
                        .into_iter()
                        .flatten()
                        .next()
                        .and_then(|symbol| service.extract_global_type(symbol.green.clone()))
                        .map(|ty| vec![OperandType::Val(ty)])
                }
                _ => data_set::INSTR_METAS
                    .get(instr_name.text())
                    .map(|meta| meta.results.clone()),
            }
        }
    }
}

type ExpectedType = (OperandType, Option<(TextRange, String)>);
fn resolve_expected_types(
    service: &LanguageService,
    uri: InternUri,
    symbol_table: &SymbolTable,
    instr: &PlainInstr,
    meta: Option<&data_set::InstrMeta>,
) -> Option<Vec<ExpectedType>> {
    if instr.instr_name()?.text() == "call" {
        let idx = instr.operands().next()?;
        let func = symbol_table
            .find_func_defs(&idx.syntax().clone().into())
            .into_iter()
            .flatten()
            .next()?;
        let root = SyntaxNode::new_root(service.root(uri));
        let related = symbol_table
            .get_declared_params_and_locals(func.key.ptr.to_node(&root))
            .filter(|(symbol, _)| matches!(symbol.kind, SymbolItemKind::Param(..)))
            .map(|(symbol, _)| {
                Some((
                    symbol.key.ptr.text_range(),
                    "parameter originally defined here".into(),
                ))
            });
        service.get_func_sig(uri, func.clone().into()).map(|sig| {
            sig.params
                .iter()
                .map(|ty| OperandType::Val(ty.0.clone()))
                .zip(related)
                .collect()
        })
    } else {
        meta.map(|meta| {
            meta.params
                .iter()
                .map(|param| (param.clone(), None))
                .collect()
        })
    }
}

fn build_incorrect_operands_count_msg(expected: usize, received: usize) -> String {
    format!(
        "expected {expected} {}, found {received}",
        if expected == 1 { "operand" } else { "operands" },
    )
}
