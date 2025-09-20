use insta::{Settings, assert_snapshot, glob};
use rowan::ast::AstNode;
use std::{collections::HashMap, fs, path::Path};
use wat_formatter::{config::FormatOptions, format};
use wat_syntax::{SyntaxNode, ast::Root};

#[test]
fn fmt_snapshot() {
    glob!("fmt/**/*.wat", |path| {
        let input = fs::read_to_string(path).unwrap();

        let options = fs::read_to_string(path.with_file_name("config.json"))
            .map(|config_file| {
                serde_json::from_str::<HashMap<String, FormatOptions>>(&config_file).unwrap()
            })
            .ok();

        if let Some(options) = options {
            options.into_iter().for_each(|(option_name, options)| {
                let output = run_format_test(path, &input, &options);
                build_settings(path).bind(|| {
                    let name = path.file_stem().unwrap().to_str().unwrap();
                    assert_snapshot!(format!("{name}.{option_name}"), output);
                });
            })
        } else {
            let output = run_format_test(path, &input, &Default::default());
            build_settings(path).bind(|| {
                let name = path.file_stem().unwrap().to_str().unwrap();
                assert_snapshot!(name, output);
            });
        }
    });
}

fn run_format_test(path: &Path, input: &str, options: &FormatOptions) -> String {
    let (tree, _) = wat_parser::parse(input);
    let output = format(&Root::cast(SyntaxNode::new_root(tree)).unwrap(), options);

    assert!(
        !output.starts_with('\n'),
        "'{}' has leading newline",
        path.display(),
    );
    assert!(
        !output.contains(" \n"),
        "'{}' has trailing whitespaces",
        path.display(),
    );

    let (tree, errors) = wat_parser::parse(&output);
    assert!(
        errors.is_empty(),
        "syntax error in stability test '{}': {:?}",
        path.display(),
        errors
    );
    let regression_format = format(&Root::cast(SyntaxNode::new_root(tree)).unwrap(), options);
    similar_asserts::assert_eq!(
        output,
        regression_format,
        "'{}' format is unstable",
        path.display()
    );

    output
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
