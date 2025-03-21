use crate::{
    binder::SymbolTable, helpers, types_analyzer::resolve_br_types, uri::InternUri, LanguageService,
};
use itertools::Itertools;
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use rowan::ast::AstNode;
use wat_syntax::{ast::PlainInstr, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "br-table-branches";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) -> Option<()> {
    let instr = PlainInstr::cast(node.clone())?;
    if instr
        .instr_name()
        .is_none_or(|name| name.text() != "br_table")
    {
        return None;
    }

    let mut immediates = instr.immediates();
    let expected = immediates
        .next()
        .map(|immediate| resolve_br_types(service, uri, symbol_table, &immediate))?;
    diagnostics.extend(immediates.filter_map(|immediate| {
        let received = resolve_br_types(service, uri, symbol_table, &immediate);
        if received != expected {
            Some(Diagnostic {
                range: helpers::rowan_range_to_lsp_range(
                    line_index,
                    immediate.syntax().text_range(),
                ),
                severity: Some(DiagnosticSeverity::Error),
                source: Some("wat".into()),
                code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                message: format!(
                    "type mismatch in `br_table`: expected [{}], found [{}]",
                    expected.iter().map(|ty| ty.render(service)).join(", "),
                    received.iter().map(|ty| ty.render(service)).join(", ")
                ),
                ..Default::default()
            })
        } else {
            None
        }
    }));
    Some(())
}
