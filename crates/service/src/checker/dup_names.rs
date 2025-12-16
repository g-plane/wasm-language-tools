use crate::{
    LanguageService,
    binder::{Symbol, SymbolKind, SymbolTable},
    document::Document,
    exports, helpers,
    uri::InternUri,
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, Union2};
use rowan::{TextRange, ast::support};
use rustc_hash::FxHashMap;
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "duplicated-names";

pub fn check(
    service: &LanguageService,
    diagnostics: &mut Vec<Diagnostic>,
    uri: InternUri,
    document: Document,
    line_index: &LineIndex,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
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
            .fold(FxHashMap::default(), |mut map, symbol| {
                if let Some(name) = symbol.idx.name {
                    map.entry((name, &symbol.region, symbol.idx_kind))
                        .or_insert_with(|| Vec::with_capacity(1))
                        .push(symbol);
                }
                map
            })
            .iter()
            .filter(|(_, symbols)| symbols.len() > 1)
            .flat_map(|((name, _, kind), symbols)| {
                let name = name.ident(service);
                symbols.iter().map(move |symbol| Diagnostic {
                    range: helpers::rowan_range_to_lsp_range(
                        line_index,
                        get_ident_range(symbol, root),
                    ),
                    severity: Some(DiagnosticSeverity::Error),
                    source: Some("wat".into()),
                    code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                    message: format!("duplicated {kind} name `{name}` in this scope"),
                    related_information: Some(
                        symbols
                            .iter()
                            .filter(|other| *other != symbol)
                            .map(|symbol| DiagnosticRelatedInformation {
                                location: Location {
                                    uri: uri.raw(service),
                                    range: helpers::rowan_range_to_lsp_range(
                                        line_index,
                                        get_ident_range(symbol, root),
                                    ),
                                },
                                message: format!("already defined here as `{name}`"),
                            })
                            .collect(),
                    ),
                    ..Default::default()
                })
            }),
    );

    exports::get_exports(service, document)
        .values()
        .for_each(|exports| {
            diagnostics.extend(
                exports
                    .iter()
                    .fold(FxHashMap::default(), |mut map, export| {
                        map.entry(&export.name)
                            .or_insert_with(|| Vec::with_capacity(1))
                            .push(export.range);
                        map
                    })
                    .iter()
                    .filter(|(_, ranges)| ranges.len() > 1)
                    .flat_map(|(name, ranges)| {
                        let name = &name[1..name.len() - 1];
                        ranges.iter().map(move |range| Diagnostic {
                            range: helpers::rowan_range_to_lsp_range(line_index, *range),
                            severity: Some(DiagnosticSeverity::Error),
                            source: Some("wat".into()),
                            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                            message: format!("duplicated export `{name}` in this module"),
                            related_information: Some(
                                ranges
                                    .iter()
                                    .filter(|other| *other != range)
                                    .map(|range| DiagnosticRelatedInformation {
                                        location: Location {
                                            uri: uri.raw(service),
                                            range: helpers::rowan_range_to_lsp_range(
                                                line_index, *range,
                                            ),
                                        },
                                        message: format!("already exported here as `{name}`"),
                                    })
                                    .collect(),
                            ),
                            ..Default::default()
                        })
                    }),
            );
        });
}

fn get_ident_range(symbol: &Symbol, root: &SyntaxNode) -> TextRange {
    support::token(&symbol.key.to_node(root), SyntaxKind::IDENT)
        .map(|token| token.text_range())
        .unwrap_or_else(|| symbol.key.text_range())
}
