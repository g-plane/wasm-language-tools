use super::{Diagnostic, DiagnosticCtx};
use crate::types_analyzer::CompositeType;
use wat_syntax::{AmberNode, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "tag-type";

pub fn check(diagnostics: &mut Vec<Diagnostic>, ctx: &DiagnosticCtx, node: AmberNode) {
    let Some(type_use) = node.children_by_kind(SyntaxKind::TYPE_USE).next() else {
        return;
    };
    if let Some(index) = type_use.children_by_kind(SyntaxKind::INDEX).next()
        && ctx
            .symbol_table
            .resolved
            .get(&index.to_ptr().into())
            .and_then(|def_key| ctx.def_types.get(def_key))
            .is_some_and(|def_type| !matches!(def_type.comp, CompositeType::Func(..)))
    {
        diagnostics.push(Diagnostic {
            range: index.text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message: "tag type must be function type".into(),
            ..Default::default()
        });
    }
}
