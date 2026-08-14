use super::{Diagnostic, DiagnosticCtx, RelatedInformation};
use crate::config::LintLevel;
use lspt::{DiagnosticSeverity, DiagnosticTag};
use rustc_hash::FxHashMap;
use wat_syntax::{
    AmberNode, SyntaxKind,
    ast::{AstNode, Cat},
};

const DIAGNOSTIC_CODE: &str = "useless-catch";

pub fn check(diagnostics: &mut Vec<Diagnostic>, ctx: &DiagnosticCtx, node: AmberNode) {
    let severity = match ctx.config.lint.useless_catch {
        LintLevel::Allow => return,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };
    let mut default_match: Option<AmberNode> = None;
    let mut matches = FxHashMap::<_, AmberNode>::default();
    node.children_by_kind(Cat::can_cast).for_each(|cat| match cat.kind() {
        SyntaxKind::CATCH => {
            if let Some(def_symbol) = cat
                .children_by_kind(SyntaxKind::INDEX)
                .next()
                .and_then(|index| ctx.symbol_table.find_def(index.into()))
            {
                let matched = match (matches.get(&def_symbol.key), &default_match) {
                    (Some(catch), Some(catch_all)) => {
                        if catch.text_range().start() < catch_all.text_range().start() {
                            catch
                        } else {
                            catch_all
                        }
                    }
                    (Some(catch), None) => catch,
                    (None, Some(catch_all)) => catch_all,
                    (None, None) => {
                        matches.insert(def_symbol.key, cat);
                        return;
                    }
                };
                diagnostics.push(build_diagnostic(cat, *matched, severity));
            }
        }
        SyntaxKind::CATCH_ALL => {
            if let Some(matched) = &default_match {
                diagnostics.push(build_diagnostic(cat, *matched, severity));
            } else {
                default_match = Some(cat);
            }
        }
        _ => {}
    });
}

fn build_diagnostic(reported: AmberNode, related: AmberNode, severity: DiagnosticSeverity) -> Diagnostic {
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
