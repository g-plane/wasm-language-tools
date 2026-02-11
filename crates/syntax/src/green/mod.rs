pub use self::{node::GreenNode, token::GreenToken};
use crate::SyntaxKind;
use text_size::TextSize;

mod element;
mod node;
mod token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct GreenHead {
    kind: SyntaxKind,
    text_len: TextSize,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) enum GreenChild {
    Node { offset: TextSize, node: GreenNode },
    Token { offset: TextSize, token: GreenToken },
}
