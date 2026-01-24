use super::Diagnostic;
use crate::document::Document;
use wat_parser::Message;

const DIAGNOSTIC_CODE: &str = "syntax";

pub fn check(db: &dyn salsa::Database, diagnostics: &mut Vec<Diagnostic>, document: Document) {
    diagnostics.extend(document.syntax_errors(db).iter().map(|error| Diagnostic {
        range: error.range,
        code: if let Message::Name(name) = error.message {
            format!("{DIAGNOSTIC_CODE}/{}", name.replace(' ', "-"))
        } else {
            DIAGNOSTIC_CODE.into()
        },
        message: format!("syntax error: {}", error.message),
        ..Default::default()
    }));
}
