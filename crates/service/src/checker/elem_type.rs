use super::{Diagnostic, RelatedInformation};
use crate::{binder::SymbolTable, document::Document, types_analyzer::RefType};
use wat_syntax::{AmberNode, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "elem-type";

pub fn check(
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
    module_id: u32,
    node: AmberNode,
) -> Option<Diagnostic> {
    let table = symbol_table
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
    let table_ref_type = RefType::from_green(table_ref_type_node.green(), db)?;
    let elem_ref_type_node = node
        .children_by_kind(SyntaxKind::ELEM_LIST)
        .next()?
        .children_by_kind(SyntaxKind::REF_TYPE)
        .next()?;
    let elem_ref_type = RefType::from_green(elem_ref_type_node.green(), db)?;
    if elem_ref_type.matches(&table_ref_type, db, document, module_id) {
        None
    } else {
        Some(Diagnostic {
            range: elem_ref_type_node.text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message: format!(
                "ref type `{}` doesn't match the table's ref type `{}`",
                elem_ref_type.render(db),
                table_ref_type.render(db),
            ),
            related_information: Some(vec![RelatedInformation {
                range: table_ref_type_node.text_range(),
                message: "table's ref type declared here".into(),
            }]),
            ..Default::default()
        })
    }
}
