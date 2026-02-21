use super::{Diagnostic, DiagnosticCtx, RelatedInformation};
use crate::types_analyzer::RefType;
use wat_syntax::{AmberNode, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "elem-type";

pub fn check(ctx: &DiagnosticCtx, node: AmberNode) -> Option<Diagnostic> {
    let table = ctx
        .symbol_table
        .find_def(
            node.children_by_kind(SyntaxKind::TABLE_USE)
                .next()?
                .children_by_kind(SyntaxKind::INDEX)
                .next()?
                .to_ptr()
                .into(),
        )?
        .amber();
    let table_ref_type_node = table
        .children_by_kind(SyntaxKind::TABLE_TYPE)
        .next()?
        .children_by_kind(SyntaxKind::REF_TYPE)
        .next()?;
    let table_ref_type = RefType::from_green(table_ref_type_node.green(), ctx.db)?;
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
                "ref type `{}` doesn't match the table's ref type `{}`",
                elem_ref_type.render(ctx.db),
                table_ref_type.render(ctx.db),
            ),
            related_information: Some(vec![RelatedInformation {
                range: table_ref_type_node.text_range(),
                message: "table's ref type declared here".into(),
            }]),
            ..Default::default()
        })
    }
}
