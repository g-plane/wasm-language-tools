use self::lexer::Lexer;
use crate::error::SyntaxError;
use wat_syntax::{GreenNode, GreenToken, NodeOrToken, SyntaxKind};

mod builder;
mod green;
mod helpers;
mod instr;
mod lexer;
mod module;
mod ty;

/// Parse the code into a rowan green node.
pub fn parse(source: &str) -> (GreenNode, Vec<SyntaxError>) {
    let mut parser = Parser::new(source);
    (parser.parse_root(), parser.errors)
}

#[inline]
/// Parse a snippet of module field starting from the specific offset.
///
/// It will return `None` if it isn't a module field.
///
/// Note that there can't be leading whitespaces or comments after that offset.
///
/// ## Examples
///
/// ```
/// # use wat_parser::parse_partial;
/// assert!(parse_partial("(module (fun))", 8).is_none());
/// assert!(parse_partial("(module (func))", 7).is_none());
/// assert!(!parse_partial("(module (func ()))", 8).unwrap().1.is_empty());
/// ```
pub fn parse_partial(source: &str, from: usize) -> Option<(GreenNode, Vec<SyntaxError>)> {
    let mut parser = Parser::offset_from(source, from);
    parser.parse_module_field().map(|green| (green, parser.errors))
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
        || c.is_ascii_punctuation() && !matches!(c, '"' | ',' | ';' | '(' | ')' | '[' | ']' | '{' | '}')
}

type GreenElement = NodeOrToken<GreenNode, GreenToken>;

fn node<I>(kind: SyntaxKind, children: I) -> GreenNode
where
    I: IntoIterator<Item = GreenElement>,
    I::IntoIter: ExactSizeIterator,
{
    GreenNode::new(kind, children)
}

struct Parser<'s> {
    source: &'s str,
    lexer: Lexer<'s>,
    errors: Vec<SyntaxError>,
    elements: Vec<GreenElement>,
}

impl<'s> Parser<'s> {
    fn new(source: &'s str) -> Self {
        Parser {
            source,
            lexer: Lexer::new(source),
            errors: Vec::new(),
            elements: Vec::new(),
        }
    }

    fn offset_from(source: &'s str, offset: usize) -> Self {
        Parser {
            source,
            lexer: Lexer::offset_from(source, offset),
            errors: Vec::new(),
            elements: Vec::new(),
        }
    }

    fn parse_root(&mut self) -> GreenNode {
        let mark = self.start_node();
        while self.recover(Self::parse_module) {}
        self.parse_trivias();
        self.finish_node(SyntaxKind::ROOT, mark)
    }
}
