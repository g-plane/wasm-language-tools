use crate::{diag::Diagnostic, InternUri};
use line_index::{LineIndex, TextSize};
use lsp_types::{Position, Range, Uri};
use rowan::ast::AstNode;
use wat_parser::Parser;
use wat_syntax::ast::Root;

#[salsa::query_group(Files)]
pub(crate) trait FilesCtx: salsa::Database {
    #[salsa::interned]
    fn uri(&self, uri: Uri) -> InternUri;

    #[salsa::input]
    fn source(&self, uri: InternUri) -> String;

    #[salsa::memoized]
    #[salsa::invoke(get_line_index)]
    fn line_index(&self, uri: InternUri) -> LineIndex;

    #[salsa::memoized]
    #[salsa::invoke(parse)]
    fn parser_result(&self, uri: InternUri) -> (Root, Vec<Diagnostic>);

    #[salsa::memoized]
    fn root(&self, uri: InternUri) -> Root;
}

fn get_line_index(db: &dyn FilesCtx, uri: InternUri) -> LineIndex {
    LineIndex::new(&db.source(uri))
}

fn parse(db: &dyn FilesCtx, uri: InternUri) -> (Root, Vec<Diagnostic>) {
    let source = db.source(uri);
    let line_index = db.line_index(uri);
    let mut parser = Parser::new(&source);
    let tree = Root::cast(parser.parse()).expect("expected AST root");
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
                message: format!("syntax error: {}", error.message),
            }
        })
        .collect();
    (tree, syntax_errors)
}

fn root(db: &dyn FilesCtx, uri: InternUri) -> Root {
    db.parser_result(uri).0
}
