use crate::{
    binder::{SymbolItem, SymbolItemKind, SymbolTable},
    helpers,
    idx::IdentsCtx,
    InternUri, LanguageService, LintLevel,
};
use line_index::LineIndex;
use lsp_types::{Diagnostic, DiagnosticSeverity};
use rowan::ast::support;
use wat_syntax::{SyntaxKind, SyntaxNode};

pub fn check(
    service: &LanguageService,
    diags: &mut Vec<Diagnostic>,
    uri: InternUri,
    line_index: &LineIndex,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
) {
    let severity = match service.get_config(uri).lint.unused {
        LintLevel::Allow => return,
        LintLevel::Warn => DiagnosticSeverity::WARNING,
        LintLevel::Deny => DiagnosticSeverity::ERROR,
    };
    diags.extend(
        symbol_table
            .symbols
            .iter()
            .filter_map(|symbol| match symbol.kind {
                SymbolItemKind::Func => {
                    if !is_used(symbol_table, symbol, SymbolItemKind::Call)
                        && !is_exported(root, symbol)
                    {
                        Some(report(service, line_index, root, severity, symbol))
                    } else {
                        None
                    }
                }
                SymbolItemKind::Param | SymbolItemKind::Local => {
                    if !is_used(symbol_table, symbol, SymbolItemKind::LocalRef) {
                        Some(report(service, line_index, root, severity, symbol))
                    } else {
                        None
                    }
                }
                SymbolItemKind::Type => {
                    if !is_used(symbol_table, symbol, SymbolItemKind::TypeUse)
                        && !is_exported(root, symbol)
                    {
                        Some(report(service, line_index, root, severity, symbol))
                    } else {
                        None
                    }
                }
                SymbolItemKind::GlobalDef => {
                    if !is_used(symbol_table, symbol, SymbolItemKind::GlobalRef)
                        && !is_exported(root, symbol)
                    {
                        Some(report(service, line_index, root, severity, symbol))
                    } else {
                        None
                    }
                }
                SymbolItemKind::MemoryDef => {
                    if !is_used(symbol_table, symbol, SymbolItemKind::MemoryRef)
                        && !is_exported(root, symbol)
                    {
                        Some(report(service, line_index, root, severity, symbol))
                    } else {
                        None
                    }
                }
                SymbolItemKind::TableDef => {
                    if !is_used(symbol_table, symbol, SymbolItemKind::TableRef)
                        && !is_exported(root, symbol)
                    {
                        Some(report(service, line_index, root, severity, symbol))
                    } else {
                        None
                    }
                }
                _ => None,
            }),
    );
}

fn is_used(symbol_table: &SymbolTable, def_symbol: &SymbolItem, ref_kind: SymbolItemKind) -> bool {
    symbol_table.symbols.iter().any(|other| {
        other.kind == ref_kind
            && other.idx.is_defined_by(&def_symbol.idx)
            && other.region == def_symbol.region
    })
}

fn is_exported(root: &SyntaxNode, def_symbol: &SymbolItem) -> bool {
    let node = def_symbol.key.ptr.to_node(root);
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
    let node = symbol.key.ptr.to_node(root);
    let range = support::token(&node, SyntaxKind::IDENT)
        .or_else(|| support::token(&node, SyntaxKind::KEYWORD))
        .map(|token| token.text_range())
        .unwrap_or_else(|| node.text_range());
    Diagnostic {
        range: helpers::rowan_range_to_lsp_range(line_index, range),
        severity: Some(severity),
        source: Some("wat".into()),
        message: format!(
            "`{}` is never used",
            symbol
                .idx
                .name
                .map(|name| service.lookup_ident(name))
                .or_else(|| symbol.idx.num.map(|num| num.to_string()))
                .unwrap_or_default()
        ),
        ..Default::default()
    }
}
