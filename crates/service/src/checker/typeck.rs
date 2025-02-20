use crate::{
    binder::{SymbolKey, SymbolTable},
    data_set, helpers,
    types_analyzer::{
        get_block_sig, resolve_br_types, OperandType, ResolvedSig, TypesAnalyzerCtx, ValType,
    },
    uri::{InternUri, UrisCtx},
    LanguageService,
};
use itertools::{EitherOrBoth, Itertools};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, Union2};
use rowan::{
    ast::{support, AstNode},
    TextRange,
};
use wat_syntax::{
    ast::{BlockInstr, Import, Instr, PlainInstr},
    SyntaxKind, SyntaxNode, SyntaxNodePtr,
};

const DIAGNOSTIC_CODE: &str = "type-check";

pub fn check_func(
    diags: &mut Vec<Diagnostic>,
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) {
    let results = service
        .get_func_sig(uri, SyntaxNodePtr::new(node), node.green().into())
        .map(|sig| {
            sig.results
                .into_iter()
                .map(OperandType::Val)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    check_block_like(
        diags,
        &Shared {
            service,
            uri,
            symbol_table,
            line_index,
        },
        node,
        Vec::with_capacity(2),
        &results,
    );
}

pub fn check_global(
    diags: &mut Vec<Diagnostic>,
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) {
    let ty = service
        .extract_global_type(node.green().into())
        .map(OperandType::Val)
        .unwrap_or(OperandType::Any);
    check_block_like(
        diags,
        &Shared {
            service,
            uri,
            symbol_table,
            line_index,
        },
        node,
        if support::child::<Import>(node).is_some() {
            vec![(ty.clone(), None)]
        } else {
            Vec::with_capacity(1)
        },
        &[ty],
    );
}

pub fn unfold(node: SyntaxNode, sequence: &mut Vec<Instr>) {
    if matches!(node.kind(), SyntaxKind::PLAIN_INSTR | SyntaxKind::BLOCK_IF) {
        node.children()
            .filter_map(Instr::cast)
            .for_each(|child| unfold(child.syntax().clone(), sequence));
    }
    if let Some(node) = Instr::cast(node) {
        sequence.push(node);
    }
}

struct Shared<'a> {
    service: &'a LanguageService,
    uri: InternUri,
    symbol_table: &'a SymbolTable,
    line_index: &'a LineIndex,
}

fn check_block_like(
    diags: &mut Vec<Diagnostic>,
    shared: &Shared,
    node: &SyntaxNode,
    init_stack: Vec<(OperandType, Option<Instr>)>,
    expected_results: &[OperandType],
) {
    let mut type_stack = TypeStack {
        uri: shared.uri,
        service: shared.service,
        line_index: shared.line_index,
        stack: init_stack,
        has_never: false,
    };
    let mut sequence = Vec::with_capacity(1);
    node.children()
        .filter(|child| Instr::can_cast(child.kind()))
        .for_each(|child| unfold(child, &mut sequence));

    sequence.into_iter().for_each(|instr| match &instr {
        Instr::Plain(plain_instr) => {
            let Some(instr_name) = plain_instr.instr_name() else {
                return;
            };
            let instr_name = instr_name.text();
            let sig = resolve_sig(shared, instr_name, plain_instr, &type_stack);
            if let Some(diag) = type_stack.check(&sig.params, ReportRange::Instr(&instr)) {
                diags.push(diag);
            }
            type_stack
                .stack
                .extend(sig.results.into_iter().map(|ty| (ty, Some(instr.clone()))));
            if helpers::can_produce_never(instr_name) {
                type_stack.has_never = true;
                type_stack.stack.clear();
            }
        }
        Instr::Block(block_instr) => {
            let node = block_instr.syntax();
            let signature = get_block_sig(shared.service, shared.uri, node);
            let params = signature.as_ref().map(|signature| &signature.params);
            if let Some(diag) = params.and_then(|params| {
                type_stack.check(
                    &params
                        .iter()
                        .map(|(ty, _)| OperandType::Val(*ty))
                        .collect::<Vec<_>>(),
                    ReportRange::Instr(&instr),
                )
            }) {
                diags.push(diag);
            };
            let init_stack = params
                .map(|params| {
                    params
                        .iter()
                        .map(|(ty, ..)| (OperandType::Val(*ty), Some(instr.clone())))
                        .collect()
                })
                .unwrap_or_default();
            let results = signature
                .map(|signature| {
                    signature
                        .results
                        .into_iter()
                        .map(OperandType::Val)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            match block_instr {
                BlockInstr::Block(..) | BlockInstr::Loop(..) => {
                    check_block_like(diags, shared, node, init_stack, &results);
                }
                BlockInstr::If(block_if) => {
                    if let Some(mut diag) = type_stack.check(
                        &[OperandType::Val(ValType::I32)],
                        ReportRange::Keyword(node),
                    ) {
                        diag.message.push_str(" for the condition of `if` block");
                        diags.push(diag);
                    }
                    if let Some(then_block) = block_if.then_block() {
                        check_block_like(
                            diags,
                            shared,
                            then_block.syntax(),
                            init_stack.clone(),
                            &results,
                        );
                    } else {
                        diags.push(Diagnostic {
                            range: helpers::rowan_range_to_lsp_range(
                                shared.line_index,
                                node.text_range(),
                            ),
                            severity: Some(DiagnosticSeverity::Error),
                            source: Some("wat".into()),
                            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                            message: format!(
                                "missing `then` branch with expected types [{}]",
                                results
                                    .iter()
                                    .map(|ty| ty.render(shared.service))
                                    .join(", ")
                            ),
                            ..Default::default()
                        });
                    }
                    if let Some(else_block) = block_if.else_block() {
                        check_block_like(diags, shared, else_block.syntax(), init_stack, &results);
                    } else if !results.is_empty() {
                        diags.push(Diagnostic {
                            range: helpers::rowan_range_to_lsp_range(
                                shared.line_index,
                                node.text_range(),
                            ),
                            severity: Some(DiagnosticSeverity::Error),
                            source: Some("wat".into()),
                            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                            message: format!(
                                "missing `else` branch with expected types [{}]",
                                results
                                    .iter()
                                    .map(|ty| ty.render(shared.service))
                                    .join(", ")
                            ),
                            ..Default::default()
                        });
                    }
                }
            }
            type_stack
                .stack
                .extend(results.into_iter().map(|ty| (ty, Some(instr.clone()))));
        }
    });
    if let Some(diag) = type_stack.check_to_bottom(expected_results, ReportRange::Last(node)) {
        diags.push(diag);
    }
}

struct TypeStack<'a> {
    uri: InternUri,
    service: &'a LanguageService,
    line_index: &'a LineIndex,
    stack: Vec<(OperandType, Option<Instr>)>,
    has_never: bool,
}
impl TypeStack<'_> {
    fn check(&mut self, expected: &[OperandType], report_range: ReportRange) -> Option<Diagnostic> {
        let service = self.service;
        let mut diagnostic = None;
        let rest_len = self.stack.len().saturating_sub(expected.len());
        let pops = self.stack.get(rest_len..).unwrap_or(&*self.stack);
        let mut mismatch = false;
        let mut related_information = vec![];
        expected
            .iter()
            .rev()
            .zip_longest(pops.iter().rev())
            .for_each(|pair| match pair {
                EitherOrBoth::Both(
                    OperandType::Val(expected),
                    (OperandType::Val(received), related_instr),
                ) if !service.value_type_matches(self.uri, *received, *expected) => {
                    mismatch = true;
                    if let Some(related_instr) = related_instr {
                        related_information.push(DiagnosticRelatedInformation {
                            location: Location {
                                uri: service.lookup_uri(self.uri),
                                range: helpers::rowan_range_to_lsp_range(
                                    self.line_index,
                                    ReportRange::Instr(related_instr).pick(),
                                ),
                            },
                            message: format!(
                                "expected type `{}`, found `{}`",
                                expected.render_compact(service),
                                received.render_compact(service),
                            ),
                        });
                    }
                }
                EitherOrBoth::Left(..) if !self.has_never => {
                    mismatch = true;
                }
                _ => {}
            });
        if mismatch {
            let expected_types = format!(
                "[{}]",
                expected.iter().map(|ty| ty.render(service)).join(", ")
            );
            let received_types = format!(
                "[{}]",
                pops.iter().map(|(ty, _)| ty.render(service)).join(", ")
            );
            diagnostic = Some(Diagnostic {
                range: helpers::rowan_range_to_lsp_range(self.line_index, report_range.pick()),
                severity: Some(DiagnosticSeverity::Error),
                source: Some("wat".into()),
                code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                message: format!("expected types {expected_types}, found {received_types}"),
                related_information: if related_information.is_empty() {
                    None
                } else {
                    Some(related_information)
                },
                ..Default::default()
            });
        }
        self.stack.truncate(rest_len);
        diagnostic
    }

    fn check_to_bottom(
        &mut self,
        expected: &[OperandType],
        report_range: ReportRange,
    ) -> Option<Diagnostic> {
        let mut diagnostic = None;
        let mut mismatch = false;
        let mut related_information = vec![];
        expected
            .iter()
            .rev()
            .zip_longest(self.stack.iter().rev())
            .for_each(|pair| match pair {
                EitherOrBoth::Both(
                    OperandType::Val(expected),
                    (OperandType::Val(received), related_instr),
                ) if expected != received => {
                    mismatch = true;
                    if let Some(related_instr) = related_instr {
                        related_information.push(DiagnosticRelatedInformation {
                            location: Location {
                                uri: self.service.lookup_uri(self.uri),
                                range: helpers::rowan_range_to_lsp_range(
                                    self.line_index,
                                    ReportRange::Instr(related_instr).pick(),
                                ),
                            },
                            message: format!(
                                "expected type `{}`, found `{}`",
                                expected.render_compact(self.service),
                                received.render_compact(self.service),
                            ),
                        });
                    }
                }
                EitherOrBoth::Left(..) if !self.has_never => {
                    mismatch = true;
                }
                EitherOrBoth::Right(..) => {
                    mismatch = true;
                }
                _ => {}
            });
        if mismatch {
            let expected_types = format!(
                "[{}]",
                expected.iter().map(|ty| ty.render(self.service)).join(", ")
            );
            let received_types = format!(
                "[{}]",
                self.stack
                    .iter()
                    .map(|(ty, _)| ty.render(self.service))
                    .join(", ")
            );
            diagnostic = Some(Diagnostic {
                range: helpers::rowan_range_to_lsp_range(self.line_index, report_range.pick()),
                severity: Some(DiagnosticSeverity::Error),
                source: Some("wat".into()),
                code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                message: format!(
                    "expected types {expected_types}, found {received_types}{}",
                    if let ReportRange::Last(..) = report_range {
                        " at the end"
                    } else {
                        ""
                    }
                ),
                related_information: if related_information.is_empty() {
                    None
                } else {
                    Some(related_information)
                },
                ..Default::default()
            });
        }
        self.stack.clear();
        diagnostic
    }
}

fn resolve_sig(
    shared: &Shared,
    instr_name: &str,
    instr: &PlainInstr,
    type_stack: &TypeStack,
) -> ResolvedSig {
    match instr_name {
        "call" => instr
            .immediates()
            .next()
            .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
            .and_then(|func| {
                shared
                    .service
                    .get_func_sig(shared.uri, func.key, func.green.clone())
            })
            .map(ResolvedSig::from)
            .unwrap_or_default(),
        "local.get" => ResolvedSig {
            params: vec![],
            results: vec![instr
                .immediates()
                .next()
                .and_then(|idx| {
                    shared
                        .symbol_table
                        .find_param_or_local_def(SymbolKey::new(idx.syntax()))
                })
                .and_then(|symbol| shared.service.extract_type(symbol.green.clone()))
                .map_or(OperandType::Any, OperandType::Val)],
        },
        "global.get" => ResolvedSig {
            params: vec![],
            results: vec![instr
                .immediates()
                .next()
                .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
                .and_then(|symbol| shared.service.extract_global_type(symbol.green.clone()))
                .map_or(OperandType::Any, OperandType::Val)],
        },
        "return" => ResolvedSig {
            params: instr
                .syntax()
                .ancestors()
                .find(|node| node.kind() == SyntaxKind::MODULE_FIELD_FUNC)
                .and_then(|func| {
                    shared.service.get_func_sig(
                        shared.uri,
                        SyntaxNodePtr::new(&func),
                        func.green().into(),
                    )
                })
                .map(|sig| sig.results.into_iter().map(OperandType::Val).collect())
                .unwrap_or_default(),
            results: vec![],
        },
        "br" => ResolvedSig {
            params: instr
                .immediates()
                .next()
                .map(|idx| resolve_br_types(shared.service, shared.uri, shared.symbol_table, &idx))
                .unwrap_or_default(),
            results: vec![],
        },
        "br_if" => {
            let results = instr
                .immediates()
                .next()
                .map(|idx| resolve_br_types(shared.service, shared.uri, shared.symbol_table, &idx))
                .unwrap_or_default();
            let mut params = results.clone();
            params.push(OperandType::Val(ValType::I32));
            ResolvedSig { params, results }
        }
        "br_table" => {
            let mut params = instr
                .immediates()
                .next()
                .map(|idx| resolve_br_types(shared.service, shared.uri, shared.symbol_table, &idx))
                .unwrap_or_default();
            params.push(OperandType::Val(ValType::I32));
            ResolvedSig {
                params,
                results: vec![],
            }
        }
        "select" => {
            let ty = if let Some(ty) = instr
                .immediates()
                .next()
                .and_then(|immediate| immediate.type_use())
                .and_then(|type_use| type_use.results().next())
                .and_then(|result| result.val_types().next())
            {
                ValType::from_green(&ty.syntax().green(), shared.service)
                    .map_or(OperandType::Any, OperandType::Val)
            } else {
                type_stack
                    .stack
                    .len()
                    .checked_sub(2)
                    .and_then(|i| type_stack.stack.get(i))
                    .map_or(OperandType::Any, |(ty, _)| ty.clone())
            };
            ResolvedSig {
                params: vec![ty.clone(), ty.clone(), OperandType::Val(ValType::I32)],
                results: vec![ty],
            }
        }
        "call_indirect" => {
            let sig = instr
                .immediates()
                .find_map(|immediate| immediate.type_use())
                .and_then(|type_use| {
                    let node = type_use.syntax();
                    shared.service.get_type_use_sig(
                        shared.uri,
                        SyntaxNodePtr::new(node),
                        node.green().into(),
                    )
                })
                .unwrap_or_default();
            let mut sig = ResolvedSig::from(sig);
            sig.params.push(OperandType::Val(ValType::I32));
            sig
        }
        _ => data_set::INSTR_SIG
            .get(instr_name)
            .cloned()
            .unwrap_or_default(),
    }
}

enum ReportRange<'a> {
    Instr(&'a Instr),
    Keyword(&'a SyntaxNode),
    Last(&'a SyntaxNode),
}
impl ReportRange<'_> {
    fn pick(&self) -> TextRange {
        match self {
            ReportRange::Instr(instr) => match instr {
                Instr::Plain(plain_instr) => plain_instr.syntax().text_range(),
                Instr::Block(block_instr) => block_instr
                    .syntax()
                    .children()
                    .find(|child| child.kind() == SyntaxKind::BLOCK_TYPE)
                    .map(|block_type| block_type.text_range())
                    .unwrap_or_else(|| block_instr.syntax().text_range()),
            },
            ReportRange::Keyword(node) => support::token(node, SyntaxKind::KEYWORD)
                .map(|token| token.text_range())
                .unwrap_or_else(|| node.text_range()),
            ReportRange::Last(node) => node
                .last_child_or_token()
                .map(|it| it.text_range())
                .unwrap_or_else(|| node.text_range()),
        }
    }
}
