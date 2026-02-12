use super::{GreenChild, GreenHead};
use crate::{GreenToken, NodeOrToken, SyntaxKind};
use std::fmt;
use text_size::TextSize;
use triomphe::{Arc, ThinArc};

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
        let mut text_len: TextSize = 0.into();
        let children = children.into_iter().map(|node_or_token| match node_or_token {
            NodeOrToken::Node(node) => {
                let offset = text_len;
                text_len += node.text_len();
                GreenChild::Node { offset, node }
            }
            NodeOrToken::Token(token) => {
                let offset = text_len;
                text_len += token.text_len();
                GreenChild::Token { offset, token }
            }
        });

        let data = ThinArc::from_header_and_iter(
            GreenHead {
                kind,
                text_len: 0.into(),
            },
            children,
        );
        let data = {
            let mut data = Arc::from_thin(data);
            Arc::get_mut(&mut data).unwrap().header.header.text_len = text_len;
            Arc::into_thin(data)
        };

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
    pub(crate) fn slice(&self) -> &[GreenChild] {
        &self.data.slice
    }

    #[inline]
    /// Returns current level green node.
    pub fn replace_child(&self, index: usize, new_child: GreenNode) -> GreenNode {
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
            .field("children_len", &self.slice().len())
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
