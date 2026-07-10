use super::{GreenChild, GreenHead};
use crate::{GreenToken, NodeOrToken, SyntaxKind};
use servo_arc::{ThinArc, UniqueArc};
use std::{fmt, hash};
use text_size::{TextRange, TextSize};

#[derive(Clone, PartialEq, Eq)]
/// Node in the green syntax tree.
pub struct GreenNode {
    data: ThinArc<GreenHead, GreenChild>,
}

impl GreenNode {
    #[inline]
    /// Create specified kind of green node with given children.
    pub fn new<I>(kind: SyntaxKind, children: I) -> GreenNode
    where
        I: IntoIterator<Item = NodeOrToken<GreenNode, GreenToken>>,
        I::IntoIter: ExactSizeIterator,
    {
        let mut total_text_len = 0.into();
        let mut data = UniqueArc::from_header_and_iter(
            GreenHead {
                kind,
                text_len: 0.into(),
            },
            children.into_iter().map(|node_or_token| match node_or_token {
                NodeOrToken::Node(node) => {
                    let text_len = node.text_len();
                    let child = GreenChild::Node {
                        offset: total_text_len,
                        node,
                    };
                    total_text_len += text_len;
                    child
                }
                NodeOrToken::Token(token) => {
                    let text_len = token.text_len();
                    let child = GreenChild::Token {
                        offset: total_text_len,
                        token,
                    };
                    total_text_len += text_len;
                    child
                }
            }),
        );
        data.header.text_len = total_text_len;

        GreenNode { data: data.shareable() }
    }

    #[inline]
    /// Kind of this node.
    pub fn kind(&self) -> SyntaxKind {
        self.data.header.kind
    }

    #[inline]
    /// Iterator over the child nodes and tokens of this node.
    pub fn children(&self) -> impl Iterator<Item = NodeOrToken<&GreenNode, &GreenToken>> + Clone {
        self.data.slice().iter().map(|child| match child {
            GreenChild::Node { node, .. } => NodeOrToken::Node(node),
            GreenChild::Token { token, .. } => NodeOrToken::Token(token),
        })
    }

    #[inline]
    /// Number of child nodes and tokens of this node.
    pub fn children_len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    /// Total length of text that this node covers.
    pub(crate) fn text_len(&self) -> TextSize {
        self.data.header.text_len
    }

    #[inline]
    pub(crate) fn slice(&self) -> &[GreenChild] {
        self.data.slice()
    }

    #[inline]
    /// Find a child node that intersects with the given range.
    pub(crate) fn child_at_range(&self, relative_range: TextRange) -> Option<(&GreenNode, TextSize, usize)> {
        let slice = self.data.slice();
        let i = slice
            .binary_search_by(|child| match child {
                GreenChild::Node { offset, node } => {
                    TextRange::new(*offset, offset + node.text_len()).ordering(relative_range)
                }
                GreenChild::Token { offset, token } => {
                    TextRange::new(*offset, offset + token.text_len()).ordering(relative_range)
                }
            })
            .unwrap_or_else(|i| i.saturating_sub(1)); // not sure why but rowan does it
        slice.get(i).and_then(|child| match child {
            GreenChild::Node { offset, node } => {
                if TextRange::new(*offset, offset + node.text_len()).contains_range(relative_range) {
                    Some((node, *offset, i))
                } else {
                    None
                }
            }
            GreenChild::Token { .. } => None,
        })
    }

    #[inline]
    /// Returns current level green node.
    pub(crate) fn replace_child(&self, index: usize, new_child: GreenNode) -> GreenNode {
        let mut replacement = Some(new_child);
        let children = self.data.slice().iter().enumerate().map(|(i, child)| match child {
            GreenChild::Node { .. } if i == index => replacement.take().expect("replacement has been taken").into(),
            GreenChild::Node { node, .. } => node.clone().into(),
            GreenChild::Token { token, .. } => token.clone().into(),
        });
        GreenNode::new(self.data.header.kind, children)
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
        self.data.slice().iter().try_for_each(|child| match child {
            GreenChild::Node { node, .. } => node.fmt(f),
            GreenChild::Token { token, .. } => token.fmt(f),
        })
    }
}

impl hash::Hash for GreenNode {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.data.header.hash(state);
        self.data.slice().hash(state);
    }
}
