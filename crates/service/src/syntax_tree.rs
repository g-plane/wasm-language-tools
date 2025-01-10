use crate::uri::InternUri;
use line_index::LineIndex;
use rowan::GreenNode;
use std::rc::Rc;
use wat_parser::{parse_to_green, SyntaxError};

#[salsa::query_group(SyntaxTree)]
pub(crate) trait SyntaxTreeCtx: salsa::Database {
    #[salsa::input]
    fn source(&self, uri: InternUri) -> String;

    #[salsa::memoized]
    #[salsa::invoke(get_line_index)]
    fn line_index(&self, uri: InternUri) -> Rc<LineIndex>;

    #[salsa::memoized]
    fn parse(&self, uri: InternUri) -> (GreenNode, Rc<Vec<SyntaxError>>);

    #[salsa::memoized]
    fn root(&self, uri: InternUri) -> GreenNode;
}

fn get_line_index(db: &dyn SyntaxTreeCtx, uri: InternUri) -> Rc<LineIndex> {
    Rc::new(LineIndex::new(&db.source(uri)))
}

fn parse(db: &dyn SyntaxTreeCtx, uri: InternUri) -> (GreenNode, Rc<Vec<SyntaxError>>) {
    let source = db.source(uri);
    let (green, errors) = parse_to_green(&source);
    (green, Rc::new(errors))
}

fn root(db: &dyn SyntaxTreeCtx, uri: InternUri) -> GreenNode {
    db.parse(uri).0
}
