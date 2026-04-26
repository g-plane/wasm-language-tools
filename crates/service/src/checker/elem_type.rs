use super::{Diagnostic, DiagnosticCtx};
use crate::types_analyzer::{self, RefType};
use wat_syntax::{AmberNode, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "elem-type";

pub fn check(ctx: &DiagnosticCtx, node: AmberNode) -> Option<Diagnostic> {
    let table = ctx.symbol_table.find_def(
        node.children_by_kind(SyntaxKind::TABLE_USE)
            .next()?
            .children_by_kind(SyntaxKind::INDEX)
            .next()?
            .to_ptr()
            .into(),
    )?;
    let table_ref_type = types_analyzer::extract_table_ref_type(ctx.db, &table.green)?;
    let elem_ref_type_node = node
        .children_by_kind(SyntaxKind::ELEM_LIST)
        .next()?
        .children_by_kind(SyntaxKind::REF_TYPE)
        .next()?;
    let elem_ref_type = RefType::from_green(elem_ref_type_node.green(), ctx.db)?;
    if elem_ref_type.matches(&table_ref_type, ctx.db, ctx.document, ctx.module_id) {
        None
    } else {
        Some(Diagnostic {
            range: elem_ref_type_node.text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message: format!(
                "ref type `{}` doesn't match ref type `{}` of table `{}`",
                elem_ref_type.render(ctx.db),
                table_ref_type.render(ctx.db),
                table.idx.render(ctx.db),
            ),
            ..Default::default()
        })
    }
}
