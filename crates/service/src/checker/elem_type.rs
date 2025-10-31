use crate::{
    LanguageService,
    binder::{SymbolKey, SymbolTable},
    document::Document,
    helpers,
    types_analyzer::RefType,
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, Union2};
use rowan::ast::AstNode;
use wat_syntax::{
    SyntaxNode,
    ast::{ModuleFieldElem, ModuleFieldTable},
};

const DIAGNOSTIC_CODE: &str = "elem-type";

pub fn check(
    service: &LanguageService,
    document: Document,
    line_index: &LineIndex,
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
    let table_ref_type = RefType::from_green(&table_ref_type_node.syntax().green(), service)?;
    let elem_ref_type_node = elem.elem_list()?.ref_type()?;
    let elem_ref_type = RefType::from_green(&elem_ref_type_node.syntax().green(), service)?;
    if elem_ref_type.matches(&table_ref_type, service, document, module_id) {
        None
    } else {
        Some(Diagnostic {
            range: helpers::rowan_range_to_lsp_range(
                line_index,
                elem_ref_type_node.syntax().text_range(),
            ),
            severity: Some(DiagnosticSeverity::Error),
            source: Some("wat".into()),
            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
            message: format!(
                "ref type `{}` doesn't match the table's ref type `{}`",
                elem_ref_type.render(service),
                table_ref_type.render(service),
            ),
            related_information: Some(vec![DiagnosticRelatedInformation {
                location: Location {
                    uri: document.uri(service).raw(service),
                    range: helpers::rowan_range_to_lsp_range(
                        line_index,
                        table_ref_type_node.syntax().text_range(),
                    ),
                },
                message: "table's ref type declared here".into(),
            }]),
            ..Default::default()
        })
    }
}
