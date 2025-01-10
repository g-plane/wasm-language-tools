use crate::uri::InternUri;
use line_index::{LineIndex, TextSize};
use lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range};
use rowan::GreenNode;
use std::rc::Rc;
use wat_parser::{parse_to_green, Message};

#[salsa::query_group(SyntaxTree)]
pub(crate) trait SyntaxTreeCtx: salsa::Database {
    #[salsa::input]
    fn source(&self, uri: InternUri) -> String;

    #[salsa::memoized]
    #[salsa::invoke(get_line_index)]
    fn line_index(&self, uri: InternUri) -> Rc<LineIndex>;

    #[salsa::memoized]
    fn parse(&self, uri: InternUri) -> (GreenNode, Rc<Vec<Diagnostic>>);

    #[salsa::memoized]
    fn root(&self, uri: InternUri) -> GreenNode;
}

fn get_line_index(db: &dyn SyntaxTreeCtx, uri: InternUri) -> Rc<LineIndex> {
    Rc::new(LineIndex::new(&db.source(uri)))
}

fn parse(db: &dyn SyntaxTreeCtx, uri: InternUri) -> (GreenNode, Rc<Vec<Diagnostic>>) {
    let source = db.source(uri);
    let line_index = db.line_index(uri);
    let (green, errors) = parse_to_green(&source);
    let syntax_errors = errors
        .into_iter()
        .map(|error| {
            let start = line_index.line_col(TextSize::new(error.start as u32));
            let end = line_index.line_col(TextSize::new(error.end as u32));
            Diagnostic {
                range: Range::new(
                    Position::new(start.line, start.col),
                    Position::new(end.line, end.col),
                ),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("wat".into()),
                code: if let Message::Name(name) = error.message {
                    Some(NumberOrString::String(format!(
                        "syntax/{}",
                        name.replace(' ', "-")
                    )))
                } else {
                    None
                },
                message: format!("syntax error: {}", error.message),
                ..Default::default()
            }
        })
        .collect();
    (green, Rc::new(syntax_errors))
}

fn root(db: &dyn SyntaxTreeCtx, uri: InternUri) -> GreenNode {
    db.parse(uri).0
}
