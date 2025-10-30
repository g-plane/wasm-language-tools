use crate::{
    LanguageService,
    binder::{SymbolKey, SymbolTable},
    config::LintLevel,
    helpers,
    uri::InternUri,
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, Union2};
use rowan::ast::{AstNode, support};
use rustc_hash::FxHashMap;
use wat_syntax::{
    SyntaxNode,
    ast::{Cat, Catch, CatchAll},
};

const DIAGNOSTIC_CODE: &str = "useless-catch";

pub fn check(
    service: &LanguageService,
    diagnostics: &mut Vec<Diagnostic>,
    lint_level: LintLevel,
    uri: InternUri,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) {
    let severity = match lint_level {
        LintLevel::Allow => return,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };
    let mut default_match: Option<CatchAll> = None;
    let mut matches = FxHashMap::<_, Catch>::default();
    support::children::<Cat>(node).for_each(|cat| match cat {
        Cat::Catch(catch) => {
            if let Some(def_key) = catch
                .tag_index()
                .and_then(|index| symbol_table.resolved.get(&SymbolKey::new(index.syntax())))
            {
                let matched = match (matches.get(def_key), &default_match) {
                    (Some(catch), Some(catch_all)) => {
                        let catch = catch.syntax();
                        let catch_all = catch_all.syntax();
                        if catch.text_range().start() < catch_all.text_range().start() {
                            catch
                        } else {
                            catch_all
                        }
                    }
                    (Some(catch), None) => catch.syntax(),
                    (None, Some(catch_all)) => catch_all.syntax(),
                    (None, None) => {
                        matches.insert(def_key, catch);
                        return;
                    }
                };
                diagnostics.push(build_diagnostic(
                    catch.syntax(),
                    matched,
                    line_index,
                    uri,
                    service,
                    severity,
                ));
            }
        }
        Cat::CatchAll(catch_all) => {
            if let Some(matched) = &default_match {
                diagnostics.push(build_diagnostic(
                    catch_all.syntax(),
                    matched.syntax(),
                    line_index,
                    uri,
                    service,
                    severity,
                ));
            } else {
                default_match = Some(catch_all);
            }
        }
    });
}

fn build_diagnostic(
    reported: &SyntaxNode,
    related: &SyntaxNode,
    line_index: &LineIndex,
    uri: InternUri,
    service: &LanguageService,
    severity: DiagnosticSeverity,
) -> Diagnostic {
    Diagnostic {
        range: helpers::rowan_range_to_lsp_range(line_index, reported.text_range()),
        severity: Some(severity),
        source: Some("wat".into()),
        code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
        message: "this catch clause will never be matched".into(),
        related_information: Some(vec![DiagnosticRelatedInformation {
            location: Location {
                uri: uri.raw(service),
                range: helpers::rowan_range_to_lsp_range(line_index, related.text_range()),
            },
            message: "catch clause already matched here".into(),
        }]),
        ..Default::default()
    }
}
