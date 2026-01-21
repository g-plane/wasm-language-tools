use rowan::ast::AstNode;
use std::{env, error::Error, fs, io};
use wat_formatter::{config::FormatOptions, format};
use wat_syntax::{SyntaxNode, ast::Root};

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(&file_path)?;
    let options = match fs::read_to_string("config.json") {
        Ok(s) => serde_json::from_str(&s)?,
        Err(error) => {
            if error.kind() == io::ErrorKind::NotFound {
                FormatOptions::default()
            } else {
                return Err(Box::new(error));
            }
        }
    };

    let (tree, _) = wat_parser::parse(&input);
    print!("{}", format(&Root::cast(SyntaxNode::new_root(tree)).unwrap(), &options));
    Ok(())
}
