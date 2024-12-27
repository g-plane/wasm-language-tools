use crate::{
    binder::{SymbolItemKind, SymbolTable},
    data_set,
    files::FilesCtx,
    helpers,
    types_analyzer::{OperandType, TypesAnalyzerCtx},
    InternUri, LanguageService,
};
use line_index::LineIndex;
use lsp_types::{
    Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, NumberOrString,
};
use rowan::{ast::AstNode, TextRange};
use wat_syntax::{
    ast::{BlockInstr, Instr, PlainInstr},
    SyntaxKind, SyntaxNode,
};

const DIAGNOSTIC_CODE: &str = "type-check";

pub fn check(
    diags: &mut Vec<Diagnostic>,
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    node: &SyntaxNode,
    symbol_table: &SymbolTable,
) {
    let mut sequence = Vec::with_capacity(1);
    node.children()
        .filter(|child| Instr::can_cast(child.kind()))
        .for_each(|child| unfold(child, &mut sequence));
    check_sequence(diags, service, uri, line_index, symbol_table, sequence);
}

pub fn unfold(node: SyntaxNode, stack: &mut Vec<Instr>) {
    node.children()
        .filter_map(|child| child.first_child().and_then(Instr::cast))
        .for_each(|child| unfold(child.syntax().clone(), stack));
    if let Some(node) = Instr::cast(node) {
        stack.push(node);
    }
}

fn check_sequence(
    diags: &mut Vec<Diagnostic>,
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    sequence: Vec<Instr>,
) {
    let mut types_stack = Vec::<(_, Instr)>::with_capacity(2);
    sequence.into_iter().for_each(|instr| match &instr {
        Instr::Plain(plain_instr) => {
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
                    code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
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
                                code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
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
        Instr::Block(block_instr) => {
            if let Some(types) = resolve_block_type(service, block_instr) {
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
        Instr::Block(block_instr) => resolve_block_type(service, block_instr),
        Instr::Plain(plain_instr) => {
            let instr_name = plain_instr.instr_name()?;
            match instr_name.text() {
                "call" => {
                    let idx = plain_instr.operands().next()?;
                    symbol_table
                        .find_defs(&idx.syntax().clone().into())
                        .into_iter()
                        .flatten()
                        .next()
                        .and_then(|func| service.get_func_sig(uri, func.clone().into()))
                        .map(|sig| sig.results.iter().map(|ty| OperandType::Val(*ty)).collect())
                }
                "local.get" => {
                    let idx = plain_instr.operands().next()?;
                    symbol_table
                        .find_param_or_local_def(&idx.syntax().clone().into())
                        .and_then(|symbol| service.extract_type(symbol.green.clone()))
                        .map(OperandType::Val)
                        .or(Some(OperandType::Never))
                        .map(|ty| vec![ty])
                }
                "global.get" => {
                    let idx = plain_instr.operands().next()?;
                    symbol_table
                        .find_defs(&idx.syntax().clone().into())
                        .into_iter()
                        .flatten()
                        .next()
                        .and_then(|symbol| service.extract_global_type(symbol.green.clone()))
                        .map(OperandType::Val)
                        .or(Some(OperandType::Never))
                        .map(|ty| vec![ty])
                }
                _ => data_set::INSTR_METAS
                    .get(instr_name.text())
                    .map(|meta| meta.results.clone()),
            }
        }
    }
}

fn resolve_block_type(
    service: &LanguageService,
    block_instr: &BlockInstr,
) -> Option<Vec<OperandType>> {
    block_instr
        .syntax()
        .children()
        .find(|child| child.kind() == SyntaxKind::BLOCK_TYPE)
        .map(|block_type| {
            service
                .extract_block_type(block_type.green().into())
                .into_iter()
                .map(OperandType::Val)
                .collect()
        })
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
            .find_defs(&idx.syntax().clone().into())
            .into_iter()
            .flatten()
            .next()?;
        let root = instr.syntax().ancestors().last()?;
        let related = symbol_table
            .get_declared(func.key.ptr.to_node(&root), SymbolItemKind::Param)
            .map(|symbol| {
                Some((
                    symbol.key.ptr.text_range(),
                    "parameter originally defined here".into(),
                ))
            });
        service.get_func_sig(uri, func.clone().into()).map(|sig| {
            sig.params
                .iter()
                .map(|ty| OperandType::Val(ty.0))
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
