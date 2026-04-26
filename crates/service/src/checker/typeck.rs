use super::{Diagnostic, DiagnosticCtx, RelatedInformation};
use crate::{
    helpers,
    types_analyzer::{
        InstrSigResolverCtx, OperandType, RefType, Sig, ValType, extract_global_type, extract_table_ref_type,
        join_types, resolve_instr_sig,
    },
};
use bumpalo::collections::Vec as BumpVec;
use itertools::{EitherOrBoth, Itertools};
use std::iter;
use wat_syntax::{
    AmberNode, SyntaxKind, TextRange,
    ast::{AstNode, BlockInstr, Instr},
};

const DIAGNOSTIC_CODE: &str = "type-check";

pub fn check_func(diagnostics: &mut Vec<Diagnostic>, ctx: &mut DiagnosticCtx, node: AmberNode) {
    {
        let func_key = node.to_ptr().into();
        let results = BumpVec::from_iter_in(
            Sig::from_func(ctx.db, ctx.document, node)
                .results
                .into_iter()
                .map(OperandType::Val),
            ctx.bump,
        );
        let imported = ctx.imports.contains(&func_key);
        check_block_like(
            diagnostics,
            ctx,
            node,
            if imported {
                BumpVec::from_iter_in(results.iter().cloned(), ctx.bump)
            } else {
                BumpVec::with_capacity_in(2, ctx.bump)
            },
            if imported {
                BumpVec::from_iter_in(iter::repeat_n(None, results.len()), ctx.bump)
            } else {
                BumpVec::new_in(ctx.bump)
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
    let imported = ctx.imports.contains(&node.to_ptr().into());
    check_block_like(
        diagnostics,
        ctx,
        node,
        if imported {
            BumpVec::from_iter_in([ty.clone()], ctx.bump)
        } else {
            BumpVec::with_capacity_in(1, ctx.bump)
        },
        if imported {
            BumpVec::from_iter_in([None], ctx.bump)
        } else {
            BumpVec::new_in(ctx.bump)
        },
        &[ty],
    );
    ctx.bump.reset();
}

pub fn check_table(diagnostics: &mut Vec<Diagnostic>, ctx: &mut DiagnosticCtx, node: AmberNode) {
    let Some(ref_type) = extract_table_ref_type(ctx.db, node.green()) else {
        return;
    };
    let ty = ValType::Ref(ref_type);
    if ty.defaultable() && node.children_by_kind(Instr::can_cast).next().is_none() {
        return;
    }
    let ty = OperandType::Val(ty);
    let imported = ctx.imports.contains(&node.to_ptr().into());
    check_block_like(
        diagnostics,
        ctx,
        node,
        if imported {
            BumpVec::from_iter_in([ty.clone()], ctx.bump)
        } else {
            BumpVec::with_capacity_in(1, ctx.bump)
        },
        if imported {
            BumpVec::from_iter_in([None], ctx.bump)
        } else {
            BumpVec::new_in(ctx.bump)
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
    init_stack: BumpVec<OperandType>,
    init_producers: BumpVec<Option<AmberNode>>,
    expected_results: &[OperandType],
) {
    let mut type_stack = TypeStack {
        ctx,
        stack: init_stack,
        producers: init_producers,
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
        let mut sig = resolve_instr_sig(
            &InstrSigResolverCtx {
                db: ctx.db,
                document: ctx.document,
                symbol_table: ctx.symbol_table,
                def_types: ctx.def_types,
                module: ctx.module,
                module_id: ctx.module_id,
                bump: ctx.bump,
            },
            instr_name,
            node,
            &type_stack.stack,
        );
        if let Some(diagnostic) = type_stack.check(&sig.params, ReportRange::Instr(node)) {
            diagnostics.push(diagnostic);
        }
        if helpers::is_stack_polymorphic(instr_name) {
            type_stack.has_never = true;
            type_stack.stack.clear();
            type_stack.producers.clear();
        }
        type_stack
            .producers
            .extend(iter::repeat_n(Some(node), sig.results.len()));
        type_stack.stack.append(&mut sig.results);
    } else if BlockInstr::can_cast(node.kind()) {
        let signature = Sig::from_func(ctx.db, ctx.document, node);
        let init_stack =
            BumpVec::from_iter_in(signature.params.iter().map(|ty| OperandType::Val(ty.clone())), ctx.bump);
        let init_producers = BumpVec::from_iter_in(iter::repeat_n(Some(node), signature.params.len()), ctx.bump);
        let mut results = BumpVec::from_iter_in(signature.results.into_iter().map(OperandType::Val), ctx.bump);
        match node.kind() {
            SyntaxKind::BLOCK_BLOCK | SyntaxKind::BLOCK_LOOP | SyntaxKind::BLOCK_TRY_TABLE => {
                if let Some(diagnostic) = type_stack.check(
                    &BumpVec::from_iter_in(signature.params.into_iter().map(OperandType::Val), ctx.bump),
                    ReportRange::Instr(node),
                ) {
                    diagnostics.push(diagnostic);
                }
                check_block_like(diagnostics, ctx, node, init_stack, init_producers, &results);
            }
            SyntaxKind::BLOCK_IF => {
                if let Some(mut diagnostic) =
                    type_stack.check(&[OperandType::Val(ValType::I32)], ReportRange::Keyword(node))
                {
                    diagnostic.message.push_str(" for the condition of `if` block");
                    diagnostics.push(diagnostic);
                }
                if let Some(diagnostic) = type_stack.check(
                    &BumpVec::from_iter_in(signature.params.into_iter().map(OperandType::Val), ctx.bump),
                    ReportRange::Instr(node),
                ) {
                    diagnostics.push(diagnostic);
                }
                let mut children = node.children();
                if let Some(then_block) = children.find(|child| child.kind() == SyntaxKind::BLOCK_IF_THEN) {
                    check_block_like(
                        diagnostics,
                        ctx,
                        then_block,
                        init_stack.clone(),
                        BumpVec::from_iter_in(init_producers.iter().cloned(), ctx.bump),
                        &results,
                    );
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
                    check_block_like(diagnostics, ctx, else_block, init_stack, init_producers, &results);
                } else {
                    let mut type_stack = TypeStack {
                        ctx,
                        stack: init_stack,
                        producers: init_producers,
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
        type_stack.producers.extend(iter::repeat_n(Some(node), results.len()));
        type_stack.stack.append(&mut results);
    }
}

struct TypeStack<'db, 'bump> {
    ctx: &'db DiagnosticCtx<'db, 'bump>,
    stack: BumpVec<'bump, OperandType<'db>>,
    producers: BumpVec<'bump, Option<AmberNode<'db>>>,
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
            .zip_longest(pops.iter().zip(self.producers.drain(rest_len..)).rev())
            .for_each(|pair| match pair {
                EitherOrBoth::Both(expected, (received, related_instr)) => {
                    if received.matches(expected, self.ctx.db, self.ctx.document, self.ctx.module_id) {
                        return;
                    }
                    mismatch = true;
                    if let Some(related_instr) = related_instr {
                        related_information.push(RelatedInformation {
                            range: ReportRange::Instr(related_instr).pick(),
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
                        pops.iter(),
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
            .zip_longest(self.stack.iter().zip(&self.producers).rev())
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
                    join_types(self.ctx.db, self.stack.iter(), "", self.ctx.bump),
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
                        .map(|ty| match ty {
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
