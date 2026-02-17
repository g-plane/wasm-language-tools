use super::Diagnostic;
use crate::{binder::SymbolTable, document::Document, types_analyzer};
use wat_syntax::SyntaxKind;

const DIAGNOSTIC_CODE: &str = "subtyping";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
) {
    let def_types = types_analyzer::get_def_types(db, document);
    diagnostics.extend(
        def_types
            .iter()
            .filter_map(|(key, sub_type)| {
                let inherits = sub_type.inherits.as_ref()?;
                let super_type = def_types.get(&inherits.symbol)?;
                let def_symbol = symbol_table.symbols.get(key)?;
                let module_id = symbol_table.symbols.get(&def_symbol.region)?.idx.num?;

                if super_type
                    .idx
                    .num
                    .zip(sub_type.idx.num)
                    .is_some_and(|(sup, sub)| sup >= sub)
                {
                    Some((
                        def_symbol,
                        format!(
                            "typeidx of super type `{}` must be smaller than type `{}`",
                            super_type.idx.render(db),
                            sub_type.idx.render(db),
                        ),
                    ))
                } else if super_type.is_final {
                    Some((def_symbol, format!("type `{}` is final", super_type.idx.render(db))))
                } else if !sub_type.comp.matches(&super_type.comp, db, document, module_id) {
                    Some((
                        def_symbol,
                        format!(
                            "type of `{}` doesn't match its super type `{}`",
                            sub_type.idx.render(db),
                            super_type.idx.render(db),
                        ),
                    ))
                } else {
                    None
                }
            })
            .filter_map(|(def_symbol, message)| {
                let range = def_symbol
                    .amber()
                    .children_by_kind(SyntaxKind::SUB_TYPE)
                    .next()?
                    .children_by_kind(SyntaxKind::INDEX)
                    .next()?
                    .text_range();
                Some(Diagnostic {
                    range,
                    code: DIAGNOSTIC_CODE.into(),
                    message,
                    ..Default::default()
                })
            }),
    );
}
