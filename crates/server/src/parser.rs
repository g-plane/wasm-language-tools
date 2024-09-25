use crate::{
    diag::Diagnostic,
    files::{get_line_index, Files},
};
use comemo::Tracked;
use line_index::TextSize;
use lsp_types::Uri;
use lsp_types::{Position, Range};
use wat_parser::Parser;

#[comemo::memoize]
pub fn parse(uri: &Uri, files: Tracked<Files>) -> Vec<Diagnostic> {
    let file = files.read(uri);
    let line_index = get_line_index(uri, files);
    let mut parser = Parser::new(&file);
    let _tree = parser.parse();

    parser
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
        .collect()
}
