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
    pub fn parent(&self) -> Option<SyntaxNode> {
        match self {
            NodeOrToken::Node(node) => node.parent(),
            NodeOrToken::Token(token) => Some(token.parent()),
        }
    }

    #[inline]
    pub fn next_sibling_or_token(&self) -> Option<SyntaxElement> {
        match self {
            NodeOrToken::Node(node) => node.next_sibling_or_token(),
            NodeOrToken::Token(token) => token.next_sibling_or_token(),
        }
    }

    #[inline]
    pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement> {
        match self {
            NodeOrToken::Node(node) => node.prev_sibling_or_token(),
            NodeOrToken::Token(token) => token.prev_sibling_or_token(),
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
