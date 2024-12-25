use crate::{data_set::INSTR_NAMES, helpers};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use line_index::LineIndex;
use lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString};
use rowan::ast::support;
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "unknown-instr";

pub fn check(diags: &mut Vec<Diagnostic>, line_index: &LineIndex, node: &SyntaxNode) {
    let Some(token) = support::token(node, SyntaxKind::INSTR_NAME) else {
        return;
    };
    let instr_name = token.text();
    if !INSTR_NAMES.contains(&instr_name) {
        let matcher = SkimMatcherV2::default();
        let message = if let Some((_, guess)) = INSTR_NAMES
            .iter()
            .filter_map(|name| {
                matcher
                    .fuzzy_match(name, instr_name)
                    .map(|score| (score, name))
            })
            .max_by_key(|(score, _)| *score)
        {
            format!("unknown instruction `{instr_name}`, do you mean `{guess}`?")
        } else {
            format!("unknown instruction `{instr_name}`")
        };
        diags.push(Diagnostic {
            range: helpers::rowan_range_to_lsp_range(line_index, token.text_range()),
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("wat".into()),
            code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
            message,
            ..Default::default()
        });
    }
}
