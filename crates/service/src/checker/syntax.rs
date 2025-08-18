use crate::{helpers, syntax_tree::SyntaxTreeCtx, uri::InternUri, LanguageService};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use wat_parser::Message;

const DIAGNOSTIC_CODE: &str = "syntax";

pub fn check(
    service: &LanguageService,
    diagnostics: &mut Vec<Diagnostic>,
    uri: InternUri,
    line_index: &LineIndex,
) {
    diagnostics.extend(service.syntax_errors(uri).iter().map(|error| Diagnostic {
        range: helpers::rowan_range_to_lsp_range(line_index, error.range),
        severity: Some(DiagnosticSeverity::Error),
        source: Some("wat".into()),
        code: if let Message::Name(name) = error.message {
            Some(Union2::B(format!(
                "{DIAGNOSTIC_CODE}/{}",
                name.replace(' ', "-")
            )))
        } else {
            Some(Union2::B(DIAGNOSTIC_CODE.into()))
        },
        message: format!("syntax error: {}", error.message),
        ..Default::default()
    }));
}
