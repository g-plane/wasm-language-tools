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

#[inline]
/// Checks if a token is whitespace or comment.
pub fn is_trivia(token: &SyntaxToken) -> bool {
    matches!(
        token.kind(),
        SyntaxKind::WHITESPACE | SyntaxKind::LINE_COMMENT | SyntaxKind::BLOCK_COMMENT
    )
}

#[inline]
/// Checks if a token is punctuation.
pub fn is_punc(token: &SyntaxToken) -> bool {
    matches!(token.kind(), SyntaxKind::L_PAREN | SyntaxKind::R_PAREN)
}
