use rowan::ast::AstNode;
use wat_formatter::format;
use wat_syntax::ast::Root;

#[test]
fn new_line() {
    let (tree, _) = wat_parser::parse2("\n");
    let output = format(&Root::cast(tree).unwrap(), &Default::default());
    assert_eq!(output, "\n");
}
