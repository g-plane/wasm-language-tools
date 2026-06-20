use crate::{NodeOrToken, SyntaxKind, SyntaxNode, SyntaxToken, TokenAtOffset};
use text_size::{TextRange, TextSize};

pub(crate) type SyntaxElement<'a> = NodeOrToken<SyntaxNode<'a>, SyntaxToken<'a>>;

impl<'a> SyntaxElement<'a> {
    #[inline]
    pub fn kind(&self) -> SyntaxKind {
        match self {
            NodeOrToken::Node(node) => node.kind(),
            NodeOrToken::Token(token) => token.kind(),
        }
    }

    #[inline]
    pub fn text_range(&self) -> TextRange {
        match self {
            NodeOrToken::Node(node) => node.text_range(),
            NodeOrToken::Token(token) => token.text_range(),
        }
    }

    #[inline]
    pub(crate) fn token_at_offset(&self, offset: TextSize) -> TokenAtOffset<'a> {
        match self {
            NodeOrToken::Node(node) => node.token_at_offset(offset),
            NodeOrToken::Token(token) => TokenAtOffset::Single(token.clone()),
        }
    }
}

impl<'a> From<SyntaxNode<'a>> for SyntaxElement<'a> {
    #[inline]
    fn from(node: SyntaxNode<'a>) -> Self {
        NodeOrToken::Node(node)
    }
}
impl<'a> From<SyntaxToken<'a>> for SyntaxElement<'a> {
    #[inline]
    fn from(token: SyntaxToken<'a>) -> Self {
        NodeOrToken::Token(token)
    }
}
