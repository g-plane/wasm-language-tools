use super::FilesCtx;
use crate::{
    binder::{SymbolItem, SymbolItemKind, SymbolTable},
    helpers,
    idx::{IdentsCtx, Idx},
    InternUri, LanguageService,
};
use line_index::LineIndex;
use lsp_types::{Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location};
use rowan::{ast::support::token, TextRange};
use rustc_hash::FxHashMap;
use wat_syntax::{SyntaxKind, SyntaxNode};

pub fn check(
    service: &LanguageService,
    diags: &mut Vec<Diagnostic>,
    uri: InternUri,
    line_index: &LineIndex,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
) {
    diags.extend(
        symbol_table
            .symbols
            .iter()
            .filter(|symbol| {
                matches!(
                    symbol.kind,
                    SymbolItemKind::Func
                        | SymbolItemKind::Param
                        | SymbolItemKind::Local
                        | SymbolItemKind::Type
                        | SymbolItemKind::GlobalDef
                        | SymbolItemKind::MemoryDef
                        | SymbolItemKind::BlockDef
                )
            })
            .fold(FxHashMap::default(), |mut map, symbol| {
                if let Some(name) = symbol.idx.name {
                    map.entry((name, &symbol.region))
                        .or_insert_with(|| Vec::with_capacity(1))
                        .push(symbol);
                }
                map
            })
            .iter()
            .filter(|(_, symbols)| symbols.len() > 1)
            .flat_map(|((name, _), symbols)| {
                let name = service.lookup_ident(*name);
                symbols.iter().map(move |symbol| Diagnostic {
                    range: helpers::rowan_range_to_lsp_range(
                        line_index,
                        get_ident_range(symbol, root),
                    ),
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("wat".into()),
                    message: format!("duplicated name `{name}` in this scope"),
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

    diags.extend(
        symbol_table
            .symbols
            .iter()
            .fold(FxHashMap::default(), |mut map, symbol| {
                if let SymbolItem {
                    kind: SymbolItemKind::BlockDef,
                    idx: Idx {
                        name: Some(name), ..
                    },
                    ..
                } = symbol
                {
                    let name = *name;
                    let mut current = symbol;
                    while let Some(
                        parent @ SymbolItem {
                            kind: SymbolItemKind::BlockDef,
                            idx,
                            ..
                        },
                    ) = symbol_table
                        .symbols
                        .iter()
                        .find(|sym| sym.key == current.region)
                    {
                        if idx.name.is_some_and(|other| other == name) {
                            map.entry((symbol, name))
                                .or_insert_with(|| Vec::with_capacity(1))
                                .push(get_ident_range(parent, root));
                        }
                        current = parent;
                    }
                    map.entry((symbol, name)).or_default().extend(
                        symbol_table
                            .symbols
                            .iter()
                            .filter(|other| {
                                *other != symbol
                                    && other.kind == SymbolItemKind::BlockDef
                                    && other.idx.name.is_some_and(|other| other == name)
                                    && symbol
                                        .key
                                        .ptr
                                        .text_range()
                                        .contains_range(other.key.ptr.text_range())
                            })
                            .map(|other| get_ident_range(other, root)),
                    );
                }
                map
            })
            .into_iter()
            .filter(|(_, ranges)| ranges.len() > 1)
            .map(|((symbol, name), mut ranges)| {
                ranges.sort_by_key(|range| range.start());
                let name = service.lookup_ident(name);
                Diagnostic {
                    range: helpers::rowan_range_to_lsp_range(
                        line_index,
                        get_ident_range(symbol, root),
                    ),
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("wat".into()),
                    message: format!("duplicated name `{name}` in this scope"),
                    related_information: Some(
                        ranges
                            .into_iter()
                            .map(|range| DiagnosticRelatedInformation {
                                location: Location {
                                    uri: service.lookup_uri(uri),
                                    range: helpers::rowan_range_to_lsp_range(line_index, range),
                                },
                                message: format!("already defined here as `{name}`"),
                            })
                            .collect(),
                    ),
                    ..Default::default()
                }
            }),
    );
}

fn get_ident_range(symbol: &SymbolItem, root: &SyntaxNode) -> TextRange {
    token(&symbol.key.ptr.to_node(root), SyntaxKind::IDENT)
        .map(|token| token.text_range())
        .unwrap_or_else(|| symbol.key.ptr.text_range())
}
