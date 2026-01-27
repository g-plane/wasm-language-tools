use super::{Diagnostic, RelatedInformation};
use crate::{
    LintLevel,
    binder::{Symbol, SymbolKind, SymbolTable},
    idx::Idx,
};
use lspt::DiagnosticSeverity;
use oxc_allocator::{Allocator, HashMap as OxcHashMap, Vec as OxcVec};
use rowan::{TextRange, ast::support};
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "shadow";

pub fn check(
    db: &dyn salsa::Database,
    diagnostics: &mut Vec<Diagnostic>,
    lint_level: LintLevel,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
    allocator: &mut Allocator,
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
            .fold(OxcHashMap::new_in(allocator), |mut map, symbol| {
                if let Symbol {
                    kind: SymbolKind::BlockDef,
                    idx: Idx { name: Some(name), .. },
                    ..
                } = symbol
                {
                    let name = *name;
                    map.entry((symbol, name))
                        .or_insert_with(|| OxcVec::new_in(allocator))
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
                                .map(|other| get_ident_range(other, root)),
                        );
                }
                map
            })
            .into_iter()
            .filter(|(_, ranges)| !ranges.is_empty())
            .map(|((symbol, name), ranges)| {
                let name = name.ident(db);
                Diagnostic {
                    range: get_ident_range(symbol, root),
                    severity,
                    code: DIAGNOSTIC_CODE.into(),
                    message: format!("`{name}` is shadowed"),
                    related_information: Some(
                        ranges
                            .into_iter()
                            .map(|range| RelatedInformation {
                                range,
                                message: format!("`{name}` shadowing occurs here"),
                            })
                            .collect(),
                    ),
                    ..Default::default()
                }
            }),
    );

    allocator.reset();
}

fn get_ident_range(symbol: &Symbol, root: &SyntaxNode) -> TextRange {
    support::token(&symbol.key.to_node(root), SyntaxKind::IDENT)
        .map(|token| token.text_range())
        .unwrap_or_else(|| symbol.key.text_range())
}
