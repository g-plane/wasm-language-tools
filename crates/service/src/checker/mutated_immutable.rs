use crate::{
    binder::{SymbolKey, SymbolTable},
    helpers,
    uri::{InternUri, UrisCtx},
    LanguageService,
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, Union2};
use rowan::ast::{support, AstNode};
use wat_syntax::{
    ast::{GlobalType, PlainInstr},
    SyntaxNode,
};

const DIAGNOSTIC_CODE: &str = "mutated-immutable";

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
        let def = symbol_table.find_def(SymbolKey::new(immediate.syntax()))?;
        support::child::<GlobalType>(&def.key.to_node(root))
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
            .map(|related_information| Diagnostic {
                range: helpers::rowan_range_to_lsp_range(
                    line_index,
                    immediate.syntax().text_range(),
                ),
                severity: Some(DiagnosticSeverity::Error),
                source: Some("wat".into()),
                code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                message: "mutating an immutable global is not allowed".into(),
                related_information: Some(vec![related_information]),
                ..Default::default()
            })
    }));
}
