use super::FilesCtx;
use crate::{
    binder::{SymbolItem, SymbolItemKind, SymbolTable},
    helpers,
    idx::{IdentsCtx, Idx},
    InternUri, LanguageService, LintLevel,
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
    let severity = match service.configs.get(&uri).map(|config| &config.lint.shadow) {
        Some(LintLevel::Allow) => return,
        Some(LintLevel::Warn) | None => DiagnosticSeverity::WARNING,
        Some(LintLevel::Deny) => DiagnosticSeverity::ERROR,
    };
    diags.extend(
        symbol_table
            .symbols
            .iter()
            .fold(FxHashMap::<_, Vec<_>>::default(), |mut map, symbol| {
                if let SymbolItem {
                    kind: SymbolItemKind::BlockDef,
                    idx: Idx {
                        name: Some(name), ..
                    },
                    ..
                } = symbol
                {
                    let name = *name;
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
            .filter(|(_, ranges)| !ranges.is_empty())
            .map(|((symbol, name), ranges)| {
                let name = service.lookup_ident(name);
                Diagnostic {
                    range: helpers::rowan_range_to_lsp_range(
                        line_index,
                        get_ident_range(symbol, root),
                    ),
                    severity: Some(severity),
                    source: Some("wat".into()),
                    message: format!("`{name}` is shadowed"),
                    related_information: Some(
                        ranges
                            .into_iter()
                            .map(|range| DiagnosticRelatedInformation {
                                location: Location {
                                    uri: service.lookup_uri(uri),
                                    range: helpers::rowan_range_to_lsp_range(line_index, range),
                                },
                                message: format!("`{name}` shadowing occurs here"),
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
