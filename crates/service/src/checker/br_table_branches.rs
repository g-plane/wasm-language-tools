use super::{Diagnostic, DiagnosticCtx};
use crate::types_analyzer::{join_types, resolve_br_types};
use bumpalo::collections::Vec as BumpVec;
use wat_syntax::{AmberNode, AmberToken, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "br-table-branches";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    ctx: &DiagnosticCtx,
    node: AmberNode,
    instr_name: AmberToken,
) -> Option<()> {
    if instr_name.text() != "br_table" {
        return None;
    }
    let mut immediates = node.children_by_kind(SyntaxKind::IMMEDIATE);
    let expected = BumpVec::from_iter_in(
        immediates.next().and_then(|immediate| {
            resolve_br_types(ctx.db, ctx.document, ctx.symbol_table, immediate.to_ptr().into())
        })?,
        ctx.bump,
    );
    diagnostics.extend(immediates.filter_map(|immediate| {
        let received = BumpVec::from_iter_in(
            resolve_br_types(ctx.db, ctx.document, ctx.symbol_table, immediate.to_ptr().into())?,
            ctx.bump,
        );
        if received != expected {
            Some(Diagnostic {
                range: immediate.text_range(),
                code: DIAGNOSTIC_CODE.into(),
                message: format!(
                    "type mismatch in `br_table`: expected {}, found {}",
                    join_types(ctx.db, expected.iter(), "", ctx.bump),
                    join_types(ctx.db, received.iter(), "", ctx.bump),
                ),
                ..Default::default()
            })
        } else {
            None
        }
    }));
    Some(())
}
