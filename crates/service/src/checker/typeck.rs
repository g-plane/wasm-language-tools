use crate::{
    LanguageService,
    binder::{SymbolKey, SymbolTable},
    data_set,
    document::Document,
    helpers,
    idx::{Idx, InternIdent},
    types_analyzer::{
        CompositeType, HeapType, OperandType, RefType, ResolvedSig, ValType, extract_global_type,
        extract_type, get_block_sig, get_def_types, get_func_sig, get_type_use_sig,
        operand_type_matches, resolve_array_type_with_idx, resolve_br_types,
        resolve_field_type_with_struct_idx,
    },
};
use itertools::{EitherOrBoth, Itertools};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, Union2};
use rowan::{
    TextRange,
    ast::{AstNode, support},
};
use wat_syntax::{
    SyntaxElement, SyntaxKind, SyntaxNode, SyntaxNodePtr,
    ast::{BlockInstr, ElemList, Import, Instr, ModuleFieldFunc, ModuleFieldTable, PlainInstr},
};

const DIAGNOSTIC_CODE: &str = "type-check";

pub fn check_func(
    diagnostics: &mut Vec<Diagnostic>,
    service: &LanguageService,
    document: Document,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    module_id: u32,
    node: &SyntaxNode,
) {
    let results = get_func_sig(
        service,
        document,
        SyntaxNodePtr::new(node),
        node.green().into(),
    )
    .results
    .into_iter()
    .map(OperandType::Val)
    .collect::<Vec<_>>();
    check_block_like(
        diagnostics,
        &Shared {
            service,
            document,
            symbol_table,
            line_index,
            module_id,
        },
        node,
        Vec::with_capacity(2),
        &results,
    );
}

pub fn check_global(
    diagnostics: &mut Vec<Diagnostic>,
    service: &LanguageService,
    document: Document,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    module_id: u32,
    node: &SyntaxNode,
) {
    let ty = extract_global_type(service, node.green().into())
        .map(OperandType::Val)
        .unwrap_or(OperandType::Any);
    check_block_like(
        diagnostics,
        &Shared {
            service,
            document,
            symbol_table,
            line_index,
            module_id,
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

pub fn check_table(
    diagnostics: &mut Vec<Diagnostic>,
    service: &LanguageService,
    document: Document,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    module_id: u32,
    node: &SyntaxNode,
) {
    let Some(ref_type) = ModuleFieldTable::cast(node.clone())
        .filter(|table| table.instrs().count() > 0) // expr is required only since WasmGC proposal
        .and_then(|table| {
            table.ref_type().or_else(|| {
                table
                    .table_type()
                    .and_then(|table_type| table_type.ref_type())
            })
        })
        .and_then(|ref_type| RefType::from_green(&ref_type.syntax().green(), service))
    else {
        return;
    };
    let ty = OperandType::Val(ValType::Ref(ref_type));
    check_block_like(
        diagnostics,
        &Shared {
            service,
            document,
            symbol_table,
            line_index,
            module_id,
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

pub fn check_offset(
    diagnostics: &mut Vec<Diagnostic>,
    service: &LanguageService,
    document: Document,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    module_id: u32,
    node: &SyntaxNode,
) {
    check_block_like(
        diagnostics,
        &Shared {
            service,
            document,
            symbol_table,
            line_index,
            module_id,
        },
        node,
        Vec::with_capacity(1),
        &[OperandType::Val(ValType::I32)],
    );
}

pub fn check_elem_list(
    diagnostics: &mut Vec<Diagnostic>,
    service: &LanguageService,
    document: Document,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    module_id: u32,
    node: &SyntaxNode,
) {
    let Some(ref_type) = ElemList::cast(node.clone())
        .and_then(|elem_list| elem_list.ref_type())
        .and_then(|ref_type| RefType::from_green(&ref_type.syntax().green(), service))
    else {
        return;
    };
    let ty = OperandType::Val(ValType::Ref(ref_type));
    node.children()
        .filter(|child| child.kind() == SyntaxKind::ELEM_EXPR)
        .for_each(|child| {
            check_block_like(
                diagnostics,
                &Shared {
                    service,
                    document,
                    symbol_table,
                    line_index,
                    module_id,
                },
                &child,
                Vec::with_capacity(1),
                std::slice::from_ref(&ty),
            );
        });
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

struct Shared<'db> {
    service: &'db dyn salsa::Database,
    document: Document,
    symbol_table: &'db SymbolTable<'db>,
    line_index: &'db LineIndex,
    module_id: u32,
}

fn check_block_like(
    diagnostics: &mut Vec<Diagnostic>,
    shared: &Shared,
    node: &SyntaxNode,
    init_stack: Vec<(OperandType, Option<Instr>)>,
    expected_results: &[OperandType],
) {
    let mut type_stack = TypeStack {
        document: shared.document,
        service: shared.service,
        line_index: shared.line_index,
        module_id: shared.module_id,
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
            if let Some(diagnostic) = type_stack.check(&sig.params, ReportRange::Instr(&instr)) {
                diagnostics.push(diagnostic);
            }
            if helpers::can_produce_never(instr_name) {
                type_stack.has_never = true;
                type_stack.stack.clear();
            }
            type_stack
                .stack
                .extend(sig.results.into_iter().map(|ty| (ty, Some(instr.clone()))));
        }
        Instr::Block(block_instr) => {
            let node = block_instr.syntax();
            let signature = get_block_sig(shared.service, shared.document, node);
            if let Some(diagnostic) = type_stack.check(
                &signature
                    .params
                    .iter()
                    .map(|(ty, _)| OperandType::Val(ty.clone()))
                    .collect::<Vec<_>>(),
                ReportRange::Instr(&instr),
            ) {
                diagnostics.push(diagnostic);
            };
            let init_stack = signature
                .params
                .into_iter()
                .map(|(ty, ..)| (OperandType::Val(ty), Some(instr.clone())))
                .collect();
            let results = signature
                .results
                .into_iter()
                .map(OperandType::Val)
                .collect::<Vec<_>>();
            match block_instr {
                BlockInstr::Block(..) | BlockInstr::Loop(..) => {
                    check_block_like(diagnostics, shared, node, init_stack, &results);
                }
                BlockInstr::If(block_if) => {
                    if let Some(mut diagnostic) = type_stack.check(
                        &[OperandType::Val(ValType::I32)],
                        ReportRange::Keyword(node),
                    ) {
                        diagnostic
                            .message
                            .push_str(" for the condition of `if` block");
                        diagnostics.push(diagnostic);
                    }
                    if let Some(then_block) = block_if.then_block() {
                        check_block_like(
                            diagnostics,
                            shared,
                            then_block.syntax(),
                            init_stack.clone(),
                            &results,
                        );
                    } else {
                        diagnostics.push(Diagnostic {
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
                        check_block_like(
                            diagnostics,
                            shared,
                            else_block.syntax(),
                            init_stack,
                            &results,
                        );
                    } else if !results.is_empty() {
                        diagnostics.push(Diagnostic {
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
    if let Some(diagnostic) = type_stack.check_to_bottom(expected_results, ReportRange::Last(node))
    {
        diagnostics.push(diagnostic);
    }
}

struct TypeStack<'db> {
    document: Document,
    service: &'db dyn salsa::Database,
    line_index: &'db LineIndex,
    module_id: u32,
    stack: Vec<(OperandType<'db>, Option<Instr>)>,
    has_never: bool,
}
impl<'db> TypeStack<'db> {
    fn check(
        &mut self,
        expected: &[OperandType<'db>],
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
                EitherOrBoth::Both(expected, (received, related_instr)) => {
                    if operand_type_matches(
                        self.service,
                        self.document,
                        self.module_id,
                        received.clone(),
                        expected.clone(),
                    ) {
                        return;
                    }
                    mismatch = true;
                    if let Some(related_instr) = related_instr {
                        related_information.push(DiagnosticRelatedInformation {
                            location: Location {
                                uri: self.document.uri(self.service).raw(self.service),
                                range: helpers::rowan_range_to_lsp_range(
                                    self.line_index,
                                    ReportRange::Instr(related_instr).pick(),
                                ),
                            },
                            message: format!(
                                "expected type `{}`, found `{}`",
                                expected.render(self.service),
                                received.render(self.service),
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
                expected.iter().map(|ty| ty.render(self.service)).join(", ")
            );
            let received_types = format!(
                "[{}]",
                pops.iter()
                    .map(|(ty, _)| ty.render(self.service))
                    .join(", ")
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
        expected: &[OperandType<'db>],
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
                EitherOrBoth::Both(expected, (received, related_instr)) => {
                    if operand_type_matches(
                        self.service,
                        self.document,
                        self.module_id,
                        received.clone(),
                        expected.clone(),
                    ) {
                        return;
                    }
                    mismatch = true;
                    if let Some(related_instr) = related_instr {
                        related_information.push(DiagnosticRelatedInformation {
                            location: Location {
                                uri: self.document.uri(self.service).raw(self.service),
                                range: helpers::rowan_range_to_lsp_range(
                                    self.line_index,
                                    ReportRange::Instr(related_instr).pick(),
                                ),
                            },
                            message: format!(
                                "expected type `{}`, found `{}`",
                                expected.render(self.service),
                                received.render(self.service),
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

fn resolve_sig<'db>(
    shared: &Shared<'db>,
    instr_name: &str,
    instr: &PlainInstr,
    type_stack: &TypeStack<'db>,
) -> ResolvedSig<'db> {
    match instr_name {
        "call" | "return_call" => instr
            .immediates()
            .next()
            .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
            .map(|func| {
                ResolvedSig::from(get_func_sig(
                    shared.service,
                    shared.document,
                    func.key,
                    func.green.clone(),
                ))
            })
            .unwrap_or_default(),
        "local.get" => ResolvedSig {
            params: vec![],
            results: vec![
                instr
                    .immediates()
                    .next()
                    .and_then(|idx| {
                        shared
                            .symbol_table
                            .find_param_or_local_def(SymbolKey::new(idx.syntax()))
                    })
                    .and_then(|symbol| extract_type(shared.service, symbol.green.clone()))
                    .map_or(OperandType::Any, OperandType::Val),
            ],
        },
        "local.set" => ResolvedSig {
            params: vec![
                instr
                    .immediates()
                    .next()
                    .and_then(|idx| {
                        shared
                            .symbol_table
                            .find_param_or_local_def(SymbolKey::new(idx.syntax()))
                    })
                    .and_then(|symbol| extract_type(shared.service, symbol.green.clone()))
                    .map_or(OperandType::Any, OperandType::Val),
            ],
            results: vec![],
        },
        "local.tee" => {
            let ty = instr
                .immediates()
                .next()
                .and_then(|idx| {
                    shared
                        .symbol_table
                        .find_param_or_local_def(SymbolKey::new(idx.syntax()))
                })
                .and_then(|symbol| extract_type(shared.service, symbol.green.clone()))
                .map_or(OperandType::Any, OperandType::Val);
            ResolvedSig {
                params: vec![ty.clone()],
                results: vec![ty],
            }
        }
        "global.get" => ResolvedSig {
            params: vec![],
            results: vec![
                instr
                    .immediates()
                    .next()
                    .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
                    .and_then(|symbol| extract_global_type(shared.service, symbol.green.clone()))
                    .map_or(OperandType::Any, OperandType::Val),
            ],
        },
        "global.set" => ResolvedSig {
            params: vec![
                instr
                    .immediates()
                    .next()
                    .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
                    .and_then(|symbol| extract_global_type(shared.service, symbol.green.clone()))
                    .map_or(OperandType::Any, OperandType::Val),
            ],
            results: vec![],
        },
        "return" => ResolvedSig {
            params: instr
                .syntax()
                .ancestors()
                .find(|node| node.kind() == SyntaxKind::MODULE_FIELD_FUNC)
                .map(|func| {
                    get_func_sig(
                        shared.service,
                        shared.document,
                        SyntaxNodePtr::new(&func),
                        func.green().into(),
                    )
                    .results
                    .into_iter()
                    .map(OperandType::Val)
                    .collect()
                })
                .unwrap_or_default(),
            results: vec![],
        },
        "br" => ResolvedSig {
            params: instr
                .immediates()
                .next()
                .map(|idx| {
                    resolve_br_types(shared.service, shared.document, shared.symbol_table, &idx)
                })
                .unwrap_or_default(),
            results: vec![],
        },
        "br_if" => {
            let results = instr
                .immediates()
                .next()
                .map(|idx| {
                    resolve_br_types(shared.service, shared.document, shared.symbol_table, &idx)
                })
                .unwrap_or_default();
            let mut params = results.clone();
            params.push(OperandType::Val(ValType::I32));
            ResolvedSig { params, results }
        }
        "br_table" => {
            let mut params = instr
                .immediates()
                .next()
                .map(|idx| {
                    resolve_br_types(shared.service, shared.document, shared.symbol_table, &idx)
                })
                .unwrap_or_default();
            params.push(OperandType::Val(ValType::I32));
            ResolvedSig {
                params,
                results: vec![],
            }
        }
        "br_on_null" => {
            let heap_ty =
                if let Some((OperandType::Val(ValType::Ref(RefType { heap_ty, .. })), _)) =
                    type_stack.stack.last()
                {
                    heap_ty.clone()
                } else {
                    HeapType::Any
                };
            let mut results = instr
                .immediates()
                .next()
                .map(|idx| {
                    resolve_br_types(shared.service, shared.document, shared.symbol_table, &idx)
                })
                .unwrap_or_default();
            let mut params = results.clone();
            params.push(OperandType::Val(ValType::Ref(RefType {
                heap_ty: heap_ty.clone(),
                nullable: true,
            })));
            results.push(OperandType::Val(ValType::Ref(RefType {
                heap_ty: heap_ty.clone(),
                nullable: false,
            })));
            ResolvedSig { params, results }
        }
        "br_on_non_null" => {
            let heap_ty =
                if let Some((OperandType::Val(ValType::Ref(RefType { heap_ty, .. })), _)) =
                    type_stack.stack.last()
                {
                    heap_ty.clone()
                } else {
                    HeapType::Any
                };
            let results = instr
                .immediates()
                .next()
                .map(|idx| {
                    resolve_br_types(shared.service, shared.document, shared.symbol_table, &idx)
                })
                .unwrap_or_default();
            let mut params = results.clone();
            params.push(OperandType::Val(ValType::Ref(RefType {
                heap_ty,
                nullable: true,
            })));
            ResolvedSig { params, results }
        }
        "br_on_cast" => {
            let mut immediates = instr.immediates();
            let mut types = immediates
                .next()
                .map(|idx| {
                    resolve_br_types(shared.service, shared.document, shared.symbol_table, &idx)
                })
                .unwrap_or_default();
            types.pop();
            let rt1 = immediates
                .next()
                .and_then(|immediate| immediate.ref_type())
                .and_then(|ref_type| {
                    RefType::from_green(&ref_type.syntax().green(), shared.service)
                });
            let rt2 = immediates
                .next()
                .and_then(|immediate| immediate.ref_type())
                .and_then(|ref_type| {
                    RefType::from_green(&ref_type.syntax().green(), shared.service)
                });
            let mut params = types.clone();
            let mut results = types;
            if let Some((rt1, rt2)) = rt1.zip(rt2) {
                params.push(OperandType::Val(ValType::Ref(rt1.clone())));
                results.push(OperandType::Val(ValType::Ref(rt1.diff(&rt2))));
            }
            ResolvedSig { params, results }
        }
        "br_on_cast_fail" => {
            let mut immediates = instr.immediates();
            let mut types = immediates
                .next()
                .map(|idx| {
                    resolve_br_types(shared.service, shared.document, shared.symbol_table, &idx)
                })
                .unwrap_or_default();
            types.pop();
            let rt1 = immediates
                .next()
                .and_then(|immediate| immediate.ref_type())
                .and_then(|ref_type| {
                    RefType::from_green(&ref_type.syntax().green(), shared.service)
                });
            let rt2 = immediates
                .next()
                .and_then(|immediate| immediate.ref_type())
                .and_then(|ref_type| {
                    RefType::from_green(&ref_type.syntax().green(), shared.service)
                });
            let mut params = types.clone();
            let mut results = types;
            if let Some((rt1, rt2)) = rt1.zip(rt2) {
                params.push(OperandType::Val(ValType::Ref(rt1)));
                results.push(OperandType::Val(ValType::Ref(rt2)));
            }
            ResolvedSig { params, results }
        }
        "select" => {
            let ty = if let Some(ty) = instr
                .immediates()
                .next()
                .and_then(|immediate| immediate.type_use())
                .and_then(|type_use| type_use.results().next())
                .and_then(|result| result.val_types().next())
            {
                ValType::from_ast(&ty, shared.service).map_or(OperandType::Any, OperandType::Val)
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
        "call_indirect" | "return_call_indirect" => {
            let mut sig = instr
                .immediates()
                .find_map(|immediate| immediate.type_use())
                .map(|type_use| {
                    let node = type_use.syntax();
                    ResolvedSig::from(get_type_use_sig(
                        shared.service,
                        shared.document,
                        SyntaxNodePtr::new(node),
                        node.green().into(),
                    ))
                })
                .unwrap_or_default();
            sig.params.push(OperandType::Val(ValType::I32));
            sig
        }
        "struct.new" => {
            let def_types = get_def_types(shared.service, shared.document);
            instr
                .immediates()
                .next()
                .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
                .and_then(|symbol| def_types.iter().find(|def_type| def_type.key == symbol.key))
                .map(|def_type| {
                    let params = if let CompositeType::Struct(fields) = &def_type.comp {
                        fields.to_operand_types()
                    } else {
                        vec![]
                    };
                    ResolvedSig {
                        params,
                        results: vec![OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(def_type.idx),
                            nullable: false,
                        }))],
                    }
                })
                .unwrap_or_else(|| ResolvedSig {
                    params: vec![],
                    results: vec![OperandType::Any],
                })
        }
        "struct.new_default" => instr
            .immediates()
            .next()
            .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
            .map(|symbol| ResolvedSig {
                params: vec![],
                results: vec![OperandType::Val(ValType::Ref(RefType {
                    heap_ty: HeapType::Type(symbol.idx),
                    nullable: false,
                }))],
            })
            .unwrap_or_else(|| ResolvedSig {
                params: vec![],
                results: vec![OperandType::Any],
            }),
        "struct.get" => {
            let mut immediates = instr.immediates();
            immediates
                .next()
                .zip(immediates.next())
                .and_then(|(struct_ref, field_ref)| {
                    resolve_field_type_with_struct_idx(
                        shared.service,
                        shared.document,
                        &struct_ref,
                        &field_ref,
                    )
                })
                .map(|(idx, ty)| ResolvedSig {
                    params: vec![OperandType::Val(ValType::Ref(RefType {
                        heap_ty: HeapType::Type(idx),
                        nullable: true,
                    }))],
                    results: vec![ty.unwrap_or(OperandType::Any)],
                })
                .unwrap_or_else(|| ResolvedSig {
                    params: vec![],
                    results: vec![OperandType::Any],
                })
        }
        "struct.get_s" | "struct.get_u" => ResolvedSig {
            params: instr
                .immediates()
                .next()
                .and_then(|immediate| {
                    shared
                        .symbol_table
                        .find_def(SymbolKey::new(immediate.syntax()))
                })
                .map(|symbol| {
                    vec![OperandType::Val(ValType::Ref(RefType {
                        heap_ty: HeapType::Type(symbol.idx),
                        nullable: true,
                    }))]
                })
                .unwrap_or_default(),
            results: vec![OperandType::Val(ValType::I32)],
        },
        "struct.set" => {
            let mut immediates = instr.immediates();
            immediates
                .next()
                .zip(immediates.next())
                .and_then(|(struct_ref, field_ref)| {
                    resolve_field_type_with_struct_idx(
                        shared.service,
                        shared.document,
                        &struct_ref,
                        &field_ref,
                    )
                })
                .map(|(idx, ty)| ResolvedSig {
                    params: vec![
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(idx),
                            nullable: true,
                        })),
                        ty.unwrap_or(OperandType::Any),
                    ],
                    results: vec![],
                })
                .unwrap_or_else(|| ResolvedSig {
                    params: vec![OperandType::Any],
                    results: vec![],
                })
        }
        "array.new" => {
            let mut sig = instr
                .immediates()
                .next()
                .and_then(|immediate| {
                    let def_types = get_def_types(shared.service, shared.document);
                    resolve_array_type_with_idx(shared.symbol_table, def_types, &immediate)
                })
                .map(|(idx, ty)| ResolvedSig {
                    params: vec![ty.unwrap_or(OperandType::Any)],
                    results: vec![OperandType::Val(ValType::Ref(RefType {
                        heap_ty: HeapType::Type(idx),
                        nullable: false,
                    }))],
                })
                .unwrap_or_else(|| ResolvedSig {
                    params: vec![],
                    results: vec![OperandType::Any],
                });
            sig.params.push(OperandType::Val(ValType::I32));
            sig
        }
        "array.new_default" => instr
            .immediates()
            .next()
            .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
            .map(|symbol| ResolvedSig {
                params: vec![OperandType::Val(ValType::I32)],
                results: vec![OperandType::Val(ValType::Ref(RefType {
                    heap_ty: HeapType::Type(symbol.idx),
                    nullable: false,
                }))],
            })
            .unwrap_or_else(|| ResolvedSig {
                params: vec![],
                results: vec![OperandType::Any],
            }),
        "array.new_fixed" => {
            let mut immediates = instr.immediates();
            immediates
                .next()
                .and_then(|immediate| {
                    let def_types = get_def_types(shared.service, shared.document);
                    resolve_array_type_with_idx(shared.symbol_table, def_types, &immediate)
                })
                .map(|(idx, ty)| {
                    let count = immediates
                        .next()
                        .and_then(|immediate| immediate.int())
                        .and_then(|int| int.text().parse().ok())
                        .unwrap_or_default();
                    ResolvedSig {
                        params: vec![ty.unwrap_or(OperandType::Any); count],
                        results: vec![OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(idx),
                            nullable: false,
                        }))],
                    }
                })
                .unwrap_or_else(|| ResolvedSig {
                    params: vec![],
                    results: vec![OperandType::Any],
                })
        }
        "array.new_data" | "array.new_elem" => instr
            .immediates()
            .next()
            .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
            .map(|symbol| ResolvedSig {
                params: vec![OperandType::Val(ValType::I32); 2],
                results: vec![OperandType::Val(ValType::Ref(RefType {
                    heap_ty: HeapType::Type(symbol.idx),
                    nullable: false,
                }))],
            })
            .unwrap_or_else(|| ResolvedSig {
                params: vec![],
                results: vec![OperandType::Any],
            }),
        "array.get" => instr
            .immediates()
            .next()
            .and_then(|immediate| {
                let def_types = get_def_types(shared.service, shared.document);
                resolve_array_type_with_idx(shared.symbol_table, def_types, &immediate)
            })
            .map(|(idx, ty)| ResolvedSig {
                params: vec![
                    OperandType::Val(ValType::Ref(RefType {
                        heap_ty: HeapType::Type(idx),
                        nullable: true,
                    })),
                    OperandType::Val(ValType::I32),
                ],
                results: vec![ty.unwrap_or(OperandType::Any)],
            })
            .unwrap_or_else(|| ResolvedSig {
                params: vec![],
                results: vec![OperandType::Any],
            }),
        "array.get_s" | "array.get_u" => instr
            .immediates()
            .next()
            .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
            .map(|symbol| ResolvedSig {
                params: vec![
                    OperandType::Val(ValType::Ref(RefType {
                        heap_ty: HeapType::Type(symbol.idx),
                        nullable: true,
                    })),
                    OperandType::Val(ValType::I32),
                ],
                results: vec![OperandType::Val(ValType::I32)],
            })
            .unwrap_or_else(|| ResolvedSig {
                params: vec![],
                results: vec![OperandType::Val(ValType::I32)],
            }),
        "array.set" => instr
            .immediates()
            .next()
            .and_then(|immediate| {
                let def_types = get_def_types(shared.service, shared.document);
                resolve_array_type_with_idx(shared.symbol_table, def_types, &immediate)
            })
            .map(|(idx, ty)| ResolvedSig {
                params: vec![
                    OperandType::Val(ValType::Ref(RefType {
                        heap_ty: HeapType::Type(idx),
                        nullable: true,
                    })),
                    OperandType::Val(ValType::I32),
                    ty.unwrap_or(OperandType::Any),
                ],
                results: vec![],
            })
            .unwrap_or_default(),
        "array.fill" => instr
            .immediates()
            .next()
            .and_then(|immediate| {
                let def_types = get_def_types(shared.service, shared.document);
                resolve_array_type_with_idx(shared.symbol_table, def_types, &immediate)
            })
            .map(|(idx, ty)| ResolvedSig {
                params: vec![
                    OperandType::Val(ValType::Ref(RefType {
                        heap_ty: HeapType::Type(idx),
                        nullable: true,
                    })),
                    OperandType::Val(ValType::I32),
                    ty.unwrap_or(OperandType::Any),
                    OperandType::Val(ValType::I32),
                ],
                results: vec![],
            })
            .unwrap_or_default(),
        "array.copy" => {
            let mut immediates = instr.immediates();
            immediates
                .next()
                .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
                .zip(
                    immediates
                        .next()
                        .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax()))),
                )
                .map(|(dst, src)| ResolvedSig {
                    params: vec![
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(dst.idx),
                            nullable: true,
                        })),
                        OperandType::Val(ValType::I32),
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(src.idx),
                            nullable: true,
                        })),
                        OperandType::Val(ValType::I32),
                        OperandType::Val(ValType::I32),
                    ],
                    results: vec![],
                })
                .unwrap_or_default()
        }
        "array.init_data" | "array.init_elem" => instr
            .immediates()
            .next()
            .and_then(|idx| shared.symbol_table.find_def(SymbolKey::new(idx.syntax())))
            .map(|symbol| ResolvedSig {
                params: vec![
                    OperandType::Val(ValType::Ref(RefType {
                        heap_ty: HeapType::Type(symbol.idx),
                        nullable: true,
                    })),
                    OperandType::Val(ValType::I32),
                    OperandType::Val(ValType::I32),
                    OperandType::Val(ValType::I32),
                ],
                results: vec![],
            })
            .unwrap_or_default(),
        "ref.null" => {
            let ty = instr
                .immediates()
                .next()
                .and_then(|immediate| immediate.syntax().first_child_or_token())
                .and_then(|element| match element {
                    SyntaxElement::Node(node) if node.kind() == SyntaxKind::HEAP_TYPE => {
                        HeapType::from_green(&node.green(), shared.service)
                    }
                    SyntaxElement::Token(token) if token.kind() == SyntaxKind::IDENT => {
                        Some(HeapType::Type(Idx {
                            num: None,
                            name: Some(InternIdent::new(shared.service, token.text())),
                        }))
                    }
                    SyntaxElement::Token(token) if token.kind() == SyntaxKind::INT => {
                        Some(HeapType::Type(Idx {
                            num: token.text().parse().ok(),
                            name: None,
                        }))
                    }
                    _ => None,
                })
                .map_or(OperandType::Any, |heap_ty| {
                    OperandType::Val(ValType::Ref(RefType {
                        heap_ty,
                        nullable: true,
                    }))
                });
            ResolvedSig {
                params: vec![],
                results: vec![ty],
            }
        }
        "ref.is_null" => {
            let heap_ty =
                if let Some((OperandType::Val(ValType::Ref(RefType { heap_ty, .. })), _)) =
                    type_stack.stack.last()
                {
                    heap_ty.clone()
                } else {
                    HeapType::Any
                };
            ResolvedSig {
                params: vec![OperandType::Val(ValType::Ref(RefType {
                    heap_ty,
                    nullable: true,
                }))],
                results: vec![OperandType::Val(ValType::I32)],
            }
        }
        "ref.as_non_null" => {
            let heap_ty =
                if let Some((OperandType::Val(ValType::Ref(RefType { heap_ty, .. })), _)) =
                    type_stack.stack.last()
                {
                    heap_ty.clone()
                } else {
                    HeapType::Any
                };
            ResolvedSig {
                params: vec![OperandType::Val(ValType::Ref(RefType {
                    heap_ty: heap_ty.clone(),
                    nullable: true,
                }))],
                results: vec![OperandType::Val(ValType::Ref(RefType {
                    heap_ty: heap_ty.clone(),
                    nullable: false,
                }))],
            }
        }
        "ref.test" => {
            let heap_ty = instr
                .immediates()
                .next()
                .and_then(|immediate| immediate.ref_type())
                .and_then(|ref_type| {
                    RefType::from_green(&ref_type.syntax().green(), shared.service)
                })
                .and_then(|ref_type| {
                    ref_type
                        .heap_ty
                        .to_top_type(shared.service, shared.document, shared.module_id)
                })
                .unwrap_or(HeapType::Any);
            ResolvedSig {
                params: vec![OperandType::Val(ValType::Ref(RefType {
                    heap_ty,
                    nullable: true,
                }))],
                results: vec![OperandType::Val(ValType::I32)],
            }
        }
        "ref.cast" => {
            let ref_type = instr
                .immediates()
                .next()
                .and_then(|immediate| immediate.ref_type())
                .and_then(|ref_type| {
                    RefType::from_green(&ref_type.syntax().green(), shared.service)
                })
                .unwrap_or(RefType {
                    heap_ty: HeapType::Any,
                    nullable: true,
                });
            let heap_ty = ref_type
                .heap_ty
                .to_top_type(shared.service, shared.document, shared.module_id)
                .unwrap_or(HeapType::Any);
            ResolvedSig {
                params: vec![OperandType::Val(ValType::Ref(RefType {
                    heap_ty,
                    nullable: true,
                }))],
                results: vec![OperandType::Val(ValType::Ref(ref_type))],
            }
        }
        "ref.func" => {
            let immediate = instr.immediates().next();
            let heap_ty = immediate
                .as_ref()
                .and_then(|immediate| {
                    shared
                        .symbol_table
                        .find_def(SymbolKey::new(immediate.syntax()))
                })
                .and_then(|symbol| {
                    let root = shared.document.root_tree(shared.service);
                    ModuleFieldFunc::cast(symbol.key.to_node(&root))
                })
                .and_then(|func| func.type_use())
                .and_then(|type_use| type_use.index())
                .map(|index| {
                    HeapType::Type(Idx {
                        num: index
                            .unsigned_int_token()
                            .and_then(|int| int.text().parse().ok()),
                        name: index
                            .ident_token()
                            .map(|ident| InternIdent::new(shared.service, ident.text())),
                    })
                })
                .or_else(|| {
                    immediate.map(|immediate| {
                        HeapType::DefFunc(Idx::from_immediate(&immediate, shared.service))
                    })
                });
            ResolvedSig {
                params: vec![],
                results: vec![heap_ty.map_or(OperandType::Any, |heap_ty| {
                    OperandType::Val(ValType::Ref(RefType {
                        heap_ty,
                        nullable: false,
                    }))
                })],
            }
        }
        "call_ref" | "return_call_ref" => {
            let def_types = get_def_types(shared.service, shared.document);
            instr
                .immediates()
                .next()
                .and_then(|immediate| {
                    shared
                        .symbol_table
                        .find_def(SymbolKey::new(immediate.syntax()))
                })
                .and_then(|symbol| def_types.iter().find(|def_type| def_type.key == symbol.key))
                .map(|def_type| {
                    let mut sig = if let CompositeType::Func(sig) = &def_type.comp {
                        ResolvedSig::from(sig.clone())
                    } else {
                        ResolvedSig::default()
                    };
                    sig.params.push(OperandType::Val(ValType::Ref(RefType {
                        heap_ty: HeapType::Type(def_type.idx),
                        nullable: true,
                    })));
                    sig
                })
                .unwrap_or_default()
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
