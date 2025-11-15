use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};
use std::{env, fs};
use wat_syntax::SyntaxNode;

fn main() {
    let path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(&path).unwrap();
    let (green, errors) = wat_parser::parse(&input);
    println!("{:#?}", SyntaxNode::new_root(green.clone()));

    if !errors.is_empty() {
        let file = SimpleFile::new(path, &input);
        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = term::Config::default();
        errors.into_iter().for_each(|error| {
            let diagnostic = Diagnostic::error()
                .with_message(error.message.to_string())
                .with_labels(vec![Label::primary(
                    (),
                    error.range.start().into()..error.range.end().into(),
                )]);
            term::emit_to_write_style(&mut writer.lock(), &config, &file, &diagnostic).unwrap();
        });
    }

    similar_asserts::assert_eq!(input, green.to_string());
}
