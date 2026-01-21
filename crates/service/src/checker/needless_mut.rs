use crate::{
    binder::{SymbolKind, SymbolTable},
    config::LintLevel,
    document::Document,
    helpers, mutability,
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, DiagnosticTag, Union2};

const DIAGNOSTIC_CODE: &str = "needless-mut";

pub fn check(
    db: &dyn salsa::Database,
    diagnostics: &mut Vec<Diagnostic>,
    lint_level: LintLevel,
    document: Document,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
) {
    let severity = match lint_level {
        LintLevel::Allow => return,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };

    let mutation_actions = mutability::get_mutation_actions(db, document);
    diagnostics.extend(
        mutability::get_mutabilities(db, document)
            .iter()
            .filter(|(key, mutability)| {
                mutability.mut_keyword.is_some()
                    && !mutability.cross_module
                    && mutation_actions
                        .values()
                        .filter(|action| action.target.is_some_and(|target| target == **key))
                        .all(|action| action.kind == mutability::MutationActionKind::Get)
            })
            .filter_map(|(key, mutability)| symbol_table.symbols.get(key).zip(mutability.mut_keyword))
            .map(|(symbol, keyword_range)| {
                let kind = match symbol.kind {
                    SymbolKind::GlobalDef => "global",
                    SymbolKind::Type => "array",
                    SymbolKind::FieldDef => "field",
                    _ => unreachable!(),
                };
                Diagnostic {
                    range: helpers::rowan_range_to_lsp_range(line_index, keyword_range),
                    severity: Some(severity),
                    source: Some("wat".into()),
                    code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                    message: format!("{kind} `{}` is unnecessarily mutable", symbol.idx.render(db)),
                    tags: Some(vec![DiagnosticTag::Unnecessary]),
                    ..Default::default()
                }
            }),
    );
}
