use super::node::NodeData;
use crate::{GreenToken, SyntaxElement, SyntaxKind, SyntaxNode, green::GreenChild};
use std::{fmt, ptr::NonNull, rc::Rc};
use text_size::{TextRange, TextSize};

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct TokenData {
    green: NonNull<GreenToken>,
    range: TextRange,
    parent: Rc<NodeData>,
    index: u32,
}

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
    pub fn next_sibling_or_token(&self) -> Option<SyntaxElement> {
        let i = self.data.index + 1;
        let parent = self.parent();
        parent.green().slice().get(i as usize).map(|child| match child {
            GreenChild::Node { offset, node } => parent.new_child(i, node, *offset).into(),
            GreenChild::Token { offset, token } => parent.new_token(i, token, *offset).into(),
        })
    }

    #[inline]
    /// Including current node.
    pub fn next_siblings_with_tokens(&self) -> impl Iterator<Item = SyntaxElement> {
        let parent = &self.data.parent;
        parent
            .green()
            .slice()
            .iter()
            .enumerate()
            .skip(self.data.index as usize)
            .map(|(i, child)| match child {
                GreenChild::Node { offset, node } => {
                    SyntaxNode::new(i as u32, node, parent.range().start() + offset, Rc::clone(parent)).into()
                }
                GreenChild::Token { offset, token } => {
                    SyntaxToken::new(i as u32, token, parent.range().start() + offset, Rc::clone(parent)).into()
                }
            })
    }

    #[inline]
    pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement> {
        let i = self.data.index.checked_sub(1)?;
        let parent = self.parent();
        parent.green().slice().get(i as usize).map(|child| match child {
            GreenChild::Node { offset, node } => parent.new_child(i, node, *offset).into(),
            GreenChild::Token { offset, token } => parent.new_token(i, token, *offset).into(),
        })
    }

    #[inline]
    /// Including current node.
    pub fn prev_siblings_with_tokens(&self) -> impl Iterator<Item = SyntaxElement> {
        let parent = &self.data.parent;
        let slice = parent.green().slice();
        slice
            .iter()
            .enumerate()
            .rev()
            .skip(slice.len() - self.data.index as usize - 1)
            .map(|(i, child)| match child {
                GreenChild::Node { offset, node } => {
                    SyntaxNode::new(i as u32, node, parent.range().start() + offset, Rc::clone(parent)).into()
                }
                GreenChild::Token { offset, token } => {
                    SyntaxToken::new(i as u32, token, parent.range().start() + offset, Rc::clone(parent)).into()
                }
            })
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
