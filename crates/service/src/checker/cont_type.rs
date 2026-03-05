use super::{Diagnostic, DiagnosticCtx, RelatedInformation};
use crate::{binder::SymbolKey, types_analyzer::CompositeType};
use wat_syntax::{AmberNode, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "cont-type";

pub fn check(ctx: &DiagnosticCtx, node: AmberNode) -> Option<Diagnostic> {
    let index = node.children_by_kind(SyntaxKind::INDEX).next()?;
    let ref_symbol = ctx.symbol_table.symbols.get(&SymbolKey::from(index.to_ptr()))?;
    let def_key = ctx.symbol_table.resolved.get(&ref_symbol.key)?;
    if matches!(ctx.def_types.get(def_key)?.comp, CompositeType::Func(..)) {
        None
    } else {
        Some(Diagnostic {
            range: index.text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message: format!("type `{}` must be a function type", ref_symbol.idx.render(ctx.db)),
            related_information: Some(vec![RelatedInformation {
                range: def_key.text_range(),
                message: format!("type `{}` defined here", ref_symbol.idx.render(ctx.db)),
            }]),
            ..Default::default()
        })
    }
}
