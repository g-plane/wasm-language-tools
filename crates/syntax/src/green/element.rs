use crate::{GreenNode, GreenToken, NodeOrToken, SyntaxKind};

impl<'a> NodeOrToken<&'a GreenNode, &'a GreenToken> {
    #[inline]
    pub fn kind(&self) -> SyntaxKind {
        match self {
            NodeOrToken::Node(node) => node.kind(),
            NodeOrToken::Token(token) => token.kind(),
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
