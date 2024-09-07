use std::{env, fs};

fn main() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let mut parser = wat_parser::Parser::new(&input);
    let tree = parser.parse();
    println!("{tree:#?}");
    dbg!(parser.errors());
    similar_asserts::assert_eq!(input, tree.to_string());
}
