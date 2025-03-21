use crate::{
    binder::{Symbol, SymbolKind, SymbolTable},
    helpers,
    idx::IdentsCtx,
    uri::{InternUri, UrisCtx},
    LanguageService,
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, Union2};
use rowan::{ast::support::token, TextRange};
use rustc_hash::FxHashMap;
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "duplicated-names";

pub fn check(
    service: &LanguageService,
    diagnostics: &mut Vec<Diagnostic>,
    uri: InternUri,
    line_index: &LineIndex,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
) {
    diagnostics.extend(
        symbol_table
            .symbols
            .iter()
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
                )
            })
            .fold(FxHashMap::default(), |mut map, symbol| {
                if let Some(name) = symbol.idx.name {
                    let kind = if symbol.kind == SymbolKind::Local {
                        // re-map this symbol kind to make comparison easier
                        SymbolKind::Param
                    } else {
                        symbol.kind
                    };
                    map.entry((name, &symbol.region, kind))
                        .or_insert_with(|| Vec::with_capacity(1))
                        .push(symbol);
                }
                map
            })
            .iter()
            .filter(|(_, symbols)| symbols.len() > 1)
            .flat_map(|((name, _, kind), symbols)| {
                let kind = match kind {
                    SymbolKind::Func => "func",
                    SymbolKind::Param | SymbolKind::Local => "param or local",
                    SymbolKind::Type => "type",
                    SymbolKind::GlobalDef => "global",
                    SymbolKind::MemoryDef => "memory",
                    SymbolKind::TableDef => "table",
                    SymbolKind::FieldDef => "field",
                    _ => unreachable!(),
                };
                let name = service.lookup_ident(*name);
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
                                    uri: service.lookup_uri(uri),
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

    diagnostics.extend(
        symbol_table
            .exports
            .iter()
            .fold(FxHashMap::default(), |mut map, export| {
                map.entry((&export.name, export.module))
                    .or_insert_with(|| Vec::with_capacity(1))
                    .push(export.range);
                map
            })
            .iter()
            .filter(|(_, ranges)| ranges.len() > 1)
            .flat_map(|((name, _), ranges)| {
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
                                    uri: service.lookup_uri(uri),
                                    range: helpers::rowan_range_to_lsp_range(line_index, *range),
                                },
                                message: format!("already exported here as `{name}`"),
                            })
                            .collect(),
                    ),
                    ..Default::default()
                })
            }),
    );
}

fn get_ident_range(symbol: &Symbol, root: &SyntaxNode) -> TextRange {
    token(&symbol.key.to_node(root), SyntaxKind::IDENT)
        .map(|token| token.text_range())
        .unwrap_or_else(|| symbol.key.text_range())
}
