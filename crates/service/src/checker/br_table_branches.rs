use super::{Diagnostic, FastPlainInstr};
use crate::{
    binder::SymbolTable,
    document::Document,
    types_analyzer::{join_types, resolve_br_types},
};
use bumpalo::{Bump, collections::Vec as BumpVec};

const DIAGNOSTIC_CODE: &str = "br-table-branches";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
    instr: &FastPlainInstr,
    bump: &Bump,
) -> Option<()> {
    if instr.name != "br_table" {
        return None;
    }
    let expected = BumpVec::from_iter_in(
        instr
            .immediates
            .first()
            .copied()
            .and_then(|immediate| resolve_br_types(db, document, symbol_table, immediate.into()))?,
        bump,
    );
    diagnostics.extend(instr.immediates.get(1..)?.iter().copied().filter_map(|immediate| {
        let received = BumpVec::from_iter_in(resolve_br_types(db, document, symbol_table, immediate.into())?, bump);
        if received != expected {
            Some(Diagnostic {
                range: immediate.text_range(),
                code: DIAGNOSTIC_CODE.into(),
                message: format!(
                    "type mismatch in `br_table`: expected {}, found {}",
                    join_types(db, expected.iter(), "", bump),
                    join_types(db, received.iter(), "", bump),
                ),
                ..Default::default()
            })
        } else {
            None
        }
    }));
    Some(())
}
