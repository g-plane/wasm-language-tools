use crate::{GreenNode, GreenToken, NodeOrToken, SyntaxKind};
use text_size::TextSize;

impl NodeOrToken<GreenNode, GreenToken> {
    #[inline]
    pub fn kind(&self) -> SyntaxKind {
        match self {
            NodeOrToken::Node(node) => node.kind(),
            NodeOrToken::Token(token) => token.kind(),
        }
    }

    #[inline]
    pub fn text_len(&self) -> TextSize {
        match self {
            NodeOrToken::Node(node) => node.text_len(),
            NodeOrToken::Token(token) => token.text_len(),
        }
    }
}

impl From<GreenNode> for NodeOrToken<GreenNode, GreenToken> {
    #[inline]
    fn from(node: GreenNode) -> Self {
        NodeOrToken::Node(node)
    }
}

impl From<GreenToken> for NodeOrToken<GreenNode, GreenToken> {
    #[inline]
    fn from(token: GreenToken) -> Self {
        NodeOrToken::Token(token)
    }
}
