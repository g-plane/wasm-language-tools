#![doc = include_str!("../README.md")]

pub mod ast;
mod green;
mod helpers;
mod kind;
mod red;

pub use self::{
    green::{GreenNode, GreenToken},
    helpers::NodeOrToken,
    kind::SyntaxKind,
    red::{
        Descendants, DescendantsWithTokens, SyntaxElement, SyntaxElementChildren, SyntaxNode, SyntaxNodeChildren,
        SyntaxNodePtr, SyntaxToken, TokenAtOffset,
    },
};
pub use text_size::{TextRange, TextSize};
