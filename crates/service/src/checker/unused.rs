use crate::{
    LanguageService, LintLevel,
    binder::{Symbol, SymbolKind, SymbolTable},
    helpers,
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, DiagnosticTag, Union2};
use rowan::{Direction, TextRange, ast::support};
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "unused";

pub fn check(
    service: &LanguageService,
    diagnostics: &mut Vec<Diagnostic>,
    lint_level: LintLevel,
    line_index: &LineIndex,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
) {
    let severity = match lint_level {
        LintLevel::Allow => return,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };
    diagnostics.extend(symbol_table.symbols.values().filter_map(|symbol| {
        match symbol.kind {
            SymbolKind::Func => {
                if is_prefixed_with_underscore(service, symbol)
                    || is_used(symbol_table, symbol)
                    || is_exported(root, symbol)
                {
                    None
                } else {
                    Some(report(service, line_index, root, severity, symbol))
                }
            }
            SymbolKind::Param | SymbolKind::Local => {
                if is_prefixed_with_underscore(service, symbol)
                    || is_used(symbol_table, symbol)
                    || symbol
                        .key
                        .to_node(root)
                        .parent()
                        .and_then(|parent| {
                            if parent.kind() == SyntaxKind::TYPE_USE {
                                Some(parent)
                            } else {
                                parent.parent()
                            }
                        })
                        .map(|node| {
                            node.siblings(Direction::Prev)
                                .any(|sibling| sibling.kind() == SyntaxKind::IMPORT)
                        })
                        .unwrap_or_default()
                {
                    None
                } else {
                    let node = symbol.key.to_node(root);
                    let range = support::token(&node, SyntaxKind::IDENT)
                        .map(|token| token.text_range())
                        .unwrap_or_else(|| node.text_range());
                    Some(report_with_range(
                        service, line_index, range, severity, symbol,
                    ))
                }
            }
            SymbolKind::Type => {
                if is_prefixed_with_underscore(service, symbol)
                    || is_used(symbol_table, symbol)
                    || is_exported(root, symbol)
                {
                    None
                } else {
                    Some(report(service, line_index, root, severity, symbol))
                }
            }
            SymbolKind::GlobalDef => {
                if is_prefixed_with_underscore(service, symbol)
                    || is_used(symbol_table, symbol)
                    || is_exported(root, symbol)
                {
                    None
                } else {
                    Some(report(service, line_index, root, severity, symbol))
                }
            }
            SymbolKind::MemoryDef => {
                if is_prefixed_with_underscore(service, symbol)
                    || is_used(symbol_table, symbol)
                    || is_exported(root, symbol)
                {
                    None
                } else {
                    Some(report(service, line_index, root, severity, symbol))
                }
            }
            SymbolKind::TableDef => {
                if is_prefixed_with_underscore(service, symbol)
                    || is_used(symbol_table, symbol)
                    || is_exported(root, symbol)
                {
                    None
                } else {
                    Some(report(service, line_index, root, severity, symbol))
                }
            }
            SymbolKind::FieldDef => {
                if is_prefixed_with_underscore(service, symbol) || is_used(symbol_table, symbol) {
                    None
                } else {
                    let node = symbol.key.to_node(root);
                    let range = support::token(&node, SyntaxKind::IDENT)
                        .map(|token| token.text_range())
                        .unwrap_or_else(|| node.text_range());
                    Some(report_with_range(
                        service, line_index, range, severity, symbol,
                    ))
                }
            }
            _ => None,
        }
    }));
}

fn is_prefixed_with_underscore(service: &LanguageService, symbol: &Symbol) -> bool {
    symbol
        .idx
        .name
        .is_some_and(|name| name.ident(service).starts_with("$_"))
}

fn is_used(symbol_table: &SymbolTable, symbol: &Symbol) -> bool {
    symbol_table.resolved.values().any(|key| key == &symbol.key)
}

fn is_exported(root: &SyntaxNode, def_symbol: &Symbol) -> bool {
    let node = def_symbol.key.to_node(root);
    node.children()
        .any(|child| child.kind() == SyntaxKind::EXPORT)
}

fn report(
    service: &LanguageService,
    line_index: &LineIndex,
    root: &SyntaxNode,
    severity: DiagnosticSeverity,
    symbol: &Symbol,
) -> Diagnostic {
    let node = symbol.key.to_node(root);
    let range = support::token(&node, SyntaxKind::IDENT)
        .or_else(|| support::token(&node, SyntaxKind::KEYWORD))
        .map(|token| token.text_range())
        .unwrap_or_else(|| node.text_range());
    report_with_range(service, line_index, range, severity, symbol)
}

fn report_with_range(
    service: &LanguageService,
    line_index: &LineIndex,
    range: TextRange,
    severity: DiagnosticSeverity,
    symbol: &Symbol,
) -> Diagnostic {
    let kind = match symbol.kind {
        SymbolKind::Func => "func",
        SymbolKind::Param => "param",
        SymbolKind::Local => "local",
        SymbolKind::Type => "type",
        SymbolKind::GlobalDef => "global",
        SymbolKind::MemoryDef => "memory",
        SymbolKind::TableDef => "table",
        SymbolKind::FieldDef => "field",
        _ => unreachable!(),
    };
    Diagnostic {
        range: helpers::rowan_range_to_lsp_range(line_index, range),
        severity: Some(severity),
        source: Some("wat".into()),
        code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
        message: format!("{kind} `{}` is never used", symbol.idx.render(service)),
        tags: Some(vec![DiagnosticTag::Unnecessary]),
        ..Default::default()
    }
}
