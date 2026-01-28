use super::{Diagnostic, RelatedInformation};
use crate::{
    binder::{SymbolKind, SymbolTable},
    document::Document,
    exports,
};
use oxc_allocator::{Allocator, HashMap as OxcHashMap, Vec as OxcVec};

const DIAGNOSTIC_CODE: &str = "duplicated-names";

pub fn check(
    db: &dyn salsa::Database,
    diagnostics: &mut Vec<Diagnostic>,
    document: Document,
    symbol_table: &SymbolTable,
    allocator: &mut Allocator,
) {
    diagnostics.extend(
        symbol_table
            .symbols
            .values()
            .filter(|symbol| {
                matches!(
                    symbol.kind,
                    SymbolKind::Func
                        | SymbolKind::Param
                        | SymbolKind::Local
                        | SymbolKind::Type
                        | SymbolKind::GlobalDef
                        | SymbolKind::MemoryDef
                        | SymbolKind::TableDef
                        | SymbolKind::FieldDef
                        | SymbolKind::TagDef
                )
            })
            .fold(OxcHashMap::new_in(allocator), |mut map, symbol| {
                if let Some(name) = symbol.idx.name {
                    map.entry((name, &symbol.region, symbol.idx_kind))
                        .or_insert_with(|| OxcVec::with_capacity_in(1, allocator))
                        .push(symbol);
                }
                map
            })
            .iter()
            .filter(|(_, symbols)| symbols.len() > 1)
            .flat_map(|((name, _, kind), symbols)| {
                let name = name.ident(db);
                symbols.iter().filter_map(move |symbol| {
                    symbol_table.def_poi.get(&symbol.key).map(|range| Diagnostic {
                        range: *range,
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!("duplicated {kind} name `{name}` in this scope"),
                        related_information: Some(
                            symbols
                                .iter()
                                .filter(|other| *other != symbol)
                                .filter_map(|symbol| {
                                    symbol_table.def_poi.get(&symbol.key).map(|range| RelatedInformation {
                                        range: *range,
                                        message: format!("already defined here as `{name}`"),
                                    })
                                })
                                .collect(),
                        ),
                        ..Default::default()
                    })
                })
            }),
    );
    allocator.reset();

    exports::get_exports(db, document).values().for_each(|exports| {
        diagnostics.extend(
            exports
                .iter()
                .fold(OxcHashMap::new_in(allocator), |mut map, export| {
                    map.entry(&export.name)
                        .or_insert_with(|| OxcVec::with_capacity_in(1, allocator))
                        .push(export.range);
                    map
                })
                .iter()
                .filter(|(_, ranges)| ranges.len() > 1)
                .flat_map(|(name, ranges)| {
                    let name = &name[1..name.len() - 1];
                    ranges.iter().map(move |range| Diagnostic {
                        range: *range,
                        code: DIAGNOSTIC_CODE.into(),
                        message: format!("duplicated export `{name}` in this module"),
                        related_information: Some(
                            ranges
                                .iter()
                                .filter(|other| *other != range)
                                .map(|range| RelatedInformation {
                                    range: *range,
                                    message: format!("already exported here as `{name}`"),
                                })
                                .collect(),
                        ),
                        ..Default::default()
                    })
                }),
        );
    });
    allocator.reset();
}
