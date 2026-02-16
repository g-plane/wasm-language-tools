use crate::{AmberNode, AmberToken, NodeOrToken, SyntaxKind};
use text_size::TextRange;

impl NodeOrToken<AmberNode<'_>, AmberToken<'_>> {
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
}

impl<'a> From<AmberNode<'a>> for NodeOrToken<AmberNode<'a>, AmberToken<'a>> {
    #[inline]
    fn from(node: AmberNode<'a>) -> Self {
        NodeOrToken::Node(node)
    }
}

impl<'a> From<AmberToken<'a>> for NodeOrToken<AmberNode<'a>, AmberToken<'a>> {
    #[inline]
    fn from(token: AmberToken<'a>) -> Self {
        NodeOrToken::Token(token)
    }
}
