use super::{Diagnostic, FastPlainInstr};
use crate::{binder::SymbolTable, document::Document, types_analyzer::resolve_br_types};
use itertools::Itertools;
use rowan::ast::AstNode;
use wat_syntax::{SyntaxNode, ast::PlainInstr};

const DIAGNOSTIC_CODE: &str = "br-table-branches";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
    instr: &FastPlainInstr,
) -> Option<()> {
    if instr.name != "br_table" {
        return None;
    }
    let mut immediates = PlainInstr::cast(node.clone())?.immediates();
    let expected = immediates
        .next()
        .map(|immediate| resolve_br_types(db, document, symbol_table, &immediate))?;
    diagnostics.extend(immediates.filter_map(|immediate| {
        let received = resolve_br_types(db, document, symbol_table, &immediate);
        if received != expected {
            Some(Diagnostic {
                range: immediate.syntax().text_range(),
                code: DIAGNOSTIC_CODE.into(),
                message: format!(
                    "type mismatch in `br_table`: expected [{}], found [{}]",
                    expected.iter().map(|ty| ty.render(db)).join(", "),
                    received.iter().map(|ty| ty.render(db)).join(", ")
                ),
                ..Default::default()
            })
        } else {
            None
        }
    }));
    Some(())
}
