use crate::{
    binder::{SymbolItemKey, SymbolItemKind, SymbolTable},
    data_set,
    files::FilesCtx,
    helpers,
    types_analyzer::{OperandType, TypesAnalyzerCtx, ValType},
    InternUri, LanguageService,
};
use itertools::{EitherOrBoth, Itertools};
use line_index::LineIndex;
use lsp_types::{
    Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, NumberOrString,
};
use rowan::{
    ast::{support, AstNode, SyntaxNodePtr},
    TextRange,
};
use wat_syntax::{
    ast::{BlockInstr, Instr, Operand, PlainInstr},
    SyntaxKind, SyntaxNode,
};

const DIAGNOSTIC_CODE: &str = "type-check";

pub fn check_func(
    diags: &mut Vec<Diagnostic>,
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    node: &SyntaxNode,
    symbol_table: &SymbolTable,
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
    node: &SyntaxNode,
    symbol_table: &SymbolTable,
) {
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
        &[service
            .extract_global_type(node.green().into())
            .map(OperandType::Val)
            .unwrap_or(OperandType::Any)],
    );
}

pub fn unfold(node: SyntaxNode, sequence: &mut Vec<Instr>) {
    match node.kind() {
        SyntaxKind::PLAIN_INSTR => node
            .children()
            .filter_map(|child| {
                if child.kind() == SyntaxKind::OPERAND {
                    child.first_child().and_then(Instr::cast)
                } else {
                    None
                }
            })
            .for_each(|child| unfold(child.syntax().clone(), sequence)),
        SyntaxKind::BLOCK_IF => node
            .children()
            .filter_map(Instr::cast)
            .for_each(|child| unfold(child.syntax().clone(), sequence)),
        _ => {}
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
    init_stack: Vec<(OperandType, Instr)>,
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
            let meta = data_set::INSTR_METAS.get(instr_name);
            let Some(params) = resolve_expected_types(shared, plain_instr, meta) else {
                return;
            };
            if let Some(diag) = type_stack.check(&params, ReportRange::Instr(&instr)) {
                diags.push(diag);
            }
            if let Some(types) = resolve_type(shared, plain_instr) {
                type_stack
                    .stack
                    .extend(types.into_iter().map(|ty| (ty, instr.clone())));
            }
            type_stack.has_never |= helpers::can_produce_never(instr_name);
        }
        Instr::Block(block_instr) => {
            let node = block_instr.syntax();
            let signature = node
                .children()
                .find(|child| child.kind() == SyntaxKind::BLOCK_TYPE)
                .and_then(|block_type| {
                    shared.service.get_func_sig(
                        shared.uri,
                        SyntaxNodePtr::new(&block_type),
                        block_type.green().into(),
                    )
                });
            let params = signature.as_ref().map(|signature| &signature.params);
            if let Some(diag) = params.and_then(|params| {
                type_stack.check(
                    &params
                        .iter()
                        .map(|(ty, ..)| (OperandType::Val(*ty), None))
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
                        .map(|(ty, ..)| (OperandType::Val(*ty), instr.clone()))
                        .collect()
                })
                .unwrap_or_default();
            let results = signature
                .map(|signature| {
                    signature
                        .results
                        .iter()
                        .map(|ty| OperandType::Val(*ty))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            match block_instr {
                BlockInstr::Block(..) | BlockInstr::Loop(..) => {
                    check_block_like(diags, shared, node, init_stack, &results);
                }
                BlockInstr::If(block_if) => {
                    if let Some(mut diag) = type_stack.check(
                        &[(OperandType::Val(ValType::I32), None)],
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
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("wat".into()),
                            code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
                            message: "missing `then` branch".into(),
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
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("wat".into()),
                            code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
                            message: "missing `else` branch".into(),
                            ..Default::default()
                        });
                    }
                }
            }
            type_stack
                .stack
                .extend(results.into_iter().map(|ty| (ty, instr.clone())));
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
    stack: Vec<(OperandType, Instr)>,
    has_never: bool,
}
impl TypeStack<'_> {
    fn check(
        &mut self,
        expected: &[ExpectedType],
        report_range: ReportRange,
    ) -> Option<Diagnostic> {
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
                    (OperandType::Val(expected), related),
                    (OperandType::Val(received), related_instr),
                ) if expected != received => {
                    mismatch = true;
                    related_information.push(DiagnosticRelatedInformation {
                        location: Location {
                            uri: self.service.lookup_uri(self.uri),
                            range: helpers::rowan_range_to_lsp_range(
                                self.line_index,
                                ReportRange::Instr(related_instr).pick(),
                            ),
                        },
                        message: format!("expected type `{expected}`, found `{received}`"),
                    });
                    if let Some((range, message)) = related {
                        related_information.push(DiagnosticRelatedInformation {
                            location: Location {
                                uri: self.service.lookup_uri(self.uri),
                                range: helpers::rowan_range_to_lsp_range(self.line_index, *range),
                            },
                            message: message.clone(),
                        });
                    }
                }
                EitherOrBoth::Left(..) if !self.has_never => {
                    mismatch = true;
                }
                _ => {}
            });
        if mismatch {
            let expected_types = format!("[{}]", expected.iter().map(|(ty, _)| ty).join(", "));
            let received_types = format!("[{}]", pops.iter().map(|(ty, _)| ty).join(", "));
            diagnostic = Some(Diagnostic {
                range: helpers::rowan_range_to_lsp_range(self.line_index, report_range.pick()),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("wat".into()),
                code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
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
        if self.has_never {
            return None;
        }
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
                    related_information.push(DiagnosticRelatedInformation {
                        location: Location {
                            uri: self.service.lookup_uri(self.uri),
                            range: helpers::rowan_range_to_lsp_range(
                                self.line_index,
                                ReportRange::Instr(related_instr).pick(),
                            ),
                        },
                        message: format!("expected type `{expected}`, found `{received}`"),
                    });
                }
                EitherOrBoth::Left(..) | EitherOrBoth::Right(..) => {
                    mismatch = true;
                }
                _ => {}
            });
        if mismatch {
            let expected_types = format!("[{}]", expected.iter().join(", "));
            let received_types = format!("[{}]", self.stack.iter().map(|(ty, _)| ty).join(", "));
            diagnostic = Some(Diagnostic {
                range: helpers::rowan_range_to_lsp_range(self.line_index, report_range.pick()),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("wat".into()),
                code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
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

fn resolve_type(shared: &Shared, plain_instr: &PlainInstr) -> Option<Vec<OperandType>> {
    let instr_name = plain_instr.instr_name()?;
    match instr_name.text() {
        "call" => {
            let idx = plain_instr.operands().next()?;
            shared
                .symbol_table
                .find_defs(SymbolItemKey::new(idx.syntax()))
                .into_iter()
                .flatten()
                .next()
                .and_then(|func| {
                    shared
                        .service
                        .get_func_sig(shared.uri, func.key, func.green.clone())
                })
                .map(|sig| sig.results.iter().map(|ty| OperandType::Val(*ty)).collect())
        }
        "local.get" => {
            let idx = plain_instr.operands().next()?;
            shared
                .symbol_table
                .find_param_or_local_def(SymbolItemKey::new(idx.syntax()))
                .and_then(|symbol| shared.service.extract_type(symbol.green.clone()))
                .map(OperandType::Val)
                .or(Some(OperandType::Never))
                .map(|ty| vec![ty])
        }
        "global.get" => {
            let idx = plain_instr.operands().next()?;
            shared
                .symbol_table
                .find_defs(SymbolItemKey::new(idx.syntax()))
                .into_iter()
                .flatten()
                .next()
                .and_then(|symbol| shared.service.extract_global_type(symbol.green.clone()))
                .map(OperandType::Val)
                .or(Some(OperandType::Never))
                .map(|ty| vec![ty])
        }
        "br" | "br_if" => plain_instr
            .operands()
            .next()
            .and_then(|idx| resolve_br_types(shared, idx))
            .map(|types| types.collect()),
        _ => data_set::INSTR_METAS
            .get(instr_name.text())
            .map(|meta| meta.results.clone()),
    }
}

type ExpectedType = (OperandType, Option<(TextRange, String)>);
fn resolve_expected_types(
    shared: &Shared,
    instr: &PlainInstr,
    meta: Option<&data_set::InstrMeta>,
) -> Option<Vec<ExpectedType>> {
    match instr.instr_name()?.text() {
        "call" => {
            let idx = instr.operands().next()?;
            let func = shared
                .symbol_table
                .find_defs(SymbolItemKey::new(idx.syntax()))
                .into_iter()
                .flatten()
                .next()?;
            let root = instr.syntax().ancestors().last()?;
            let related = shared
                .symbol_table
                .get_declared(func.key.to_node(&root), SymbolItemKind::Param)
                .map(|symbol| {
                    Some((
                        symbol.key.text_range(),
                        "parameter originally defined here".into(),
                    ))
                });
            shared
                .service
                .get_func_sig(shared.uri, func.key, func.green.clone())
                .map(|sig| {
                    sig.params
                        .iter()
                        .map(|(ty, ..)| OperandType::Val(*ty))
                        .zip(related)
                        .collect()
                })
        }
        "return" => {
            let func = instr
                .syntax()
                .ancestors()
                .find(|node| node.kind() == SyntaxKind::MODULE_FIELD_FUNC)?;
            shared
                .service
                .get_func_sig(shared.uri, SyntaxNodePtr::new(&func), func.green().into())
                .map(|sig| {
                    sig.results
                        .into_iter()
                        .map(|ty| (OperandType::Val(ty), None))
                        .collect()
                })
        }
        "br" => instr
            .operands()
            .next()
            .and_then(|idx| resolve_br_types(shared, idx))
            .map(|types| types.map(|ty| (ty, None)).collect()),
        "br_if" => {
            let mut types = instr
                .operands()
                .next()
                .and_then(|idx| resolve_br_types(shared, idx))
                .map(|types| types.map(|ty| (ty, None)).collect::<Vec<_>>())
                .unwrap_or_default();
            types.push((OperandType::Val(ValType::I32), None));
            Some(types)
        }
        _ => meta.map(|meta| {
            meta.params
                .iter()
                .map(|param| (param.clone(), None))
                .collect()
        }),
    }
}
fn resolve_br_types(shared: &Shared, idx: Operand) -> Option<impl Iterator<Item = OperandType>> {
    let key = SymbolItemKey::new(idx.syntax());
    shared
        .symbol_table
        .blocks
        .iter()
        .find(|block| block.ref_key == key)
        .and_then(|block| {
            block
                .def_key
                .to_node(&SyntaxNode::new_root(shared.service.root(shared.uri)))
                .children()
                .find(|child| child.kind() == SyntaxKind::BLOCK_TYPE)
        })
        .and_then(|block_type| {
            shared.service.get_func_sig(
                shared.uri,
                SyntaxNodePtr::new(&block_type),
                block_type.green().into(),
            )
        })
        .map(|sig| sig.results.into_iter().map(OperandType::Val))
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
