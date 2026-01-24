use super::Diagnostic;
use crate::{
    LintLevel,
    binder::{Symbol, SymbolKind, SymbolTable},
};
use lspt::{DiagnosticSeverity, DiagnosticTag};
use rowan::{Direction, TextRange, ast::support};
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "unused";

pub fn check(
    db: &dyn salsa::Database,
    diagnostics: &mut Vec<Diagnostic>,
    lint_level: LintLevel,
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
            SymbolKind::Func
            | SymbolKind::Type
            | SymbolKind::GlobalDef
            | SymbolKind::MemoryDef
            | SymbolKind::TableDef
            | SymbolKind::TagDef => {
                if is_prefixed_with_underscore(db, symbol) || is_used(symbol_table, symbol) || is_exported(root, symbol)
                {
                    None
                } else {
                    Some(report(db, root, severity, symbol))
                }
            }
            SymbolKind::Param | SymbolKind::Local => {
                if is_prefixed_with_underscore(db, symbol)
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
                    Some(report_with_range(db, range, severity, symbol))
                }
            }
            SymbolKind::FieldDef => {
                if is_prefixed_with_underscore(db, symbol) || is_used(symbol_table, symbol) {
                    None
                } else {
                    let node = symbol.key.to_node(root);
                    let range = support::token(&node, SyntaxKind::IDENT)
                        .map(|token| token.text_range())
                        .unwrap_or_else(|| node.text_range());
                    Some(report_with_range(db, range, severity, symbol))
                }
            }
            _ => None,
        }
    }));
}

fn is_prefixed_with_underscore(db: &dyn salsa::Database, symbol: &Symbol) -> bool {
    symbol.idx.name.is_some_and(|name| name.ident(db).starts_with("$_"))
}

fn is_used(symbol_table: &SymbolTable, symbol: &Symbol) -> bool {
    symbol_table.resolved.values().any(|key| key == &symbol.key)
}

fn is_exported(root: &SyntaxNode, def_symbol: &Symbol) -> bool {
    let node = def_symbol.key.to_node(root);
    node.children().any(|child| child.kind() == SyntaxKind::EXPORT)
}

fn report(db: &dyn salsa::Database, root: &SyntaxNode, severity: DiagnosticSeverity, symbol: &Symbol) -> Diagnostic {
    let node = symbol.key.to_node(root);
    let range = support::token(&node, SyntaxKind::IDENT)
        .or_else(|| support::token(&node, SyntaxKind::KEYWORD))
        .map(|token| token.text_range())
        .unwrap_or_else(|| node.text_range());
    report_with_range(db, range, severity, symbol)
}

fn report_with_range(
    db: &dyn salsa::Database,
    range: TextRange,
    severity: DiagnosticSeverity,
    symbol: &Symbol,
) -> Diagnostic {
    Diagnostic {
        range,
        severity,
        code: DIAGNOSTIC_CODE.into(),
        message: format!("{} `{}` is never used", symbol.kind, symbol.idx.render(db)),
        tags: Some(vec![DiagnosticTag::Unnecessary]),
        ..Default::default()
    }
}
