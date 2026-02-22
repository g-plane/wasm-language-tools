use crate::{NodeOrToken, SyntaxKind, SyntaxNode, SyntaxToken, TokenAtOffset};
use text_size::{TextRange, TextSize};

pub(crate) type SyntaxElement = NodeOrToken<SyntaxNode, SyntaxToken>;

impl SyntaxElement {
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
    pub(crate) fn token_at_offset(&self, offset: TextSize) -> TokenAtOffset {
        match self {
            NodeOrToken::Node(node) => node.token_at_offset(offset),
            NodeOrToken::Token(token) => TokenAtOffset::Single(token.clone()),
        }
    }
}

impl From<SyntaxNode> for SyntaxElement {
    #[inline]
    fn from(node: SyntaxNode) -> Self {
        NodeOrToken::Node(node)
    }
}
impl From<SyntaxToken> for SyntaxElement {
    #[inline]
    fn from(token: SyntaxToken) -> Self {
        NodeOrToken::Token(token)
    }
}
