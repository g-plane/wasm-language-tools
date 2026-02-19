use wat_formatter::format;

#[test]
fn new_line() {
    let (root, _) = wat_parser::parse("\n");
    let output = format(&root, &Default::default());
    assert_eq!(output, "\n");
}
