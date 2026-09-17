use super::{
    instr::{InstrSigResolverCtx, ResolvedSig, resolve_instr_sig},
    signature::Sig,
    types::OperandType,
};
use crate::helpers;
use bumpalo::collections::Vec as BumpVec;
use std::ops::ControlFlow;
use wat_syntax::{
    AmberNode, SyntaxKind,
    ast::{AstNode, Instr},
};

type TypeStack<'db, 'bump> = BumpVec<'bump, OperandType<'db>>;

pub(crate) fn perform_types_till<'db, 'bump>(
    target: AmberNode<'db>,
    (outer_block, init_types_block): (AmberNode<'db>, Option<AmberNode<'db>>),
    ctx: &InstrSigResolverCtx<'db, 'bump>,
) -> Option<(TypeStack<'db, 'bump>, ResolvedSig<'db, 'bump>)> {
    fn unfold<'db, 'bump>(
        node: AmberNode<'db>,
        ctx: &InstrSigResolverCtx<'db, 'bump>,
        stack: &mut TypeStack<'db, 'bump>,
        target: AmberNode<'db>,
    ) -> ControlFlow<ResolvedSig<'db, 'bump>> {
        let kind = node.kind();
        if matches!(kind, SyntaxKind::PLAIN_INSTR | SyntaxKind::BLOCK_IF) {
            node.children_by_kind(Instr::can_cast)
                .try_for_each(|child| unfold(child, ctx, stack, target))?;
        }
        match kind {
            SyntaxKind::PLAIN_INSTR => {
                if let Some(instr_name) = node.tokens_by_kind(SyntaxKind::INSTR_NAME).next() {
                    let instr_name = instr_name.text();
                    let mut sig = resolve_instr_sig(ctx, instr_name, node, stack);
                    if node == target {
                        ControlFlow::Break(sig)
                    } else {
                        stack.truncate(stack.len().saturating_sub(sig.params.len()));
                        if helpers::is_stack_polymorphic(instr_name) {
                            stack.clear();
                        }
                        stack.append(&mut sig.results);
                        ControlFlow::Continue(())
                    }
                } else {
                    ControlFlow::Continue(())
                }
            }
            SyntaxKind::BLOCK_BLOCK | SyntaxKind::BLOCK_LOOP | SyntaxKind::BLOCK_TRY_TABLE => {
                let sig = Sig::from_func(ctx.db, ctx.document, node);
                stack.truncate(stack.len().saturating_sub(sig.params.len()));
                stack.extend(sig.results.iter().map(|ty| OperandType::Val(ty.clone())));
                ControlFlow::Continue(())
            }
            SyntaxKind::BLOCK_IF => {
                let sig = Sig::from_func(ctx.db, ctx.document, node);
                stack.truncate(stack.len().saturating_sub(sig.params.len() + 1));
                stack.extend(sig.results.iter().map(|ty| OperandType::Val(ty.clone())));
                ControlFlow::Continue(())
            }
            _ => ControlFlow::Continue(()),
        }
    }

    let mut stack = init_types_block
        .map(|block| {
            BumpVec::from_iter_in(
                Sig::from_func(ctx.db, ctx.document, block)
                    .params
                    .into_iter()
                    .map(OperandType::Val),
                ctx.bump,
            )
        })
        .unwrap_or_else(|| BumpVec::new_in(ctx.bump));
    outer_block
        .children_by_kind(Instr::can_cast)
        .try_for_each(|child| unfold(child, ctx, &mut stack, target))
        .break_value()
        .map(|sig| (stack, sig))
}
