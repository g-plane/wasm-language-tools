use crate::{
    binder::{SymbolKey, SymbolTable},
    helpers,
    types_analyzer::RefType,
    uri::InternUri,
    LanguageService, UrisCtx,
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, Union2};
use rowan::ast::AstNode;
use wat_syntax::{
    ast::{ModuleFieldElem, ModuleFieldTable},
    SyntaxNode,
};

const DIAGNOSTIC_CODE: &str = "elem-type";

#[expect(clippy::too_many_arguments)]
pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
    module_id: u32,
    node: &SyntaxNode,
) -> Option<()> {
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
    if !elem_ref_type.matches(&table_ref_type, service, uri, module_id) {
        diagnostics.push(Diagnostic {
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
                    uri: service.lookup_uri(uri),
                    range: helpers::rowan_range_to_lsp_range(
                        line_index,
                        table_ref_type_node.syntax().text_range(),
                    ),
                },
                message: "table's ref type declared here".into(),
            }]),
            ..Default::default()
        });
    }
    Some(())
}
