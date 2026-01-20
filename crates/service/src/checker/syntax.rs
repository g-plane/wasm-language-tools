use crate::{document::Document, helpers};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use wat_parser::Message;

const DIAGNOSTIC_CODE: &str = "syntax";

pub fn check(
    db: &dyn salsa::Database,
    diagnostics: &mut Vec<Diagnostic>,
    document: Document,
    line_index: &LineIndex,
) {
    diagnostics.extend(document.syntax_errors(db).iter().map(|error| Diagnostic {
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
