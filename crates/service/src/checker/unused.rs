use crate::{
    binder::{Symbol, SymbolKind, SymbolTable},
    helpers,
    idx::IdentsCtx,
    LanguageService, LintLevel,
};
use line_index::LineIndex;
use lsp_types::{Diagnostic, DiagnosticSeverity, DiagnosticTag, NumberOrString};
use rowan::{ast::support, Direction};
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "unused";

pub fn check(
    service: &LanguageService,
    diags: &mut Vec<Diagnostic>,
    lint_level: LintLevel,
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
    diags.extend(symbol_table.symbols.iter().filter_map(|symbol| {
        match symbol.kind {
            SymbolKind::Func => {
                if is_prefixed_with_underscore(service, symbol)
                    || is_used(symbol_table, symbol, SymbolKind::Call)
                    || is_exported(root, symbol)
                {
                    None
                } else {
                    Some(report(service, line_index, root, severity, symbol))
                }
            }
            SymbolKind::Param | SymbolKind::Local => {
                if is_prefixed_with_underscore(service, symbol)
                    || is_used(symbol_table, symbol, SymbolKind::LocalRef)
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
                    Some(report(service, line_index, root, severity, symbol))
                }
            }
            SymbolKind::Type => {
                if is_prefixed_with_underscore(service, symbol)
                    || is_used(symbol_table, symbol, SymbolKind::TypeUse)
                    || is_exported(root, symbol)
                {
                    None
                } else {
                    Some(report(service, line_index, root, severity, symbol))
                }
            }
            SymbolKind::GlobalDef => {
                if is_prefixed_with_underscore(service, symbol)
                    || is_used(symbol_table, symbol, SymbolKind::GlobalRef)
                    || is_exported(root, symbol)
                {
                    None
                } else {
                    Some(report(service, line_index, root, severity, symbol))
                }
            }
            SymbolKind::MemoryDef => {
                if is_prefixed_with_underscore(service, symbol)
                    || is_used(symbol_table, symbol, SymbolKind::MemoryRef)
                    || is_exported(root, symbol)
                {
                    None
                } else {
                    Some(report(service, line_index, root, severity, symbol))
                }
            }
            SymbolKind::TableDef => {
                if is_prefixed_with_underscore(service, symbol)
                    || is_used(symbol_table, symbol, SymbolKind::TableRef)
                    || is_exported(root, symbol)
                {
                    None
                } else {
                    Some(report(service, line_index, root, severity, symbol))
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
        .is_some_and(|name| service.lookup_ident(name).starts_with("$_"))
}

fn is_used(symbol_table: &SymbolTable, def_symbol: &Symbol, ref_kind: SymbolKind) -> bool {
    symbol_table.symbols.iter().any(|other| {
        other.kind == ref_kind
            && other.idx.is_defined_by(&def_symbol.idx)
            && other.region == def_symbol.region
    })
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
    let kind = match symbol.kind {
        SymbolKind::Func => "func",
        SymbolKind::Param => "param",
        SymbolKind::Local => "local",
        SymbolKind::Type => "type",
        SymbolKind::GlobalDef => "global",
        SymbolKind::MemoryDef => "memory",
        SymbolKind::TableDef => "table",
        _ => unreachable!(),
    };
    Diagnostic {
        range: helpers::rowan_range_to_lsp_range(line_index, range),
        severity: Some(severity),
        source: Some("wat".into()),
        code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
        message: format!("{kind} `{}` is never used", symbol.idx.render(service)),
        tags: Some(vec![DiagnosticTag::UNNECESSARY]),
        ..Default::default()
    }
}
