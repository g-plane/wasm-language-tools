use super::Diagnostic;
use crate::{
    LintLevel,
    binder::{Symbol, SymbolKey, SymbolKind, SymbolTable},
    helpers::{self, BumpCollectionsExt, BumpHashSet},
};
use bumpalo::Bump;
use lspt::{DiagnosticSeverity, DiagnosticTag};
use wat_syntax::{SyntaxKind, TextRange};

const DIAGNOSTIC_CODE: &str = "unused";

pub fn check(
    db: &dyn salsa::Database,
    diagnostics: &mut Vec<Diagnostic>,
    lint_level: LintLevel,
    symbol_table: &SymbolTable,
    imports: &[SymbolKey],
    bump: &Bump,
) {
    let severity = match lint_level {
        LintLevel::Allow => return,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };
    let used = BumpHashSet::from_iter_in(symbol_table.resolved.values().copied(), bump);
    diagnostics.extend(symbol_table.symbols.iter().filter_map(|symbol| match symbol.kind {
        SymbolKind::Func
        | SymbolKind::Local
        | SymbolKind::Type
        | SymbolKind::GlobalDef
        | SymbolKind::MemoryDef
        | SymbolKind::TableDef
        | SymbolKind::FieldDef
        | SymbolKind::TagDef
        | SymbolKind::DataDef
        | SymbolKind::ElemDef => {
            if used.contains(&symbol.key) || has_export(symbol) || is_prefixed_with_underscore(db, symbol) {
                None
            } else {
                let range = helpers::syntax::infer_def_poi(symbol.amber());
                Some(report(db, range, severity, symbol))
            }
        }
        SymbolKind::Param => {
            if used.contains(&symbol.key)
                || is_prefixed_with_underscore(db, symbol)
                || imports.contains(&symbol.region)
                || symbol.region.kind() == SyntaxKind::TYPE_DEF
            {
                None
            } else {
                let range = helpers::syntax::infer_def_poi(symbol.amber());
                Some(report(db, range, severity, symbol))
            }
        }
        _ => None,
    }));
}

fn is_prefixed_with_underscore(db: &dyn salsa::Database, symbol: &Symbol) -> bool {
    symbol.idx.name.is_some_and(|name| name.ident(db).starts_with("$_"))
}

fn has_export(symbol: &Symbol) -> bool {
    symbol.amber().children_by_kind(SyntaxKind::EXPORT).next().is_some()
}

fn report(db: &dyn salsa::Database, range: TextRange, severity: DiagnosticSeverity, symbol: &Symbol) -> Diagnostic {
    Diagnostic {
        range,
        severity,
        code: DIAGNOSTIC_CODE.into(),
        message: format!("{} `{}` is never used", symbol.kind, symbol.idx.render(db)),
        tags: Some(vec![DiagnosticTag::Unnecessary]),
        ..Default::default()
    }
}
