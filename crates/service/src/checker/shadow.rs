use super::{Diagnostic, RelatedInformation};
use crate::{
    LintLevel,
    binder::{Symbol, SymbolKind, SymbolTable},
    helpers::{BumpCollectionsExt, BumpHashMap},
    idx::Idx,
};
use bumpalo::{Bump, collections::Vec as BumpVec};
use lspt::DiagnosticSeverity;

const DIAGNOSTIC_CODE: &str = "shadow";

pub fn check(
    db: &dyn salsa::Database,
    diagnostics: &mut Vec<Diagnostic>,
    lint_level: LintLevel,
    symbol_table: &SymbolTable,
    bump: &mut Bump,
) {
    let severity = match lint_level {
        LintLevel::Allow => return,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };
    diagnostics.extend(
        symbol_table
            .symbols
            .values()
            .fold(BumpHashMap::new_in(bump), |mut map, symbol| {
                if let Symbol {
                    kind: SymbolKind::BlockDef,
                    idx: Idx { name: Some(name), .. },
                    ..
                } = symbol
                {
                    let name = *name;
                    map.entry((symbol, name))
                        .or_insert_with(|| BumpVec::new_in(bump))
                        .extend(
                            symbol_table
                                .symbols
                                .values()
                                .filter(|other| {
                                    *other != symbol
                                        && other.kind == SymbolKind::BlockDef
                                        && other.idx.name.is_some_and(|other| other == name)
                                        && symbol.key.text_range().contains_range(other.key.text_range())
                                })
                                .filter_map(|other| symbol_table.def_poi.get(&other.key).copied()),
                        );
                }
                map
            })
            .iter()
            .filter(|(_, ranges)| !ranges.is_empty())
            .filter_map(|((symbol, name), ranges)| {
                let name = name.ident(db);
                let range = symbol_table.def_poi.get(&symbol.key)?;
                Some(Diagnostic {
                    range: *range,
                    severity,
                    code: DIAGNOSTIC_CODE.into(),
                    message: format!("`{name}` is shadowed"),
                    related_information: Some(
                        ranges
                            .into_iter()
                            .map(|range| RelatedInformation {
                                range: *range,
                                message: format!("`{name}` shadowing occurs here"),
                            })
                            .collect(),
                    ),
                    ..Default::default()
                })
            }),
    );

    bump.reset();
}
