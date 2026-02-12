use super::{Diagnostic, RelatedInformation};
use crate::{
    binder::{SymbolKey, SymbolTable},
    document::Document,
    types_analyzer::RefType,
};
use wat_syntax::{
    SyntaxNode,
    ast::{AstNode, ModuleFieldElem, ModuleFieldTable},
};

const DIAGNOSTIC_CODE: &str = "elem-type";

pub fn check(
    db: &dyn salsa::Database,
    document: Document,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
    module_id: u32,
    node: &SyntaxNode,
) -> Option<Diagnostic> {
    let elem = ModuleFieldElem::cast(node.clone())?;
    let table = ModuleFieldTable::cast(
        symbol_table
            .find_def(SymbolKey::new(elem.table_use()?.index()?.syntax()))?
            .key
            .to_node(root),
    )?;
    let table_ref_type_node = table.table_type()?.ref_type()?;
    let table_ref_type = RefType::from_green(table_ref_type_node.syntax().green(), db)?;
    let elem_ref_type_node = elem.elem_list()?.ref_type()?;
    let elem_ref_type = RefType::from_green(elem_ref_type_node.syntax().green(), db)?;
    if elem_ref_type.matches(&table_ref_type, db, document, module_id) {
        None
    } else {
        Some(Diagnostic {
            range: elem_ref_type_node.syntax().text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message: format!(
                "ref type `{}` doesn't match the table's ref type `{}`",
                elem_ref_type.render(db),
                table_ref_type.render(db),
            ),
            related_information: Some(vec![RelatedInformation {
                range: table_ref_type_node.syntax().text_range(),
                message: "table's ref type declared here".into(),
            }]),
            ..Default::default()
        })
    }
}
