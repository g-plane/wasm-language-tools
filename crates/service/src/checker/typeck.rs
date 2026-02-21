use super::{Diagnostic, DiagnosticCtx, RelatedInformation};
use crate::{
    binder::SymbolKind,
    data_set, helpers,
    idx::{Idx, InternIdent},
    types_analyzer::{
        CompositeType, HeapType, OperandType, RefType, Signature, ValType, extract_global_type, extract_type,
        get_func_sig, get_type_use_sig, join_types, resolve_array_type_with_idx, resolve_br_types,
        resolve_field_type_with_struct_idx,
    },
};
use bumpalo::{Bump, collections::Vec as BumpVec};
use itertools::{EitherOrBoth, Itertools};
use std::iter;
use wat_syntax::{
    AmberNode, NodeOrToken, SyntaxKind, TextRange,
    ast::{AstNode, BlockInstr, Instr, ValType as AstValType},
};

const DIAGNOSTIC_CODE: &str = "type-check";

pub fn check_func(diagnostics: &mut Vec<Diagnostic>, ctx: &mut DiagnosticCtx, node: AmberNode) {
    {
        let func_key = node.to_ptr().into();
        let results = BumpVec::from_iter_in(
            get_func_sig(ctx.db, ctx.document, func_key, node.green())
                .results
                .into_iter()
                .map(OperandType::Val),
            ctx.bump,
        );
        check_block_like(
            diagnostics,
            ctx,
            node,
            if ctx.imports.contains(&func_key) {
                BumpVec::from_iter_in(results.iter().map(|ty| (ty.clone(), None)), ctx.bump)
            } else {
                BumpVec::with_capacity_in(2, ctx.bump)
            },
            &results,
        );
    }
    ctx.bump.reset();
}

pub fn check_global(diagnostics: &mut Vec<Diagnostic>, ctx: &mut DiagnosticCtx, node: AmberNode) {
    let ty = extract_global_type(ctx.db, node.green())
        .map(OperandType::Val)
        .unwrap_or(OperandType::Any);
    check_block_like(
        diagnostics,
        ctx,
        node,
        if ctx.imports.contains(&node.to_ptr().into()) {
            BumpVec::from_iter_in([(ty.clone(), None)], ctx.bump)
        } else {
            BumpVec::with_capacity_in(1, ctx.bump)
        },
        &[ty],
    );
    ctx.bump.reset();
}

pub fn check_table(diagnostics: &mut Vec<Diagnostic>, ctx: &mut DiagnosticCtx, node: AmberNode) {
    let Some(ref_type) = node
        .children_by_kind(SyntaxKind::TABLE_TYPE)
        .next()
        .unwrap_or(node)
        .children_by_kind(SyntaxKind::REF_TYPE)
        .next()
        .and_then(|ref_type| RefType::from_green(ref_type.green(), ctx.db))
    else {
        return;
    };
    let ty = ValType::Ref(ref_type);
    if ty.defaultable() && node.children_by_kind(Instr::can_cast).next().is_none() {
        return;
    }
    let ty = OperandType::Val(ty);
    check_block_like(
        diagnostics,
        ctx,
        node,
        if ctx.imports.contains(&node.to_ptr().into()) {
            BumpVec::from_iter_in([(ty.clone(), None)], ctx.bump)
        } else {
            BumpVec::with_capacity_in(1, ctx.bump)
        },
        &[ty],
    );
    ctx.bump.reset();
}

pub fn check_offset(diagnostics: &mut Vec<Diagnostic>, ctx: &mut DiagnosticCtx, node: AmberNode) {
    check_block_like(
        diagnostics,
        ctx,
        node,
        BumpVec::with_capacity_in(1, ctx.bump),
        &[OperandType::Val(ValType::I32)],
    );
    ctx.bump.reset();
}

pub fn check_elem_list(diagnostics: &mut Vec<Diagnostic>, ctx: &mut DiagnosticCtx, node: AmberNode) {
    let Some(ref_type) = node
        .children_by_kind(SyntaxKind::REF_TYPE)
        .next()
        .and_then(|ref_type| RefType::from_green(ref_type.green(), ctx.db))
    else {
        return;
    };
    let ty = OperandType::Val(ValType::Ref(ref_type));
    node.children_by_kind(SyntaxKind::ELEM_EXPR).for_each(|child| {
        check_block_like(
            diagnostics,
            ctx,
            child,
            BumpVec::with_capacity_in(1, ctx.bump),
            std::slice::from_ref(&ty),
        );
    });
    ctx.bump.reset();
}

fn check_block_like(
    diagnostics: &mut Vec<Diagnostic>,
    ctx: &DiagnosticCtx,
    node: AmberNode,
    init_stack: BumpVec<(OperandType, Option<AmberNode>)>,
    expected_results: &[OperandType],
) {
    let mut type_stack = TypeStack {
        ctx,
        stack: init_stack,
        has_never: false,
    };

    fn unfold<'db, 'bump>(
        node: AmberNode<'db>,
        type_stack: &mut TypeStack<'db, 'bump>,
        diagnostics: &mut Vec<Diagnostic>,
        ctx: &'db DiagnosticCtx<'db, 'bump>,
    ) {
        if matches!(node.kind(), SyntaxKind::PLAIN_INSTR | SyntaxKind::BLOCK_IF) {
            node.children_by_kind(Instr::can_cast)
                .for_each(|child| unfold(child, type_stack, diagnostics, ctx));
        }
        check_instr(node, type_stack, diagnostics, ctx);
    }
    node.children_by_kind(Instr::can_cast)
        .for_each(|child| unfold(child, &mut type_stack, diagnostics, ctx));

    if let Some(diagnostic) = type_stack.check_to_bottom(expected_results, ReportRange::Last(node)) {
        diagnostics.push(diagnostic);
    }
}

fn check_instr<'db, 'bump>(
    node: AmberNode<'db>,
    type_stack: &mut TypeStack<'db, 'bump>,
    diagnostics: &mut Vec<Diagnostic>,
    ctx: &'db DiagnosticCtx<'db, 'bump>,
) {
    if node.kind() == SyntaxKind::PLAIN_INSTR {
        let Some(instr_name) = node.tokens_by_kind(SyntaxKind::INSTR_NAME).next() else {
            return;
        };
        let instr_name = instr_name.text();
        let sig = resolve_sig(ctx, instr_name, node, type_stack);
        if let Some(diagnostic) = type_stack.check(&sig.params, ReportRange::Instr(node)) {
            diagnostics.push(diagnostic);
        }
        if helpers::is_stack_polymorphic(instr_name) {
            type_stack.has_never = true;
            type_stack.stack.clear();
        }
        type_stack
            .stack
            .extend(sig.results.into_iter().map(|ty| (ty, Some(node))));
    } else if BlockInstr::can_cast(node.kind()) {
        let signature = get_func_sig(ctx.db, ctx.document, node.to_ptr().into(), node.green());
        let init_stack = BumpVec::from_iter_in(
            signature
                .params
                .iter()
                .map(|(ty, ..)| (OperandType::Val(ty.clone()), Some(node))),
            ctx.bump,
        );
        let results = BumpVec::from_iter_in(signature.results.into_iter().map(OperandType::Val), ctx.bump);
        match node.kind() {
            SyntaxKind::BLOCK_BLOCK | SyntaxKind::BLOCK_LOOP | SyntaxKind::BLOCK_TRY_TABLE => {
                if let Some(diagnostic) = type_stack.check(
                    &BumpVec::from_iter_in(
                        signature.params.into_iter().map(|(ty, _)| OperandType::Val(ty)),
                        ctx.bump,
                    ),
                    ReportRange::Instr(node),
                ) {
                    diagnostics.push(diagnostic);
                }
                check_block_like(diagnostics, ctx, node, init_stack, &results);
            }
            SyntaxKind::BLOCK_IF => {
                if let Some(mut diagnostic) =
                    type_stack.check(&[OperandType::Val(ValType::I32)], ReportRange::Keyword(node))
                {
                    diagnostic.message.push_str(" for the condition of `if` block");
                    diagnostics.push(diagnostic);
                }
                if let Some(diagnostic) = type_stack.check(
                    &BumpVec::from_iter_in(
                        signature.params.into_iter().map(|(ty, _)| OperandType::Val(ty)),
                        ctx.bump,
                    ),
                    ReportRange::Instr(node),
                ) {
                    diagnostics.push(diagnostic);
                }
                let mut children = node.children();
                if let Some(then_block) = children.find(|child| child.kind() == SyntaxKind::BLOCK_IF_THEN) {
                    check_block_like(diagnostics, ctx, then_block, init_stack.clone(), &results);
                } else {
                    diagnostics.push(Diagnostic {
                        range: node.text_range(),
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!(
                            "missing `then` branch with expected types {}",
                            join_types(ctx.db, results.iter(), "", ctx.bump)
                        ),
                        ..Default::default()
                    });
                }
                if let Some(else_block) = children.find(|child| child.kind() == SyntaxKind::BLOCK_IF_ELSE) {
                    check_block_like(diagnostics, ctx, else_block, init_stack, &results);
                } else {
                    let mut type_stack = TypeStack {
                        ctx,
                        stack: init_stack,
                        has_never: false,
                    };
                    if type_stack.check_to_bottom(&results, ReportRange::Instr(node)).is_some() {
                        diagnostics.push(Diagnostic {
                            range: node.text_range(),
                            code: DIAGNOSTIC_CODE.into(),
                            message: format!(
                                "missing `else` branch with expected types {}",
                                join_types(ctx.db, results.iter(), "", ctx.bump)
                            ),
                            ..Default::default()
                        });
                    }
                }
            }
            _ => {}
        }
        type_stack.stack.extend(results.into_iter().map(|ty| (ty, Some(node))));
    }
}

struct TypeStack<'db, 'bump> {
    ctx: &'db DiagnosticCtx<'db, 'bump>,
    stack: BumpVec<'bump, (OperandType<'db>, Option<AmberNode<'db>>)>,
    has_never: bool,
}
impl<'db, 'bump> TypeStack<'db, 'bump> {
    fn check(&mut self, expected: &[OperandType<'db>], report_range: ReportRange) -> Option<Diagnostic> {
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
                    if received.matches(expected, self.ctx.db, self.ctx.document, self.ctx.module_id) {
                        return;
                    }
                    mismatch = true;
                    if let Some(related_instr) = related_instr {
                        related_information.push(RelatedInformation {
                            range: ReportRange::Instr(*related_instr).pick(),
                            message: format!(
                                "expected type `{}`, found `{}`",
                                expected.render(self.ctx.db),
                                received.render(self.ctx.db),
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
            diagnostic = Some(Diagnostic {
                range: report_range.pick(),
                code: DIAGNOSTIC_CODE.into(),
                message: format!(
                    "expected types {}, found {}",
                    join_types(self.ctx.db, expected.iter(), "", self.ctx.bump),
                    join_types(
                        self.ctx.db,
                        pops.iter().map(|(ty, _)| ty),
                        if self.stack.len() > pops.len() { "... " } else { "" },
                        self.ctx.bump,
                    ),
                ),
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

    fn check_to_bottom(&mut self, expected: &[OperandType<'db>], report_range: ReportRange) -> Option<Diagnostic> {
        let mut mismatch = false;
        let mut related_information = vec![];
        expected
            .iter()
            .rev()
            .zip_longest(self.stack.iter().rev())
            .for_each(|pair| match pair {
                EitherOrBoth::Both(expected, (received, related_instr)) => {
                    if received.matches(expected, self.ctx.db, self.ctx.document, self.ctx.module_id) {
                        return;
                    }
                    mismatch = true;
                    if let Some(related_instr) = related_instr {
                        related_information.push(RelatedInformation {
                            range: ReportRange::Instr(*related_instr).pick(),
                            message: format!(
                                "expected type `{}`, found `{}`",
                                expected.render(self.ctx.db),
                                received.render(self.ctx.db),
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
            Some(Diagnostic {
                range: report_range.pick(),
                code: DIAGNOSTIC_CODE.into(),
                message: format!(
                    "expected types {}, found {}{}",
                    join_types(self.ctx.db, expected.iter(), "", self.ctx.bump),
                    join_types(self.ctx.db, self.stack.iter().map(|(ty, _)| ty), "", self.ctx.bump),
                    if let ReportRange::Last(..) = report_range {
                        " at the end"
                    } else {
                        ""
                    },
                ),
                related_information: if related_information.is_empty() {
                    None
                } else {
                    Some(related_information)
                },
                data: if expected.is_empty() {
                    self.stack
                        .iter()
                        .map(|(ty, _)| match ty {
                            OperandType::Val(ty) => Some(serde_json::Value::String(ty.render(self.ctx.db).to_string())),
                            OperandType::Any => None,
                        })
                        .collect::<Option<Vec<_>>>()
                        .map(serde_json::Value::Array)
                } else {
                    None
                },
                ..Default::default()
            })
        } else {
            None
        }
    }
}

struct ResolvedSig<'db, 'bump> {
    params: BumpVec<'bump, OperandType<'db>>,
    results: BumpVec<'bump, OperandType<'db>>,
}
impl<'db, 'bump> ResolvedSig<'db, 'bump> {
    fn new_in(bump: &'bump Bump) -> Self {
        Self {
            params: BumpVec::new_in(bump),
            results: BumpVec::new_in(bump),
        }
    }
    fn from_func_sig_in(sig: Signature<'db>, bump: &'bump Bump) -> Self {
        Self {
            params: BumpVec::from_iter_in(sig.params.into_iter().map(|(ty, _)| OperandType::Val(ty)), bump),
            results: BumpVec::from_iter_in(sig.results.into_iter().map(OperandType::Val), bump),
        }
    }
}
fn resolve_sig<'db, 'bump>(
    ctx: &'db DiagnosticCtx<'db, 'bump>,
    instr_name: &str,
    instr: AmberNode<'db>,
    type_stack: &TypeStack<'db, 'bump>,
) -> ResolvedSig<'db, 'bump> {
    let bump = &ctx.bump;
    match instr_name {
        "call" | "return_call" | "throw" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
            .map(|func| ResolvedSig::from_func_sig_in(get_func_sig(ctx.db, ctx.document, func.key, &func.green), bump))
            .unwrap_or_else(|| ResolvedSig::new_in(bump)),
        "local.get" => ResolvedSig {
            params: BumpVec::new_in(bump),
            results: BumpVec::from_iter_in(
                [instr
                    .children_by_kind(SyntaxKind::IMMEDIATE)
                    .next()
                    .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
                    .and_then(|symbol| extract_type(ctx.db, &symbol.green))
                    .map_or(OperandType::Any, OperandType::Val)],
                bump,
            ),
        },
        "local.set" => ResolvedSig {
            params: BumpVec::from_iter_in(
                [instr
                    .children_by_kind(SyntaxKind::IMMEDIATE)
                    .next()
                    .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
                    .and_then(|symbol| extract_type(ctx.db, &symbol.green))
                    .map_or(OperandType::Any, OperandType::Val)],
                bump,
            ),
            results: BumpVec::new_in(bump),
        },
        "local.tee" => {
            let ty = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
                .and_then(|symbol| extract_type(ctx.db, &symbol.green))
                .map_or(OperandType::Any, OperandType::Val);
            ResolvedSig {
                params: BumpVec::from_iter_in([ty.clone()], bump),
                results: BumpVec::from_iter_in([ty], bump),
            }
        }
        "global.get" => ResolvedSig {
            params: BumpVec::new_in(bump),
            results: BumpVec::from_iter_in(
                [instr
                    .children_by_kind(SyntaxKind::IMMEDIATE)
                    .next()
                    .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
                    .and_then(|symbol| extract_global_type(ctx.db, &symbol.green))
                    .map_or(OperandType::Any, OperandType::Val)],
                bump,
            ),
        },
        "global.set" => ResolvedSig {
            params: BumpVec::from_iter_in(
                [instr
                    .children_by_kind(SyntaxKind::IMMEDIATE)
                    .next()
                    .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
                    .and_then(|symbol| extract_global_type(ctx.db, &symbol.green))
                    .map_or(OperandType::Any, OperandType::Val)],
                bump,
            ),
            results: BumpVec::new_in(bump),
        },
        "i32.const" => ResolvedSig {
            params: BumpVec::new_in(bump),
            results: BumpVec::from_iter_in([OperandType::Val(ValType::I32)], bump),
        },
        "i64.const" => ResolvedSig {
            params: BumpVec::new_in(bump),
            results: BumpVec::from_iter_in([OperandType::Val(ValType::I64)], bump),
        },
        "f32.const" => ResolvedSig {
            params: BumpVec::new_in(bump),
            results: BumpVec::from_iter_in([OperandType::Val(ValType::F32)], bump),
        },
        "f64.const" => ResolvedSig {
            params: BumpVec::new_in(bump),
            results: BumpVec::from_iter_in([OperandType::Val(ValType::F64)], bump),
        },
        "return" => ResolvedSig {
            params: ctx
                .symbol_table
                .symbols
                .values()
                .find(|symbol| {
                    symbol.kind == SymbolKind::Func && symbol.key.text_range().contains_range(instr.text_range())
                })
                .map(|func| {
                    BumpVec::from_iter_in(
                        get_func_sig(ctx.db, ctx.document, func.key, &func.green)
                            .results
                            .into_iter()
                            .map(OperandType::Val),
                        bump,
                    )
                })
                .unwrap_or_else(|| BumpVec::new_in(bump)),
            results: BumpVec::new_in(bump),
        },
        "br" => ResolvedSig {
            params: instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|idx| resolve_br_types(ctx.db, ctx.document, ctx.symbol_table, idx.to_ptr().into()))
                .map(|types| BumpVec::from_iter_in(types, bump))
                .unwrap_or_else(|| BumpVec::new_in(bump)),
            results: BumpVec::new_in(bump),
        },
        "br_if" => {
            let results = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|idx| resolve_br_types(ctx.db, ctx.document, ctx.symbol_table, idx.to_ptr().into()))
                .map(|types| BumpVec::from_iter_in(types, bump))
                .unwrap_or_else(|| BumpVec::new_in(bump));
            let params = BumpVec::from_iter_in(
                results
                    .iter()
                    .cloned()
                    .chain(iter::once(OperandType::Val(ValType::I32))),
                bump,
            );
            ResolvedSig { params, results }
        }
        "br_table" => {
            let mut params = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|idx| resolve_br_types(ctx.db, ctx.document, ctx.symbol_table, idx.to_ptr().into()))
                .map(|types| BumpVec::from_iter_in(types, bump))
                .unwrap_or_else(|| BumpVec::new_in(bump));
            params.push(OperandType::Val(ValType::I32));
            ResolvedSig {
                params,
                results: BumpVec::new_in(bump),
            }
        }
        "br_on_null" => {
            let heap_ty =
                if let Some((OperandType::Val(ValType::Ref(RefType { heap_ty, .. })), _)) = type_stack.stack.last() {
                    heap_ty.clone()
                } else {
                    HeapType::Any
                };
            let mut results = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|idx| resolve_br_types(ctx.db, ctx.document, ctx.symbol_table, idx.to_ptr().into()))
                .map(|types| BumpVec::from_iter_in(types, bump))
                .unwrap_or_else(|| BumpVec::new_in(bump));
            let params = BumpVec::from_iter_in(
                results
                    .iter()
                    .cloned()
                    .chain(iter::once(OperandType::Val(ValType::Ref(RefType {
                        heap_ty: heap_ty.clone(),
                        nullable: true,
                    })))),
                bump,
            );
            results.push(OperandType::Val(ValType::Ref(RefType {
                heap_ty: heap_ty.clone(),
                nullable: false,
            })));
            ResolvedSig { params, results }
        }
        "br_on_non_null" => {
            let heap_ty =
                if let Some((OperandType::Val(ValType::Ref(RefType { heap_ty, .. })), _)) = type_stack.stack.last() {
                    heap_ty.clone()
                } else {
                    HeapType::Any
                };
            let results = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|idx| resolve_br_types(ctx.db, ctx.document, ctx.symbol_table, idx.to_ptr().into()))
                .map(|types| BumpVec::from_iter_in(types, bump))
                .unwrap_or_else(|| BumpVec::new_in(bump));
            let params = BumpVec::from_iter_in(
                results
                    .iter()
                    .cloned()
                    .chain(iter::once(OperandType::Val(ValType::Ref(RefType {
                        heap_ty,
                        nullable: true,
                    })))),
                bump,
            );
            ResolvedSig { params, results }
        }
        "br_on_cast" => {
            let mut immediates = instr.children_by_kind(SyntaxKind::IMMEDIATE);
            let mut types = immediates
                .next()
                .and_then(|idx| resolve_br_types(ctx.db, ctx.document, ctx.symbol_table, idx.to_ptr().into()))
                .map(|types| BumpVec::from_iter_in(types, bump))
                .unwrap_or_else(|| BumpVec::new_in(bump));
            types.pop();
            let rt1 = immediates
                .next()
                .and_then(|immediate| immediate.children_by_kind(SyntaxKind::REF_TYPE).next())
                .and_then(|ref_type| RefType::from_green(ref_type.green(), ctx.db));
            let rt2 = immediates
                .next()
                .and_then(|immediate| immediate.children_by_kind(SyntaxKind::REF_TYPE).next())
                .and_then(|ref_type| RefType::from_green(ref_type.green(), ctx.db));
            let mut params = BumpVec::from_iter_in(types.iter().cloned(), bump);
            let mut results = types;
            if let Some((rt1, rt2)) = rt1.zip(rt2) {
                params.push(OperandType::Val(ValType::Ref(rt1.clone())));
                results.push(OperandType::Val(ValType::Ref(rt1.diff(&rt2))));
            }
            ResolvedSig { params, results }
        }
        "br_on_cast_fail" => {
            let mut immediates = instr.children_by_kind(SyntaxKind::IMMEDIATE);
            let mut types = immediates
                .next()
                .and_then(|idx| resolve_br_types(ctx.db, ctx.document, ctx.symbol_table, idx.to_ptr().into()))
                .map(|types| BumpVec::from_iter_in(types, bump))
                .unwrap_or_else(|| BumpVec::new_in(bump));
            types.pop();
            let rt1 = immediates
                .next()
                .and_then(|immediate| immediate.children_by_kind(SyntaxKind::REF_TYPE).next())
                .and_then(|ref_type| RefType::from_green(ref_type.green(), ctx.db));
            let rt2 = immediates
                .next()
                .and_then(|immediate| immediate.children_by_kind(SyntaxKind::REF_TYPE).next())
                .and_then(|ref_type| RefType::from_green(ref_type.green(), ctx.db));
            let mut params = BumpVec::from_iter_in(types.iter().cloned(), bump);
            let mut results = types;
            if let Some((rt1, rt2)) = rt1.zip(rt2) {
                params.push(OperandType::Val(ValType::Ref(rt1)));
                results.push(OperandType::Val(ValType::Ref(rt2)));
            }
            ResolvedSig { params, results }
        }
        "select" => {
            let ty = if let Some(ty) = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| immediate.children_by_kind(SyntaxKind::TYPE_USE).next())
                .and_then(|type_use| type_use.children_by_kind(SyntaxKind::RESULT).next())
                .and_then(|result| result.children_by_kind(AstValType::can_cast).next())
            {
                ValType::from_green(ty.green(), ctx.db).map_or(OperandType::Any, OperandType::Val)
            } else {
                type_stack
                    .stack
                    .len()
                    .checked_sub(2)
                    .and_then(|i| type_stack.stack.get(i))
                    .map_or(OperandType::Any, |(ty, _)| ty.clone())
            };
            ResolvedSig {
                params: BumpVec::from_iter_in([ty.clone(), ty.clone(), OperandType::Val(ValType::I32)], bump),
                results: BumpVec::from_iter_in([ty], bump),
            }
        }
        "call_indirect" | "return_call_indirect" => {
            let mut sig = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .find_map(|child| child.children_by_kind(SyntaxKind::TYPE_USE).next())
                .map(|node| {
                    ResolvedSig::from_func_sig_in(
                        get_type_use_sig(ctx.db, ctx.document, node.to_ptr(), node.green()),
                        bump,
                    )
                })
                .unwrap_or_else(|| ResolvedSig::new_in(bump));
            sig.params.push(OperandType::Val(ValType::I32));
            sig
        }
        "struct.new" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|immediate| ctx.symbol_table.resolved.get(&immediate.to_ptr().into()))
            .and_then(|key| ctx.def_types.get(key))
            .map(|def_type| {
                let params = if let CompositeType::Struct(fields) = &def_type.comp {
                    BumpVec::from_iter_in(fields.0.iter().map(|(field, _)| field.storage.clone().into()), bump)
                } else {
                    BumpVec::new_in(bump)
                };
                ResolvedSig {
                    params,
                    results: BumpVec::from_iter_in(
                        [OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(def_type.idx),
                            nullable: false,
                        }))],
                        bump,
                    ),
                }
            })
            .unwrap_or_else(|| ResolvedSig {
                params: BumpVec::new_in(bump),
                results: BumpVec::from_iter_in([OperandType::Any], bump),
            }),
        "struct.new_default" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
            .map(|symbol| ResolvedSig {
                params: BumpVec::new_in(bump),
                results: BumpVec::from_iter_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty: HeapType::Type(symbol.idx),
                        nullable: false,
                    }))],
                    bump,
                ),
            })
            .unwrap_or_else(|| ResolvedSig {
                params: BumpVec::new_in(bump),
                results: BumpVec::from_iter_in([OperandType::Any], bump),
            }),
        "struct.get" => {
            let mut immediates = instr.children_by_kind(SyntaxKind::IMMEDIATE);
            immediates
                .next()
                .zip(immediates.next())
                .and_then(|(struct_ref, field_ref)| {
                    resolve_field_type_with_struct_idx(ctx.db, ctx.document, struct_ref.to_ptr(), field_ref.to_ptr())
                })
                .map(|(idx, ty)| ResolvedSig {
                    params: BumpVec::from_iter_in(
                        [OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(idx),
                            nullable: true,
                        }))],
                        bump,
                    ),
                    results: BumpVec::from_iter_in([ty.unwrap_or(OperandType::Any)], bump),
                })
                .unwrap_or_else(|| ResolvedSig {
                    params: BumpVec::new_in(bump),
                    results: BumpVec::from_iter_in([OperandType::Any], bump),
                })
        }
        "struct.get_s" | "struct.get_u" => ResolvedSig {
            params: instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                .map(|symbol| {
                    BumpVec::from_iter_in(
                        [OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(symbol.idx),
                            nullable: true,
                        }))],
                        bump,
                    )
                })
                .unwrap_or_else(|| BumpVec::new_in(bump)),
            results: BumpVec::from_iter_in([OperandType::Val(ValType::I32)], bump),
        },
        "struct.set" => {
            let mut immediates = instr.children_by_kind(SyntaxKind::IMMEDIATE);
            immediates
                .next()
                .zip(immediates.next())
                .and_then(|(struct_ref, field_ref)| {
                    resolve_field_type_with_struct_idx(ctx.db, ctx.document, struct_ref.to_ptr(), field_ref.to_ptr())
                })
                .map(|(idx, ty)| ResolvedSig {
                    params: BumpVec::from_iter_in(
                        [
                            OperandType::Val(ValType::Ref(RefType {
                                heap_ty: HeapType::Type(idx),
                                nullable: true,
                            })),
                            ty.unwrap_or(OperandType::Any),
                        ],
                        bump,
                    ),
                    results: BumpVec::new_in(bump),
                })
                .unwrap_or_else(|| ResolvedSig {
                    params: BumpVec::from_iter_in([OperandType::Any], bump),
                    results: BumpVec::new_in(bump),
                })
        }
        "array.new" => {
            let mut sig = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| resolve_array_type_with_idx(ctx.symbol_table, ctx.def_types, immediate.to_ptr()))
                .map(|(idx, ty)| ResolvedSig {
                    params: BumpVec::from_iter_in([ty.unwrap_or(OperandType::Any)], bump),
                    results: BumpVec::from_iter_in(
                        [OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(idx),
                            nullable: false,
                        }))],
                        bump,
                    ),
                })
                .unwrap_or_else(|| ResolvedSig {
                    params: BumpVec::new_in(bump),
                    results: BumpVec::from_iter_in([OperandType::Any], bump),
                });
            sig.params.push(OperandType::Val(ValType::I32));
            sig
        }
        "array.new_default" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
            .map(|symbol| ResolvedSig {
                params: BumpVec::from_iter_in([OperandType::Val(ValType::I32)], bump),
                results: BumpVec::from_iter_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty: HeapType::Type(symbol.idx),
                        nullable: false,
                    }))],
                    bump,
                ),
            })
            .unwrap_or_else(|| ResolvedSig {
                params: BumpVec::new_in(bump),
                results: BumpVec::from_iter_in([OperandType::Any], bump),
            }),
        "array.new_fixed" => {
            let mut immediates = instr.children_by_kind(SyntaxKind::IMMEDIATE);
            immediates
                .next()
                .and_then(|immediate| resolve_array_type_with_idx(ctx.symbol_table, ctx.def_types, immediate.to_ptr()))
                .map(|(idx, ty)| {
                    let count = immediates
                        .next()
                        .and_then(|immediate| {
                            immediate
                                .green()
                                .children()
                                .find_map(|node_or_token| match node_or_token {
                                    NodeOrToken::Token(token) if token.kind() == SyntaxKind::INT => Some(token),
                                    _ => None,
                                })
                        })
                        .and_then(|int| helpers::parse_u32(int.text()).ok())
                        .unwrap_or_default();
                    ResolvedSig {
                        params: BumpVec::from_iter_in(
                            iter::repeat_n(ty.unwrap_or(OperandType::Any), count as usize),
                            bump,
                        ),
                        results: BumpVec::from_iter_in(
                            [OperandType::Val(ValType::Ref(RefType {
                                heap_ty: HeapType::Type(idx),
                                nullable: false,
                            }))],
                            bump,
                        ),
                    }
                })
                .unwrap_or_else(|| ResolvedSig {
                    params: BumpVec::new_in(bump),
                    results: BumpVec::from_iter_in([OperandType::Any], bump),
                })
        }
        "array.new_data" | "array.new_elem" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
            .map(|symbol| ResolvedSig {
                params: BumpVec::from_iter_in(iter::repeat_n(OperandType::Val(ValType::I32), 2), bump),
                results: BumpVec::from_iter_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty: HeapType::Type(symbol.idx),
                        nullable: false,
                    }))],
                    bump,
                ),
            })
            .unwrap_or_else(|| ResolvedSig {
                params: BumpVec::new_in(bump),
                results: BumpVec::from_iter_in([OperandType::Any], bump),
            }),
        "array.get" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|immediate| resolve_array_type_with_idx(ctx.symbol_table, ctx.def_types, immediate.to_ptr()))
            .map(|(idx, ty)| ResolvedSig {
                params: BumpVec::from_iter_in(
                    [
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(idx),
                            nullable: true,
                        })),
                        OperandType::Val(ValType::I32),
                    ],
                    bump,
                ),
                results: BumpVec::from_iter_in([ty.unwrap_or(OperandType::Any)], bump),
            })
            .unwrap_or_else(|| ResolvedSig {
                params: BumpVec::new_in(bump),
                results: BumpVec::from_iter_in([OperandType::Any], bump),
            }),
        "array.get_s" | "array.get_u" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
            .map(|symbol| ResolvedSig {
                params: BumpVec::from_iter_in(
                    [
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(symbol.idx),
                            nullable: true,
                        })),
                        OperandType::Val(ValType::I32),
                    ],
                    bump,
                ),
                results: BumpVec::from_iter_in([OperandType::Val(ValType::I32)], bump),
            })
            .unwrap_or_else(|| ResolvedSig {
                params: BumpVec::new_in(bump),
                results: BumpVec::from_iter_in([OperandType::Val(ValType::I32)], bump),
            }),
        "array.set" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|immediate| resolve_array_type_with_idx(ctx.symbol_table, ctx.def_types, immediate.to_ptr()))
            .map(|(idx, ty)| ResolvedSig {
                params: BumpVec::from_iter_in(
                    [
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(idx),
                            nullable: true,
                        })),
                        OperandType::Val(ValType::I32),
                        ty.unwrap_or(OperandType::Any),
                    ],
                    bump,
                ),
                results: BumpVec::new_in(bump),
            })
            .unwrap_or_else(|| ResolvedSig::new_in(bump)),
        "array.fill" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|immediate| resolve_array_type_with_idx(ctx.symbol_table, ctx.def_types, immediate.to_ptr()))
            .map(|(idx, ty)| ResolvedSig {
                params: BumpVec::from_iter_in(
                    [
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(idx),
                            nullable: true,
                        })),
                        OperandType::Val(ValType::I32),
                        ty.unwrap_or(OperandType::Any),
                        OperandType::Val(ValType::I32),
                    ],
                    bump,
                ),
                results: BumpVec::new_in(bump),
            })
            .unwrap_or_else(|| ResolvedSig::new_in(bump)),
        "array.copy" => {
            let mut immediates = instr.children_by_kind(SyntaxKind::IMMEDIATE);
            immediates
                .next()
                .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
                .zip(
                    immediates
                        .next()
                        .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into())),
                )
                .map(|(dst, src)| ResolvedSig {
                    params: BumpVec::from_iter_in(
                        [
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
                        bump,
                    ),
                    results: BumpVec::new_in(bump),
                })
                .unwrap_or_else(|| ResolvedSig::new_in(bump))
        }
        "array.init_data" | "array.init_elem" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|idx| ctx.symbol_table.find_def(idx.to_ptr().into()))
            .map(|symbol| ResolvedSig {
                params: BumpVec::from_iter_in(
                    [
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty: HeapType::Type(symbol.idx),
                            nullable: true,
                        })),
                        OperandType::Val(ValType::I32),
                        OperandType::Val(ValType::I32),
                        OperandType::Val(ValType::I32),
                    ],
                    bump,
                ),
                results: BumpVec::new_in(bump),
            })
            .unwrap_or_else(|| ResolvedSig::new_in(bump)),
        "ref.null" => {
            let ty = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| immediate.green().children().next())
                .and_then(|node_or_token| match node_or_token {
                    NodeOrToken::Node(node) if node.kind() == SyntaxKind::HEAP_TYPE => {
                        HeapType::from_green(node, ctx.db)
                    }
                    NodeOrToken::Token(token) if token.kind() == SyntaxKind::IDENT => Some(HeapType::Type(Idx {
                        num: None,
                        name: Some(InternIdent::new(ctx.db, token.text())),
                    })),
                    NodeOrToken::Token(token) if token.kind() == SyntaxKind::INT => Some(HeapType::Type(Idx {
                        num: helpers::parse_u32(token.text()).ok(),
                        name: None,
                    })),
                    _ => None,
                })
                .map_or(OperandType::Any, |heap_ty| {
                    OperandType::Val(ValType::Ref(RefType {
                        heap_ty,
                        nullable: true,
                    }))
                });
            ResolvedSig {
                params: BumpVec::new_in(bump),
                results: BumpVec::from_iter_in([ty], bump),
            }
        }
        "ref.is_null" => {
            let heap_ty =
                if let Some((OperandType::Val(ValType::Ref(RefType { heap_ty, .. })), _)) = type_stack.stack.last() {
                    heap_ty.clone()
                } else {
                    HeapType::Any
                };
            ResolvedSig {
                params: BumpVec::from_iter_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty,
                        nullable: true,
                    }))],
                    bump,
                ),
                results: BumpVec::from_iter_in([OperandType::Val(ValType::I32)], bump),
            }
        }
        "ref.as_non_null" => {
            let heap_ty =
                if let Some((OperandType::Val(ValType::Ref(RefType { heap_ty, .. })), _)) = type_stack.stack.last() {
                    heap_ty.clone()
                } else {
                    HeapType::Any
                };
            ResolvedSig {
                params: BumpVec::from_iter_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty: heap_ty.clone(),
                        nullable: true,
                    }))],
                    bump,
                ),
                results: BumpVec::from_iter_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty: heap_ty.clone(),
                        nullable: false,
                    }))],
                    bump,
                ),
            }
        }
        "ref.test" => {
            let heap_ty = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| immediate.children_by_kind(SyntaxKind::REF_TYPE).next())
                .and_then(|ref_type| RefType::from_green(ref_type.green(), ctx.db))
                .and_then(|ref_type| ref_type.heap_ty.to_top_type(ctx.db, ctx.document, ctx.module_id))
                .unwrap_or(HeapType::Any);
            ResolvedSig {
                params: BumpVec::from_iter_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty,
                        nullable: true,
                    }))],
                    bump,
                ),
                results: BumpVec::from_iter_in([OperandType::Val(ValType::I32)], bump),
            }
        }
        "ref.cast" => {
            let ref_type = instr
                .children_by_kind(SyntaxKind::IMMEDIATE)
                .next()
                .and_then(|immediate| immediate.children_by_kind(SyntaxKind::REF_TYPE).next())
                .and_then(|ref_type| RefType::from_green(ref_type.green(), ctx.db))
                .unwrap_or(RefType {
                    heap_ty: HeapType::Any,
                    nullable: true,
                });
            let heap_ty = ref_type
                .heap_ty
                .to_top_type(ctx.db, ctx.document, ctx.module_id)
                .unwrap_or(HeapType::Any);
            ResolvedSig {
                params: BumpVec::from_iter_in(
                    [OperandType::Val(ValType::Ref(RefType {
                        heap_ty,
                        nullable: true,
                    }))],
                    bump,
                ),
                results: BumpVec::from_iter_in([OperandType::Val(ValType::Ref(ref_type))], bump),
            }
        }
        "ref.func" => {
            let immediate = instr.children_by_kind(SyntaxKind::IMMEDIATE).next();
            let heap_ty = immediate
                .as_ref()
                .and_then(|immediate| ctx.symbol_table.find_def(immediate.to_ptr().into()))
                .and_then(|def_symbol| def_symbol.amber().children_by_kind(SyntaxKind::TYPE_USE).next())
                .and_then(|type_use| type_use.children_by_kind(SyntaxKind::INDEX).next())
                .and_then(|index| Idx::from_green_for_ref(index.green(), ctx.db))
                .map(HeapType::Type)
                .or_else(|| {
                    immediate
                        .and_then(|immediate| Idx::from_green_for_ref(immediate.green(), ctx.db))
                        .map(HeapType::DefFunc)
                });
            ResolvedSig {
                params: BumpVec::new_in(bump),
                results: BumpVec::from_iter_in(
                    [heap_ty.map_or(OperandType::Any, |heap_ty| {
                        OperandType::Val(ValType::Ref(RefType {
                            heap_ty,
                            nullable: false,
                        }))
                    })],
                    bump,
                ),
            }
        }
        "call_ref" | "return_call_ref" => instr
            .children_by_kind(SyntaxKind::IMMEDIATE)
            .next()
            .and_then(|immediate| ctx.symbol_table.resolved.get(&immediate.to_ptr().into()))
            .and_then(|key| ctx.def_types.get(key))
            .map(|def_type| {
                let mut sig = def_type
                    .comp
                    .as_func()
                    .map(|sig| ResolvedSig {
                        params: BumpVec::from_iter_in(
                            sig.params.iter().map(|(ty, _)| OperandType::Val(ty.clone())),
                            bump,
                        ),
                        results: BumpVec::from_iter_in(sig.results.iter().map(|ty| OperandType::Val(ty.clone())), bump),
                    })
                    .unwrap_or_else(|| ResolvedSig::new_in(bump));
                sig.params.push(OperandType::Val(ValType::Ref(RefType {
                    heap_ty: HeapType::Type(def_type.idx),
                    nullable: true,
                })));
                sig
            })
            .unwrap_or_else(|| ResolvedSig::new_in(bump)),
        _ => data_set::INSTR_SIG
            .get(instr_name)
            .map(|sig| ResolvedSig {
                params: BumpVec::from_iter_in(sig.params.iter().cloned(), bump),
                results: BumpVec::from_iter_in(sig.results.iter().cloned(), bump),
            })
            .unwrap_or_else(|| ResolvedSig::new_in(bump)),
    }
}

enum ReportRange<'a> {
    Instr(AmberNode<'a>),
    Keyword(AmberNode<'a>),
    Last(AmberNode<'a>),
}
impl ReportRange<'_> {
    fn pick(&self) -> TextRange {
        match self {
            ReportRange::Instr(node) => {
                if BlockInstr::can_cast(node.kind()) {
                    node.children_by_kind(SyntaxKind::TYPE_USE)
                        .next()
                        .map(|type_use| type_use.text_range())
                        .unwrap_or_else(|| node.text_range())
                } else {
                    node.text_range()
                }
            }
            ReportRange::Keyword(node) => node
                .tokens_by_kind(SyntaxKind::KEYWORD)
                .next()
                .map(|token| token.text_range())
                .unwrap_or_else(|| node.text_range()),
            ReportRange::Last(node) => node
                .children_with_tokens()
                .next_back()
                .map(|node_or_token| node_or_token.text_range())
                .unwrap_or_else(|| node.text_range()),
        }
    }
}
