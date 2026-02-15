use super::node::NodeData;
use crate::{GreenToken, SyntaxElement, SyntaxKind, SyntaxNode};
use std::{fmt, ptr::NonNull, rc::Rc};
use text_size::{TextRange, TextSize};

#[derive(Clone, PartialEq, Eq, Hash)]
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
    pub fn kind(&self) -> SyntaxKind {
        self.green().kind()
    }

    #[inline]
    pub fn text_range(&self) -> TextRange {
        self.data.range
    }

    #[inline]
    pub fn text(&self) -> &str {
        self.green().text()
    }

    #[inline]
    pub fn green(&self) -> &GreenToken {
        unsafe { self.data.green.as_ref() }
    }

    #[inline]
    pub fn parent(&self) -> SyntaxNode {
        SyntaxNode {
            data: Rc::clone(&self.data.parent),
        }
    }

    #[inline]
    pub fn parent_ancestors(&self) -> impl Iterator<Item = SyntaxNode> {
        std::iter::successors(Some(self.parent()), SyntaxNode::parent)
    }

    #[inline]
    pub fn next_siblings(&self) -> impl Iterator<Item = SyntaxNode> {
        self.data.parent.next_children(self.data.index)
    }

    #[inline]
    pub fn next_sibling_or_token(&self) -> Option<SyntaxElement> {
        self.data.parent.next_child_or_token(self.data.index)
    }

    #[inline]
    pub fn next_siblings_with_tokens(&self) -> impl Iterator<Item = SyntaxElement> {
        self.data.parent.next_children_with_tokens(self.data.index)
    }

    #[inline]
    pub fn next_consecutive_tokens(&self) -> impl Iterator<Item = SyntaxToken> {
        self.data.parent.next_consecutive_tokens(self.data.index)
    }

    #[inline]
    pub fn prev_siblings(&self) -> impl Iterator<Item = SyntaxNode> {
        self.data.parent.prev_children(self.data.index)
    }

    #[inline]
    pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement> {
        self.data.parent.prev_child_or_token(self.data.index)
    }

    #[inline]
    pub fn prev_siblings_with_tokens(&self) -> impl Iterator<Item = SyntaxElement> {
        self.data.parent.prev_children_with_tokens(self.data.index)
    }

    #[inline]
    pub fn prev_consecutive_tokens(&self) -> impl Iterator<Item = SyntaxToken> {
        self.data.parent.prev_consecutive_tokens(self.data.index)
    }

    #[inline]
    pub fn has_prev_sibling_or_token(&self) -> bool {
        self.data.index > 0
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
