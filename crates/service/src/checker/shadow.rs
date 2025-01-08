use super::FilesCtx;
use crate::{
    binder::{SymbolItem, SymbolItemKind, SymbolTable},
    helpers,
    idx::{IdentsCtx, Idx},
    InternUri, LanguageService, LintLevel,
};
use line_index::LineIndex;
use lsp_types::{
    Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, NumberOrString,
};
use rowan::{ast::support::token, TextRange};
use rustc_hash::FxHashMap;
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "shadow";

pub fn check(
    service: &LanguageService,
    diags: &mut Vec<Diagnostic>,
    lint_level: LintLevel,
    uri: InternUri,
    line_index: &LineIndex,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
) {
    let severity = match lint_level {
        LintLevel::Allow => return,
        LintLevel::Hint => DiagnosticSeverity::HINT,
        LintLevel::Warn => DiagnosticSeverity::WARNING,
        LintLevel::Deny => DiagnosticSeverity::ERROR,
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
                                        .text_range()
                                        .contains_range(other.key.text_range())
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
                    code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
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
    token(&symbol.key.to_node(root), SyntaxKind::IDENT)
        .map(|token| token.text_range())
        .unwrap_or_else(|| symbol.key.text_range())
}
