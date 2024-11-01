use crate::{
    binder::SymbolTable,
    data_set::{self, OperandType},
    helpers,
    types_analyzer::TypesAnalyzerCtx,
    InternUri, LanguageService,
};
use line_index::LineIndex;
use lsp_types::{Diagnostic, DiagnosticSeverity};
use rowan::ast::AstNode;
use wat_syntax::{
    ast::{Instr, PlainInstr},
    SyntaxNode,
};

pub fn check(
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
                } else {
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
    let params = if is_call {
        let Some(idx) = instr.operands().next() else {
            return;
        };
        if let Some(sig) = symbol_table
            .find_func_defs(&idx.syntax().clone().into())
            .into_iter()
            .flatten()
            .next()
            .and_then(|func| service.get_func_sig(uri, func.clone().into()))
        {
            sig.params
                .iter()
                .map(|ty| OperandType::Val(ty.0.clone()))
                .collect()
        } else {
            return;
        }
    } else if let Some(meta) = meta {
        meta.params.clone()
    } else {
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
            message: format!(
                "expected {expected_count} {}, found {received_count}",
                if expected_count == 1 {
                    "operand"
                } else {
                    "operands"
                },
            ),
            ..Default::default()
        });
    }

    let type_mismatches = params.iter().zip(received_types.iter()).filter_map(|pair| {
        if let (OperandType::Val(expected), (OperandType::Val(received), operand)) = pair {
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
                    ..Default::default()
                })
            }
        } else {
            None
        }
    });
    diags.extend(type_mismatches);
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
