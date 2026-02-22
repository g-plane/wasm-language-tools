#![doc = include_str!("../README.md")]

mod amber;
pub mod ast;
mod green;
mod helpers;
mod kind;
mod red;

pub use self::{
    amber::{AmberNode, AmberToken},
    green::{GreenNode, GreenToken},
    helpers::NodeOrToken,
    kind::{SyntaxKind, SyntaxKindMatch},
    red::{SyntaxNode, SyntaxNodeChildren, SyntaxNodePtr, SyntaxToken, TokenAtOffset},
};
pub use text_size::{TextRange, TextSize};
