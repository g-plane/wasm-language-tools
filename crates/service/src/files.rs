use crate::InternUri;
use line_index::{LineIndex, TextSize};
use lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range, Uri};
use rowan::GreenNode;
use std::rc::Rc;
use wat_parser::{Message, Parser};

#[salsa::query_group(Files)]
pub(crate) trait FilesCtx: salsa::Database {
    #[salsa::interned]
    fn uri(&self, uri: Uri) -> InternUri;

    #[salsa::input]
    fn source(&self, uri: InternUri) -> String;

    #[salsa::memoized]
    #[salsa::invoke(get_line_index)]
    fn line_index(&self, uri: InternUri) -> Rc<LineIndex>;

    #[salsa::memoized]
    #[salsa::invoke(parse)]
    fn parser_result(&self, uri: InternUri) -> (GreenNode, Vec<Diagnostic>);

    #[salsa::memoized]
    fn root(&self, uri: InternUri) -> GreenNode;
}

fn get_line_index(db: &dyn FilesCtx, uri: InternUri) -> Rc<LineIndex> {
    Rc::new(LineIndex::new(&db.source(uri)))
}

fn parse(db: &dyn FilesCtx, uri: InternUri) -> (GreenNode, Vec<Diagnostic>) {
    let source = db.source(uri);
    let line_index = db.line_index(uri);
    let mut parser = Parser::new(&source);
    let green = parser.parse_to_green();
    let syntax_errors = parser
        .errors()
        .iter()
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
    (green, syntax_errors)
}

fn root(db: &dyn FilesCtx, uri: InternUri) -> GreenNode {
    db.parser_result(uri).0
}
