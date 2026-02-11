use wat_formatter::format;
use wat_syntax::{
    SyntaxNode,
    ast::{AstNode, Root},
};

#[test]
fn new_line() {
    let (tree, _) = wat_parser::parse("\n");
    let output = format(&Root::cast(SyntaxNode::new_root(tree)).unwrap(), &Default::default());
    assert_eq!(output, "\n");
}
