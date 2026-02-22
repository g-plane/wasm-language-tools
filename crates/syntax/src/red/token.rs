use super::{element::SyntaxElement, node::NodeData};
use crate::{AmberToken, GreenToken, SyntaxKind, SyntaxNode};
use std::{fmt, ptr::NonNull, rc::Rc};
use text_size::{TextRange, TextSize};

#[derive(Clone, PartialEq, Eq, Hash)]
/// Leaf token in the red syntax tree.
pub struct SyntaxToken {
    pub(crate) data: Rc<TokenData>,
}

impl SyntaxToken {
    #[inline]
    pub(crate) fn new(index: u32, green: &GreenToken, offset: TextSize, parent: Rc<NodeData>) -> Self {
        SyntaxToken {
            data: Rc::new(TokenData {
                green: NonNull::from(green),
                range: TextRange::new(offset, offset + green.text_len()),
                parent,
                index,
            }),
        }
    }

    #[inline]
    /// Kind of this token.
    pub fn kind(&self) -> SyntaxKind {
        self.green().kind()
    }

    #[inline]
    /// The range that this token covers in the original text.
    pub fn text_range(&self) -> TextRange {
        self.data.range
    }

    #[inline]
    /// Text of this token.
    pub fn text(&self) -> &str {
        self.green().text()
    }

    #[inline]
    /// The underlying green token of this red token.
    pub fn green(&self) -> &GreenToken {
        unsafe { self.data.green.as_ref() }
    }

    #[inline]
    /// The corresponding amber token of this red token.
    pub fn amber(&self) -> AmberToken<'_> {
        self.into()
    }

    #[inline]
    /// Parent of this token.
    pub fn parent(&self) -> SyntaxNode {
        SyntaxNode {
            data: Rc::clone(&self.data.parent),
        }
    }

    #[inline]
    /// Iterator along the chain of parents of this token.
    pub fn parent_ancestors(&self) -> impl Iterator<Item = SyntaxNode> {
        std::iter::successors(Some(self.parent()), SyntaxNode::parent)
    }

    #[inline]
    /// Nodes that come immediately after this token.
    ///
    /// If you want to iterate over both nodes and tokens, use [`next_siblings_with_tokens`](Self::next_siblings_with_tokens) instead.
    pub fn next_siblings(&self) -> impl Iterator<Item = SyntaxNode> {
        self.data.parent.next_children(self.data.index)
    }

    #[inline]
    /// Node or token that comes immediately after this token.
    pub fn next_sibling_or_token(&self) -> Option<SyntaxElement> {
        self.data.parent.next_child_or_token(self.data.index)
    }

    #[inline]
    /// Nodes and tokens that come immediately after this token.
    ///
    /// Unlike rowan, the iterator doesn't contain the current token itself.
    pub fn next_siblings_with_tokens(&self) -> impl Iterator<Item = SyntaxElement> {
        self.data.parent.next_children_with_tokens(self.data.index)
    }

    #[inline]
    /// Consecutive tokens sequence that come immediately after this token without any nodes in between.
    pub fn next_consecutive_tokens(&self) -> impl Iterator<Item = SyntaxToken> {
        self.data.parent.next_consecutive_tokens(self.data.index)
    }

    #[inline]
    /// Nodes that come immediately before this token.
    ///
    /// If you want to iterate over both nodes and tokens, use [`prev_siblings_with_tokens`](Self::prev_siblings_with_tokens) instead.
    pub fn prev_siblings(&self) -> impl Iterator<Item = SyntaxNode> {
        self.data.parent.prev_children(self.data.index)
    }

    #[inline]
    /// Node or token that comes immediately before this token.
    pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement> {
        self.data.parent.prev_child_or_token(self.data.index)
    }

    #[inline]
    /// Nodes and tokens that come immediately before this token.
    ///
    /// Unlike rowan, the iterator doesn't contain the current token itself.
    pub fn prev_siblings_with_tokens(&self) -> impl Iterator<Item = SyntaxElement> {
        self.data.parent.prev_children_with_tokens(self.data.index)
    }

    #[inline]
    /// Consecutive tokens sequence that come immediately before this token without any nodes in between.
    pub fn prev_consecutive_tokens(&self) -> impl Iterator<Item = SyntaxToken> {
        self.data.parent.prev_consecutive_tokens(self.data.index)
    }
}

impl fmt::Debug for SyntaxToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}@{:?} {:?}", self.kind(), self.text_range(), self.text())
    }
}

impl fmt::Display for SyntaxToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.text().fmt(f)
    }
}

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct TokenData {
    green: NonNull<GreenToken>,
    range: TextRange,
    parent: Rc<NodeData>,
    index: u32,
}
