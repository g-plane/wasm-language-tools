use super::{Diagnostic, FastPlainInstr};
use crate::{
    binder::SymbolTable,
    document::Document,
    types_analyzer::{join_types, resolve_br_types},
};
use bumpalo::{Bump, collections::Vec as BumpVec};
use wat_syntax::SyntaxKind;

const DIAGNOSTIC_CODE: &str = "br-table-branches";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
    instr: &FastPlainInstr,
    bump: &Bump,
) -> Option<()> {
    if instr.name.text() != "br_table" {
        return None;
    }
    let mut immediates = instr.amber.children_by_kind(SyntaxKind::IMMEDIATE);
    let expected = BumpVec::from_iter_in(
        immediates
            .next()
            .and_then(|immediate| resolve_br_types(db, document, symbol_table, immediate.to_ptr().into()))?,
        bump,
    );
    diagnostics.extend(immediates.filter_map(|immediate| {
        let received = BumpVec::from_iter_in(
            resolve_br_types(db, document, symbol_table, immediate.to_ptr().into())?,
            bump,
        );
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
