use crate::diag::Diagnostic;
use line_index::{LineIndex, TextSize};
use lsp_types::{Position, Range, Uri};
use rowan::ast::AstNode;
use wat_parser::Parser;
use wat_syntax::ast::Root;

#[salsa::query_group(Files)]
pub trait FilesCtx: salsa::Database {
    #[salsa::input]
    fn source(&self, uri: Uri) -> String;

    #[salsa::memoized]
    #[salsa::invoke(get_line_index)]
    fn line_index(&self, uri: Uri) -> LineIndex;

    #[salsa::dependencies]
    #[salsa::invoke(parse)]
    fn parser_result(&self, uri: Uri) -> (Root, Vec<Diagnostic>);

    #[salsa::dependencies]
    fn root(&self, uri: Uri) -> Root;
}

fn get_line_index(db: &dyn FilesCtx, uri: Uri) -> LineIndex {
    LineIndex::new(&db.source(uri))
}

fn parse(db: &dyn FilesCtx, uri: Uri) -> (Root, Vec<Diagnostic>) {
    let source = db.source(uri.clone());
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

fn root(db: &dyn FilesCtx, uri: Uri) -> Root {
    db.parser_result(uri).0
}
