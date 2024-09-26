use crate::diag::Diagnostic;
use line_index::{LineIndex, TextSize};
use lsp_types::{Position, Range};
use rowan::ast::AstNode;
use wat_parser::Parser;
use wat_syntax::ast::Root;

#[derive(Clone, Debug)]
pub struct File {
    pub line_index: LineIndex,
    pub tree: Root,
    pub syntax_errors: Vec<Diagnostic>,
}
impl File {
    pub fn new(source: &str) -> Self {
        let line_index = LineIndex::new(source);

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

        Self {
            line_index,
            tree,
            syntax_errors,
        }
    }
}
