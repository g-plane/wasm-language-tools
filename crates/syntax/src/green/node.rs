use super::{GreenChild, GreenHead};
use crate::{GreenToken, NodeOrToken, SyntaxKind};
use std::fmt;
use text_size::TextSize;
use triomphe::{Arc, HeaderWithLength, ThinArc, UniqueArc};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GreenNode {
    data: ThinArc<GreenHead, GreenChild>,
}

impl GreenNode {
    #[inline]
    pub fn new<I>(kind: SyntaxKind, children: I) -> GreenNode
    where
        I: IntoIterator<Item = NodeOrToken<GreenNode, GreenToken>>,
        I::IntoIter: ExactSizeIterator,
    {
        let mut children = children.into_iter();
        let mut data = UniqueArc::from_header_and_uninit_slice(
            HeaderWithLength::new(
                GreenHead {
                    kind,
                    text_len: 0.into(),
                },
                children.len(),
            ),
            children.len(),
        );
        let mut slots = data.slice.iter_mut();
        let mut total_text_len = 0.into();
        while let Some((slot, node_or_token)) = slots.next().zip(children.next()) {
            match node_or_token {
                NodeOrToken::Node(node) => {
                    let text_len = node.text_len();
                    slot.write(GreenChild::Node {
                        offset: total_text_len,
                        node,
                    });
                    total_text_len += text_len;
                }
                NodeOrToken::Token(token) => {
                    let text_len = token.text_len();
                    slot.write(GreenChild::Token {
                        offset: total_text_len,
                        token,
                    });
                    total_text_len += text_len;
                }
            }
        }
        assert!(slots.next().is_none(), "incorrect children exact size iterator");
        assert!(children.next().is_none(), "incorrect children exact size iterator");
        data.header.header.text_len = total_text_len;

        // SAFETY: all data has been written to slice
        let data = unsafe { Arc::into_thin(data.assume_init_slice_with_header().shareable()) };

        GreenNode { data }
    }

    #[inline]
    pub fn kind(&self) -> SyntaxKind {
        self.data.header.header.kind
    }

    #[inline]
    pub fn text_len(&self) -> TextSize {
        self.data.header.header.text_len
    }

    #[inline]
    pub fn children(&self) -> impl Iterator<Item = NodeOrToken<&GreenNode, &GreenToken>> + Clone {
        self.data.slice.iter().map(|child| match child {
            GreenChild::Node { node, .. } => NodeOrToken::Node(node),
            GreenChild::Token { token, .. } => NodeOrToken::Token(token),
        })
    }

    #[inline]
    pub fn children_len(&self) -> usize {
        self.data.header.length
    }

    #[inline]
    pub(crate) fn slice(&self) -> &[GreenChild] {
        &self.data.slice
    }

    #[inline]
    /// Returns current level green node.
    pub(crate) fn replace_child(&self, index: usize, new_child: GreenNode) -> GreenNode {
        let mut replacement = Some(new_child);
        let children = self.data.slice.iter().enumerate().map(|(i, child)| match child {
            GreenChild::Node { .. } if i == index => replacement.take().expect("replacement has been taken").into(),
            GreenChild::Node { node, .. } => node.clone().into(),
            GreenChild::Token { token, .. } => token.clone().into(),
        });
        GreenNode::new(self.data.header.header.kind, children)
    }
}

impl fmt::Debug for GreenNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GreenNode")
            .field("kind", &self.kind())
            .field("text_len", &self.text_len())
            .field("children_len", &self.children_len())
            .finish()
    }
}

impl fmt::Display for GreenNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.slice.iter().try_for_each(|child| match child {
            GreenChild::Node { node, .. } => node.fmt(f),
            GreenChild::Token { token, .. } => token.fmt(f),
        })
    }
}
