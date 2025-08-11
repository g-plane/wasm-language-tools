use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
    term::{self, termcolor::Buffer},
};
use insta::{assert_snapshot, glob, Settings};
use std::{fs, path::Path};

#[test]
fn parser_snapshot() {
    glob!("parse/**/*.wat", |path| {
        let input = fs::read_to_string(path).unwrap();
        let (tree, errors) = wat_parser::parse2(&input);
        similar_asserts::assert_eq!(
            tree.to_string(),
            input,
            "syntax tree of '{}' does not match source",
            path.display()
        );

        let file = SimpleFile::new(path.file_name().unwrap().to_str().unwrap(), &input);
        let config = term::Config::default();
        let mut buffer = Buffer::no_color();
        errors
            .into_iter()
            .map(|error| {
                Diagnostic::error()
                    .with_message(error.message.to_string())
                    .with_labels(vec![Label::primary((), error.start..error.end)])
            })
            .for_each(|diagnostic| {
                term::emit(&mut buffer, &config, &file, &diagnostic).unwrap();
            });

        build_settings(path).bind(|| {
            let name = path.file_stem().unwrap().to_str().unwrap();
            assert_snapshot!(
                name,
                format!(
                    "{tree:#?}\n{}",
                    String::from_utf8(buffer.into_inner()).unwrap()
                )
            );
        });
    });
}

fn build_settings(path: &Path) -> Settings {
    let mut settings = Settings::clone_current();
    settings.set_snapshot_path(path.parent().unwrap());
    settings.remove_snapshot_suffix();
    settings.set_prepend_module_to_snapshot(false);
    settings.remove_input_file();
    settings.set_omit_expression(true);
    settings.remove_input_file();
    settings.remove_info();
    settings
}
