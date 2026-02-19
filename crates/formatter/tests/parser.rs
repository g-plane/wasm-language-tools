use insta::{Settings, assert_snapshot, glob};
use std::{fs, path::Path};
use wat_formatter::{config::FormatOptions, format};

#[test]
fn parser_fixture_snapshot() {
    glob!("../../parser/tests", "parse/*.wat", |path| {
        let input = fs::read_to_string(path).unwrap();
        let output = run_format_test(path, &input, &Default::default());
        build_settings().bind(|| {
            let name = path.file_stem().unwrap().to_str().unwrap();
            assert_snapshot!(name, output);
        });
    });
}

fn run_format_test(path: &Path, input: &str, options: &FormatOptions) -> String {
    let (root, _) = wat_parser::parse(input);
    let output = format(&root, options);

    similar_asserts::assert_eq!(
        output.replace(" \n", "\n"),
        output,
        "'{}' has trailing whitespaces",
        path.display()
    );

    let (root, _) = wat_parser::parse(input);
    let regression_format = format(&root, options);
    similar_asserts::assert_eq!(output, regression_format, "'{}' format is unstable", path.display());

    output
}

fn build_settings() -> Settings {
    let mut settings = Settings::clone_current();
    settings.set_snapshot_path(Path::new("./parser/"));
    settings.remove_snapshot_suffix();
    settings.set_prepend_module_to_snapshot(false);
    settings.remove_input_file();
    settings.set_omit_expression(true);
    settings.remove_input_file();
    settings.remove_info();
    settings
}
