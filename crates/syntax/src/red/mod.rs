mod element;
mod iter;
mod node;
mod ptr;
mod token;
mod traversal;

pub use self::{iter::SyntaxNodeChildren, node::SyntaxNode, ptr::SyntaxNodePtr, token::SyntaxToken};

#[derive(Clone, Debug)]
/// The result type of [`SyntaxNode::token_at_offset`] method, representing there may be zero, one or two tokens at the given offset.
pub enum TokenAtOffset<'a> {
    None,
    Single(SyntaxToken<'a>),
    Between(SyntaxToken<'a>, SyntaxToken<'a>),
}
