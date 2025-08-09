use self::lexer::Lexer;
use crate::error::SyntaxError;
use rowan::{GreenNode, GreenToken, NodeOrToken};
use wat_syntax::{SyntaxKind, SyntaxNode};

mod helpers;
mod instr;
mod lexer;
mod module;
mod ty;

/// Parse the code into a rowan syntax node.
pub fn parse(source: &str) -> (SyntaxNode, Vec<SyntaxError>) {
    let (green, errors) = parse_to_green(source);
    (SyntaxNode::new_root(green), errors)
}

/// Parse the code into a rowan green node.
pub fn parse_to_green(source: &str) -> (GreenNode, Vec<SyntaxError>) {
    let mut parser = Parser::new(source);
    (parser.parse_root(), parser.errors)
}

#[inline]
/// Checks if a character is a valid identifier character.
///
/// ## Examples
///
/// ```
/// # use wat_parser::is_id_char;
/// assert!(is_id_char('a'));
/// assert!(is_id_char('Z'));
/// assert!(is_id_char('0'));
/// assert!(is_id_char('$'));
/// assert!(is_id_char('.'));
/// assert!(!is_id_char('('));
/// assert!(!is_id_char(')'));
/// ```
pub fn is_id_char(c: char) -> bool {
    c.is_ascii_alphanumeric()
        || c.is_ascii_punctuation()
            && !matches!(c, '"' | ',' | ';' | '(' | ')' | '[' | ']' | '{' | '}')
}

type GreenElement = NodeOrToken<GreenNode, GreenToken>;

fn node<I>(kind: SyntaxKind, children: I) -> GreenNode
where
    I: IntoIterator<Item = GreenElement>,
    I::IntoIter: ExactSizeIterator,
{
    GreenNode::new(kind.into(), children)
}

#[derive(Debug)]
struct Parser<'s> {
    source: &'s str,
    lexer: Lexer<'s>,
    errors: Vec<SyntaxError>,
}

impl<'s> Parser<'s> {
    fn new(source: &'s str) -> Self {
        Parser {
            source,
            lexer: Lexer::new(source),
            errors: Vec::new(),
        }
    }

    fn parse_root(&mut self) -> GreenNode {
        let mut children = Vec::with_capacity(2);
        while self.recover(Self::parse_module, &mut children) {}
        self.parse_trivias(&mut children);
        node(SyntaxKind::ROOT, children)
    }
}
