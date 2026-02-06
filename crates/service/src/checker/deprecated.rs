use super::Diagnostic;
use crate::{
    binder::{SymbolKind, SymbolTable},
    config::LintLevel,
    deprecation,
    document::Document,
};
use lspt::{DiagnosticSeverity, DiagnosticTag};

const DIAGNOSTIC_CODE: &str = "deprecated";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    document: Document,
    lint_level: LintLevel,
    symbol_table: &SymbolTable,
) {
    let severity = match lint_level {
        LintLevel::Allow => return,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };
    let deprecation = deprecation::get_deprecation(db, document);
    let deprecated_usage = symbol_table
        .symbols
        .values()
        .filter(|symbol| {
            matches!(
                symbol.kind,
                SymbolKind::Call
                    | SymbolKind::TypeUse
                    | SymbolKind::GlobalRef
                    | SymbolKind::MemoryRef
                    | SymbolKind::TableRef
                    | SymbolKind::TagRef
                    | SymbolKind::DataRef
            )
        })
        .filter_map(|symbol| {
            symbol_table
                .resolved
                .get(&symbol.key)
                .and_then(|def_key| deprecation.get(def_key))
                .map(|reason| Diagnostic {
                    range: symbol.key.text_range(),
                    severity,
                    code: DIAGNOSTIC_CODE.into(),
                    message: if let Some(reason) = reason {
                        format!("{} `{}` is deprecated: {reason}", symbol.kind, symbol.idx.render(db))
                    } else {
                        format!("{} `{}` is deprecated", symbol.kind, symbol.idx.render(db))
                    },
                    tags: Some(vec![DiagnosticTag::Deprecated]),
                    ..Default::default()
                })
        });
    diagnostics.extend(deprecated_usage);
}
