use crate::{
    binder::SymbolTable, helpers, types_analyzer::resolve_br_types, uri::InternUri, LanguageService,
};
use itertools::Itertools;
use line_index::LineIndex;
use lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString};
use rowan::ast::AstNode;
use wat_syntax::{ast::PlainInstr, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "br-table-branches";

pub fn check(
    diags: &mut Vec<Diagnostic>,
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) {
    let Some(instr) = PlainInstr::cast(node.clone()) else {
        return;
    };
    if instr
        .instr_name()
        .is_none_or(|name| name.text() != "br_table")
    {
        return;
    }

    let mut immediates = instr.immediates();
    let Some(expected) = immediates
        .next()
        .map(|immediate| resolve_br_types(service, uri, symbol_table, &immediate))
    else {
        return;
    };
    diags.extend(immediates.filter_map(|immediate| {
        let received = resolve_br_types(service, uri, symbol_table, &immediate);
        if received != expected {
            Some(Diagnostic {
                range: helpers::rowan_range_to_lsp_range(
                    line_index,
                    immediate.syntax().text_range(),
                ),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("wat".into()),
                code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
                message: format!(
                    "type mismatch in `br_table`: expected [{}], found [{}]",
                    expected.iter().join(", "),
                    received.iter().join(", ")
                ),
                ..Default::default()
            })
        } else {
            None
        }
    }));
}
