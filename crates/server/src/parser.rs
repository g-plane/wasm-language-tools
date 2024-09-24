use crate::diag::Diagnostic;
use wat_parser::Parser;

pub fn parse(source: &str) -> Vec<Diagnostic> {
    let mut parser = Parser::new(source);
    parser.parse();
    parser
        .errors()
        .iter()
        .map(|error| Diagnostic {
            start: error.start,
            end: error.end,
            message: error.message.to_string(),
        })
        .collect()
}
