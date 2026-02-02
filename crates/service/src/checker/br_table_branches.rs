use super::{Diagnostic, FastPlainInstr};
use crate::{binder::SymbolTable, document::Document, types_analyzer::resolve_br_types};
use itertools::Itertools;
use oxc_allocator::{Allocator, Vec as OxcVec};

const DIAGNOSTIC_CODE: &str = "br-table-branches";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
    instr: &FastPlainInstr,
    allocator: &Allocator,
) -> Option<()> {
    if instr.name != "br_table" {
        return None;
    }
    let expected = OxcVec::from_iter_in(
        instr
            .immediates
            .first()
            .copied()
            .and_then(|immediate| resolve_br_types(db, document, symbol_table, immediate.into()))?,
        allocator,
    );
    diagnostics.extend(instr.immediates.get(1..)?.iter().copied().filter_map(|immediate| {
        let received = OxcVec::from_iter_in(
            resolve_br_types(db, document, symbol_table, immediate.into())?,
            allocator,
        );
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
