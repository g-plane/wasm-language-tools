use super::{Diagnostic, RelatedInformation};
use crate::{
    binder::{SymbolKey, SymbolTable},
    config::LintLevel,
};
use lspt::{DiagnosticSeverity, DiagnosticTag};
use rustc_hash::FxHashMap;
use wat_syntax::{
    SyntaxNode,
    ast::{AstNode, Cat, Catch, CatchAll, support},
};

const DIAGNOSTIC_CODE: &str = "useless-catch";

pub fn check(diagnostics: &mut Vec<Diagnostic>, lint_level: LintLevel, symbol_table: &SymbolTable, node: &SyntaxNode) {
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
                diagnostics.push(build_diagnostic(catch.syntax(), matched, severity));
            }
        }
        Cat::CatchAll(catch_all) => {
            if let Some(matched) = &default_match {
                diagnostics.push(build_diagnostic(catch_all.syntax(), matched.syntax(), severity));
            } else {
                default_match = Some(catch_all);
            }
        }
    });
}

fn build_diagnostic(reported: &SyntaxNode, related: &SyntaxNode, severity: DiagnosticSeverity) -> Diagnostic {
    Diagnostic {
        range: reported.text_range(),
        severity,
        code: DIAGNOSTIC_CODE.into(),
        message: "this catch clause will never be matched".into(),
        tags: Some(vec![DiagnosticTag::Unnecessary]),
        related_information: Some(vec![RelatedInformation {
            range: related.text_range(),
            message: "catch clause already matched here".into(),
        }]),
        ..Default::default()
    }
}
