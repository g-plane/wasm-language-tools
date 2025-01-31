#![doc = include_str!("../README.md")]

pub mod ast;
mod kind;

pub use kind::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WatLanguage {}

impl rowan::Language for WatLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::ROOT as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub type SyntaxNode = rowan::SyntaxNode<WatLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<WatLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<WatLanguage>;
pub type SyntaxNodePtr = rowan::ast::SyntaxNodePtr<WatLanguage>;
