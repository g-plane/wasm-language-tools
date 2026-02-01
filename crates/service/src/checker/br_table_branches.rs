use super::{Diagnostic, FastPlainInstr};
use crate::{binder::SymbolTable, document::Document, types_analyzer::resolve_br_types};
use itertools::Itertools;

const DIAGNOSTIC_CODE: &str = "br-table-branches";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
    instr: &FastPlainInstr,
) -> Option<()> {
    if instr.name != "br_table" {
        return None;
    }
    let expected = instr
        .immediates
        .first()
        .copied()
        .map(|immediate| resolve_br_types(db, document, symbol_table, immediate.into()))?;
    diagnostics.extend(instr.immediates.get(1..)?.iter().copied().filter_map(|immediate| {
        let received = resolve_br_types(db, document, symbol_table, immediate.into());
        if received != expected {
            Some(Diagnostic {
                range: immediate.text_range(),
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
