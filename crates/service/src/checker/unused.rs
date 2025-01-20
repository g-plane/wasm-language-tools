use crate::{
    binder::{SymbolItem, SymbolItemKind, SymbolTable},
    helpers,
    idx::IdentsCtx,
    LanguageService, LintLevel,
};
use line_index::LineIndex;
use lsp_types::{Diagnostic, DiagnosticSeverity, DiagnosticTag, NumberOrString};
use rowan::ast::support;
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
    diags.extend(
        symbol_table
            .symbols
            .iter()
            .filter_map(|symbol| match symbol.kind {
                SymbolItemKind::Func => {
                    if is_prefixed_with_underscore(service, symbol)
                        || is_used(symbol_table, symbol, SymbolItemKind::Call)
                        || is_exported(root, symbol)
                    {
                        None
                    } else {
                        Some(report(service, line_index, root, severity, symbol))
                    }
                }
                SymbolItemKind::Param | SymbolItemKind::Local => {
                    if is_prefixed_with_underscore(service, symbol)
                        || is_used(symbol_table, symbol, SymbolItemKind::LocalRef)
                    {
                        None
                    } else {
                        Some(report(service, line_index, root, severity, symbol))
                    }
                }
                SymbolItemKind::Type => {
                    if is_prefixed_with_underscore(service, symbol)
                        || is_used(symbol_table, symbol, SymbolItemKind::TypeUse)
                        || is_exported(root, symbol)
                    {
                        None
                    } else {
                        Some(report(service, line_index, root, severity, symbol))
                    }
                }
                SymbolItemKind::GlobalDef => {
                    if is_prefixed_with_underscore(service, symbol)
                        || is_used(symbol_table, symbol, SymbolItemKind::GlobalRef)
                        || is_exported(root, symbol)
                    {
                        None
                    } else {
                        Some(report(service, line_index, root, severity, symbol))
                    }
                }
                SymbolItemKind::MemoryDef => {
                    if is_prefixed_with_underscore(service, symbol)
                        || is_used(symbol_table, symbol, SymbolItemKind::MemoryRef)
                        || is_exported(root, symbol)
                    {
                        None
                    } else {
                        Some(report(service, line_index, root, severity, symbol))
                    }
                }
                SymbolItemKind::TableDef => {
                    if is_prefixed_with_underscore(service, symbol)
                        || is_used(symbol_table, symbol, SymbolItemKind::TableRef)
                        || is_exported(root, symbol)
                    {
                        None
                    } else {
                        Some(report(service, line_index, root, severity, symbol))
                    }
                }
                _ => None,
            }),
    );
}

fn is_prefixed_with_underscore(service: &LanguageService, symbol: &SymbolItem) -> bool {
    symbol
        .idx
        .name
        .is_some_and(|name| service.lookup_ident(name).starts_with("$_"))
}

fn is_used(symbol_table: &SymbolTable, def_symbol: &SymbolItem, ref_kind: SymbolItemKind) -> bool {
    symbol_table.symbols.iter().any(|other| {
        other.kind == ref_kind
            && other.idx.is_defined_by(&def_symbol.idx)
            && other.region == def_symbol.region
    })
}

fn is_exported(root: &SyntaxNode, def_symbol: &SymbolItem) -> bool {
    let node = def_symbol.key.to_node(root);
    node.children()
        .any(|child| child.kind() == SyntaxKind::EXPORT)
}

fn report(
    service: &LanguageService,
    line_index: &LineIndex,
    root: &SyntaxNode,
    severity: DiagnosticSeverity,
    symbol: &SymbolItem,
) -> Diagnostic {
    let node = symbol.key.to_node(root);
    let range = support::token(&node, SyntaxKind::IDENT)
        .or_else(|| support::token(&node, SyntaxKind::KEYWORD))
        .map(|token| token.text_range())
        .unwrap_or_else(|| node.text_range());
    Diagnostic {
        range: helpers::rowan_range_to_lsp_range(line_index, range),
        severity: Some(severity),
        source: Some("wat".into()),
        code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
        message: format!("`{}` is never used", symbol.idx.render(service)),
        tags: Some(vec![DiagnosticTag::UNNECESSARY]),
        ..Default::default()
    }
}
