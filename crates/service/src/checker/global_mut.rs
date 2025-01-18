use crate::{
    binder::{SymbolItemKey, SymbolTable},
    helpers,
    uri::{InternUri, UrisCtx},
    LanguageService,
};
use line_index::LineIndex;
use lsp_types::{
    Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, NumberOrString,
};
use rowan::ast::{support, AstNode};
use wat_syntax::{
    ast::{GlobalType, PlainInstr},
    SyntaxNode,
};

const DIAGNOSTIC_CODE: &str = "global-mutation";

pub fn check(
    service: &LanguageService,
    diags: &mut Vec<Diagnostic>,
    uri: InternUri,
    line_index: &LineIndex,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) {
    let Some(instr) = PlainInstr::cast(node.clone()) else {
        return;
    };
    match instr.instr_name() {
        Some(name) if name.text() == "global.set" => {}
        _ => return,
    }
    diags.extend(instr.immediates().filter_map(|immediate| {
        let defs = symbol_table.find_defs(SymbolItemKey::new(immediate.syntax()))?;
        let related_information = defs
            .filter_map(|def| support::child::<GlobalType>(&def.key.to_node(root)))
            .filter(|global_type| global_type.mut_keyword().is_none())
            .map(|global_type| DiagnosticRelatedInformation {
                location: Location {
                    uri: service.lookup_uri(uri),
                    range: helpers::rowan_range_to_lsp_range(
                        line_index,
                        global_type.syntax().text_range(),
                    ),
                },
                message: "immutable global type".into(),
            })
            .collect::<Vec<_>>();
        if related_information.is_empty() {
            None
        } else {
            Some(Diagnostic {
                range: helpers::rowan_range_to_lsp_range(
                    line_index,
                    immediate.syntax().text_range(),
                ),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("wat".into()),
                code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
                message: "mutating an immutable global is not allowed".into(),
                related_information: Some(related_information),
                ..Default::default()
            })
        }
    }));
}
