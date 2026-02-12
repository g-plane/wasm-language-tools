use super::Diagnostic;
use crate::{
    LintLevel,
    binder::{Symbol, SymbolKind, SymbolTable},
    document::Document,
    helpers::{BumpCollectionsExt, BumpHashSet},
    imex,
};
use bumpalo::Bump;
use lspt::{DiagnosticSeverity, DiagnosticTag};
use wat_syntax::TextRange;

const DIAGNOSTIC_CODE: &str = "unused";

pub fn check(
    db: &dyn salsa::Database,
    diagnostics: &mut Vec<Diagnostic>,
    document: Document,
    lint_level: LintLevel,
    symbol_table: &SymbolTable,
    bump: &Bump,
) {
    let severity = match lint_level {
        LintLevel::Allow => return,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };
    let imports = imex::get_imports(db, document);
    let exports = imex::get_exports(db, document);
    let used = BumpHashSet::from_iter_in(
        symbol_table.resolved.values().copied().chain(
            exports
                .values()
                .flat_map(|exports| exports.iter().map(|export| export.def_key)),
        ),
        bump,
    );
    diagnostics.extend(symbol_table.symbols.values().filter_map(|symbol| match symbol.kind {
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
            if used.contains(&symbol.key) || is_prefixed_with_underscore(db, symbol) {
                None
            } else {
                symbol_table
                    .def_poi
                    .get(&symbol.key)
                    .map(|range| report(db, *range, severity, symbol))
            }
        }
        SymbolKind::Param => {
            if used.contains(&symbol.key) || is_prefixed_with_underscore(db, symbol) || imports.contains(&symbol.region)
            {
                None
            } else {
                symbol_table
                    .def_poi
                    .get(&symbol.key)
                    .map(|range| report(db, *range, severity, symbol))
            }
        }
        _ => None,
    }));
}

fn is_prefixed_with_underscore(db: &dyn salsa::Database, symbol: &Symbol) -> bool {
    symbol.idx.name.is_some_and(|name| name.ident(db).starts_with("$_"))
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
