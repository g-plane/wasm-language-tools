use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};
use std::{env, fs};

fn main() {
    let path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(&path).unwrap();
    let mut parser = wat_parser::Parser::new(&input);
    let tree = parser.parse();
    println!("{tree:#?}");

    if !parser.errors().is_empty() {
        let file = SimpleFile::new(path, &input);
        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = term::Config::default();
        parser.errors().iter().for_each(|error| {
            let diagnostic = Diagnostic::error()
                .with_message(error.message.to_string())
                .with_labels(vec![Label::primary((), error.start..error.end)]);
            term::emit(&mut writer.lock(), &config, &file, &diagnostic).unwrap();
        });
    }

    similar_asserts::assert_eq!(input, tree.to_string());
}
