mod element;
mod iter;
mod node;
mod ptr;
mod token;
mod traversal;

pub use self::{
    element::SyntaxElement, iter::SyntaxNodeChildren, node::SyntaxNode, ptr::SyntaxNodePtr, token::SyntaxToken,
    traversal::Descendants,
};

#[derive(Clone, Debug)]
pub enum TokenAtOffset {
    None,
    Single(SyntaxToken),
    Between(SyntaxToken, SyntaxToken),
}
