use super::traversal::{DescendantToken, DescendantTokens};
use crate::{
    AmberToken, GreenNode, NodeOrToken, SyntaxKind, SyntaxKindMatch, SyntaxNode, SyntaxNodePtr, green::GreenChild,
};
use text_size::{TextRange, TextSize};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
/// Node in the amber syntax tree.
///
/// It's a lightweight version of [`SyntaxNode`](crate::SyntaxNode) without access to parent and siblings.
/// It's much cheaper than [`SyntaxNode`](crate::SyntaxNode) to create and use.
/// This is preferred to use for better performance if you don't need to visit parent and siblings.
pub struct AmberNode<'a> {
    green: &'a GreenNode,
    range: TextRange,
}

impl<'a> AmberNode<'a> {
    #[inline]
    /// Build a new syntax tree on top of a green tree.
    pub fn new_root(green: &'a GreenNode) -> Self {
        Self {
            green,
            range: TextRange::new(0.into(), green.text_len()),
        }
    }

    #[inline]
    /// Create a new amber node with the given green node based on the offset.
    ///
    /// Note that passing wrong offset can cause unexpectedly incorrect syntax tree. Be careful.
    pub fn new(green: &'a GreenNode, start: TextSize) -> Self {
        Self {
            green,
            range: TextRange::new(start, start + green.text_len()),
        }
    }

    #[inline]
    /// Kind of this node.
    pub fn kind(&self) -> SyntaxKind {
        self.green.kind()
    }

    #[inline]
    /// The range that this node covers in the original text.
    pub fn text_range(&self) -> TextRange {
        self.range
    }

    #[inline]
    /// The underlying green node of this amber node.
    pub fn green(&self) -> &'a GreenNode {
        self.green
    }

    #[inline]
    /// The corresponding [`SyntaxNodePtr`](crate::SyntaxNodePtr) of this amber node.
    pub fn to_ptr(&self) -> SyntaxNodePtr {
        SyntaxNodePtr {
            kind: self.green.kind(),
            range: self.range,
        }
    }

    #[inline]
    /// Iterator over the child nodes of this node.
    ///
    /// If you want to iterate over both nodes and tokens, use [`children_with_tokens`](Self::children_with_tokens) instead.
    pub fn children(&self) -> impl DoubleEndedIterator<Item = AmberNode<'a>> + Clone + use<'a> {
        let start = self.range.start();
        self.green.slice().iter().filter_map(move |child| match child {
            GreenChild::Node { offset, node } => Some(AmberNode::new(node, start + offset)),
            GreenChild::Token { .. } => None,
        })
    }

    #[inline]
    /// Iterator over specific kinds of child nodes of this node.
    pub fn children_by_kind<M>(&self, matcher: M) -> impl DoubleEndedIterator<Item = AmberNode<'a>> + use<'a, M>
    where
        M: SyntaxKindMatch,
    {
        let start = self.range.start();
        self.green.slice().iter().filter_map(move |child| match child {
            GreenChild::Node { offset, node } if matcher.matches(node.kind()) => {
                Some(AmberNode::new(node, start + offset))
            }
            _ => None,
        })
    }

    #[inline]
    /// Iterator over specific kinds of child tokens of this node.
    pub fn tokens_by_kind<M>(&self, matcher: M) -> impl DoubleEndedIterator<Item = AmberToken<'a>> + use<'a, M>
    where
        M: SyntaxKindMatch,
    {
        let start = self.range.start();
        self.green.slice().iter().filter_map(move |child| match child {
            GreenChild::Token { offset, token } if matcher.matches(token.kind()) => {
                Some(AmberToken::new(token, start + offset))
            }
            _ => None,
        })
    }

    #[inline]
    /// Iterator over the child nodes and tokens of this node.
    pub fn children_with_tokens(
        &self,
    ) -> impl DoubleEndedIterator<Item = NodeOrToken<AmberNode<'a>, AmberToken<'a>>> + Clone + use<'a> {
        let start = self.range.start();
        self.green.slice().iter().map(move |child| match child {
            GreenChild::Node { offset, node } => AmberNode::new(node, start + offset).into(),
            GreenChild::Token { offset, token } => AmberToken::new(token, start + offset).into(),
        })
    }

    #[inline]
    /// Iterator over all tokens in the subtree.
    ///
    /// The iterator yields a three-component tuple:
    /// 1. current token
    /// 2. the parent of current token
    /// 3. the grandparent of current token
    pub fn descendant_tokens(&self) -> impl Iterator<Item = DescendantToken<'a>> + 'a {
        DescendantTokens::new(*self)
    }

    #[inline]
    /// Find a child node that intersects with the given range.
    pub fn child_at_range(&self, range: TextRange) -> Option<AmberNode<'a>> {
        if !self.range.contains_range(range) {
            return None;
        }
        let relative_range = range - self.range.start();
        self.green
            .child_at_range(relative_range)
            .map(|(node, offset, _)| AmberNode::new(node, self.range.start() + offset))
    }

    #[inline]
    pub(crate) fn child_or_token_at(&self, index: usize) -> Option<NodeOrToken<AmberNode<'a>, AmberToken<'a>>> {
        self.green.slice().get(index).map(|child| match child {
            GreenChild::Node { offset, node } => AmberNode::new(node, self.range.start() + offset).into(),
            GreenChild::Token { offset, token } => AmberToken::new(token, self.range.start() + offset).into(),
        })
    }
}

impl<'a> From<&SyntaxNode<'a>> for AmberNode<'a> {
    #[inline]
    fn from(node: &SyntaxNode<'a>) -> Self {
        Self {
            green: node.green(),
            range: node.text_range(),
        }
    }
}
