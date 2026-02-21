use super::{Diagnostic, DiagnosticCtx};
use crate::types_analyzer::CompositeType;
use wat_syntax::{AmberNode, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "block-type";

pub fn check(ctx: &DiagnosticCtx, node: AmberNode) -> Option<Diagnostic> {
    let index = node
        .children_by_kind(SyntaxKind::TYPE_USE)
        .next()
        .and_then(|type_use| type_use.children_by_kind(SyntaxKind::INDEX).next())?;
    if ctx
        .symbol_table
        .resolved
        .get(&index.to_ptr().into())
        .and_then(|key| ctx.def_types.get(key))
        .is_some_and(|def_type| !matches!(def_type.comp, CompositeType::Func(..)))
    {
        Some(Diagnostic {
            range: index.text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message: "block type must be function type".into(),
            ..Default::default()
        })
    } else {
        None
    }
}
