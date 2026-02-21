use super::{Diagnostic, DiagnosticCtx};
use crate::types_analyzer;
use wat_syntax::{AmberNode, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "start";

pub fn check(ctx: &DiagnosticCtx, node: AmberNode) -> Option<Diagnostic> {
    let index = node.children_by_kind(SyntaxKind::INDEX).next()?;
    if ctx
        .symbol_table
        .find_def(index.to_ptr().into())
        .map(|func| types_analyzer::get_func_sig(ctx.db, ctx.document, func.key, &func.green))
        .is_some_and(|sig| !sig.params.is_empty() || !sig.results.is_empty())
    {
        Some(Diagnostic {
            range: index.text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message: "start function must be type of [] -> []".into(),
            ..Default::default()
        })
    } else {
        None
    }
}
