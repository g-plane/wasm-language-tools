use crate::{
    LanguageService,
    binder::{SymbolKind, SymbolTable},
    config::LintLevel,
    deprecation,
    document::Document,
    helpers,
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, DiagnosticTag, Union2};

const DIAGNOSTIC_CODE: &str = "deprecated";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    service: &LanguageService,
    document: Document,
    lint_level: LintLevel,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
) {
    let severity = match lint_level {
        LintLevel::Allow => return,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };
    let deprecation = deprecation::get_deprecation(service, document);
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
            )
        })
        .filter_map(|symbol| {
            symbol_table
                .resolved
                .get(&symbol.key)
                .and_then(|def_key| deprecation.get(def_key))
                .map(|reason| Diagnostic {
                    range: helpers::rowan_range_to_lsp_range(line_index, symbol.key.text_range()),
                    severity: Some(severity),
                    source: Some("wat".into()),
                    code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                    message: if let Some(reason) = reason {
                        format!(
                            "{} `{}` is deprecated: {reason}",
                            symbol.kind,
                            symbol.idx.render(service),
                        )
                    } else {
                        format!(
                            "{} `{}` is deprecated",
                            symbol.kind,
                            symbol.idx.render(service),
                        )
                    },
                    tags: Some(vec![DiagnosticTag::Deprecated]),
                    ..Default::default()
                })
        });
    diagnostics.extend(deprecated_usage);
}
