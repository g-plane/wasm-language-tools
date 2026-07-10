use super::{element::SyntaxElement, traversal::Descendants};
use crate::{
    AmberNode, GreenNode, GreenToken, NodeOrToken, SyntaxKind, SyntaxKindMatch, SyntaxNodeChildren, SyntaxToken,
    TokenAtOffset, green::GreenChild,
};
use std::{fmt, rc::Rc};
use text_size::{TextRange, TextSize};

#[derive(Clone, PartialEq, Eq, Hash)]
/// Node in the red syntax tree.
pub struct SyntaxNode<'a> {
    pub(crate) data: Rc<NodeData<'a>>,
}

impl<'a> SyntaxNode<'a> {
    #[inline]
    /// Build a new syntax tree on top of a green tree.
    pub fn new_root(green: &'a GreenNode) -> Self {
        SyntaxNode {
            data: Rc::new(NodeData {
                range: TextRange::new(0.into(), green.text_len()),
                green,
                parent: None,
                index: 0,
            }),
        }
    }

    #[inline]
    pub(crate) fn new(index: u32, green: &'a GreenNode, offset: TextSize, parent: Rc<NodeData<'a>>) -> Self {
        SyntaxNode {
            data: Rc::new(NodeData {
                range: TextRange::new(offset, offset + green.text_len()),
                green,
                parent: Some(parent),
                index,
            }),
        }
    }
    #[inline]
    pub(crate) fn new_child(&self, index: u32, green: &'a GreenNode, offset: TextSize) -> SyntaxNode<'a> {
        SyntaxNode::new(index, green, self.data.range.start() + offset, Rc::clone(&self.data))
    }
    #[inline]
    pub(crate) fn new_token(&self, index: u32, green: &'a GreenToken, offset: TextSize) -> SyntaxToken<'a> {
        SyntaxToken::new(index, green, self.data.range.start() + offset, Rc::clone(&self.data))
    }

    #[inline]
    /// Kind of this node.
    pub fn kind(&self) -> SyntaxKind {
        self.data.green.kind()
    }

    #[inline]
    /// The range that this node covers in the original text.
    pub fn text_range(&self) -> TextRange {
        self.data.range
    }

    #[inline]
    /// The underlying green node of this red node.
    pub fn green(&self) -> &'a GreenNode {
        self.data.green
    }

    #[inline]
    /// The corresponding amber node of this red node.
    pub fn amber(&self) -> AmberNode<'a> {
        self.into()
    }

    #[inline]
    /// Parent of this node. It returns `None` if this node is the root.
    pub fn parent(&self) -> Option<SyntaxNode<'a>> {
        self.data.parent.as_ref().map(|parent| SyntaxNode {
            data: Rc::clone(parent),
        })
    }

    #[inline]
    /// Iterator along the chain of parents of this node.
    pub fn ancestors(&self) -> impl Iterator<Item = SyntaxNode<'a>> {
        std::iter::successors(Some(self.clone()), SyntaxNode::parent)
    }

    #[inline]
    /// Iterator over the child nodes of this node.
    ///
    /// If you want to iterate over both nodes and tokens, use [`children_with_tokens`](Self::children_with_tokens) instead.
    ///
    /// Though you can filter specific kinds of children on this iterator manually,
    /// it is more efficient to use [`children_by_kind`](Self::children_by_kind) instead.
    pub fn children(&self) -> SyntaxNodeChildren<'a> {
        SyntaxNodeChildren {
            parent: self.clone(),
            green: self.data.green,
            index: 0,
        }
    }

    #[inline]
    /// Iterator over specific kinds of child nodes of this node.
    /// This is more efficient than filtering with [`children`](Self::children) manually.
    pub fn children_by_kind<M>(&self, matcher: M) -> impl DoubleEndedIterator<Item = SyntaxNode<'a>> + use<'_, 'a, M>
    where
        M: SyntaxKindMatch,
    {
        self.data
            .green
            .slice()
            .iter()
            .enumerate()
            .filter_map(move |(i, child)| match child {
                GreenChild::Node { offset, node } if matcher.matches(node.kind()) => {
                    Some(self.new_child(i as u32, node, *offset))
                }
                _ => None,
            })
    }

    #[inline]
    /// Iterator over specific kinds of child tokens of this node.
    pub fn tokens_by_kind<M>(&self, matcher: M) -> impl DoubleEndedIterator<Item = SyntaxToken<'a>> + use<'_, 'a, M>
    where
        M: SyntaxKindMatch,
    {
        self.data
            .green
            .slice()
            .iter()
            .enumerate()
            .filter_map(move |(i, child)| match child {
                GreenChild::Token { offset, token } if matcher.matches(token.kind()) => {
                    Some(self.new_token(i as u32, token, *offset))
                }
                _ => None,
            })
    }

    #[inline]
    /// Iterator over the child nodes and tokens of this node.
    pub fn children_with_tokens(&self) -> impl DoubleEndedIterator<Item = SyntaxElement<'a>> {
        self.data
            .green
            .slice()
            .iter()
            .enumerate()
            .map(|(i, child)| match child {
                GreenChild::Node { offset, node } => self.new_child(i as u32, node, *offset).into(),
                GreenChild::Token { offset, token } => self.new_token(i as u32, token, *offset).into(),
            })
    }

    #[inline]
    /// Check if this node has specific kinds of child nodes or tokens.
    ///
    /// This is an efficient alternative to `node.children_with_tokens().any(...)`
    /// since it won't create any nodes or tokens.
    pub fn has_child_or_token_by_kind<M>(&self, matcher: M) -> bool
    where
        M: SyntaxKindMatch,
    {
        self.data.green.slice().iter().any(|child| match child {
            GreenChild::Node { node, .. } => matcher.matches(node.kind()),
            GreenChild::Token { token, .. } => matcher.matches(token.kind()),
        })
    }

    #[inline]
    /// Nodes that come immediately after this node.
    ///
    /// If you want to iterate over both nodes and tokens, use [`next_siblings_with_tokens`](Self::next_siblings_with_tokens) instead.
    ///
    /// Unlike rowan, the iterator doesn't contain the current node itself.
    pub fn next_siblings(&self) -> impl Iterator<Item = SyntaxNode<'a>> {
        self.data
            .parent
            .as_ref()
            .into_iter()
            .flat_map(|parent| parent.next_children(self.data.index))
    }

    #[inline]
    /// Node or token that comes immediately after this node.
    pub fn next_sibling_or_token(&self) -> Option<SyntaxElement<'a>> {
        self.data
            .parent
            .as_ref()
            .and_then(|parent| parent.next_child_or_token(self.data.index))
    }

    #[inline]
    /// Nodes and tokens that come immediately after this node.
    ///
    /// Unlike rowan, the iterator doesn't contain the current node itself.
    pub fn next_siblings_with_tokens(&self) -> impl Iterator<Item = SyntaxElement<'a>> {
        self.data
            .parent
            .as_ref()
            .into_iter()
            .flat_map(|parent| parent.next_children_with_tokens(self.data.index))
    }

    #[inline]
    /// Consecutive tokens sequence that come immediately after this node without any nodes in between.
    pub fn next_consecutive_tokens(&self) -> impl Iterator<Item = SyntaxToken<'a>> {
        self.data
            .parent
            .as_ref()
            .into_iter()
            .flat_map(|parent| parent.next_consecutive_tokens(self.data.index))
    }

    #[inline]
    /// Nodes that come immediately before this node.
    ///
    /// If you want to iterate over both nodes and tokens, use [`prev_siblings_with_tokens`](Self::prev_siblings_with_tokens) instead.
    ///
    /// Unlike rowan, the iterator doesn't contain the current node itself.
    pub fn prev_siblings(&self) -> impl Iterator<Item = SyntaxNode<'a>> {
        self.data
            .parent
            .as_ref()
            .into_iter()
            .flat_map(|parent| parent.prev_children(self.data.index))
    }

    #[inline]
    /// Node or token that comes immediately before this node.
    pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement<'a>> {
        self.data
            .parent
            .as_ref()
            .and_then(|parent| parent.prev_child_or_token(self.data.index))
    }

    #[inline]
    /// Nodes and tokens that come immediately before this node.
    ///
    /// Unlike rowan, the iterator doesn't contain the current node itself.
    pub fn prev_siblings_with_tokens(&self) -> impl Iterator<Item = SyntaxElement<'a>> {
        self.data
            .parent
            .as_ref()
            .into_iter()
            .flat_map(|parent| parent.prev_children_with_tokens(self.data.index))
    }

    #[inline]
    /// Consecutive tokens sequence that come immediately before this node without any nodes in between.
    pub fn prev_consecutive_tokens(&self) -> impl Iterator<Item = SyntaxToken<'a>> {
        self.data
            .parent
            .as_ref()
            .into_iter()
            .flat_map(|parent| parent.prev_consecutive_tokens(self.data.index))
    }

    #[inline]
    /// Iterator over all nodes in the subtree, including this node itself.
    pub fn descendants(&self) -> impl Iterator<Item = SyntaxNode<'a>> {
        Descendants::new(self.clone())
    }

    /// Find a token in the subtree corresponding to this node, which covers the offset.
    // Copied from rowan with modification.
    pub fn token_at_offset(&self, offset: TextSize) -> TokenAtOffset<'a> {
        let range = self.data.range;
        let relative_offset = offset - range.start();
        let mut children = self
            .data
            .green
            .slice()
            .iter()
            .enumerate()
            .filter(|(_, child)| {
                let (start, end) = match child {
                    GreenChild::Node { offset, node } => (*offset, offset + node.text_len()),
                    GreenChild::Token { offset, token } => (*offset, offset + token.text_len()),
                };
                start <= relative_offset && relative_offset <= end
            })
            .map(|(i, child)| match child {
                GreenChild::Node { offset, node } => NodeOrToken::Node(self.new_child(i as u32, node, *offset)),
                GreenChild::Token { offset, token } => NodeOrToken::Token(self.new_token(i as u32, token, *offset)),
            });

        let Some(left) = children.next() else {
            return TokenAtOffset::None;
        };
        let right = children.next();
        if let Some(right) = right {
            match (left.token_at_offset(offset), right.token_at_offset(offset)) {
                (TokenAtOffset::Single(left), TokenAtOffset::Single(right)) => TokenAtOffset::Between(left, right),
                _ => TokenAtOffset::None,
            }
        } else {
            left.token_at_offset(offset)
        }
    }

    #[inline]
    /// Find a child node that intersects with the given range.
    pub fn child_at_range(&self, range: TextRange) -> Option<SyntaxNode<'a>> {
        if !self.data.range.contains_range(range) {
            return None;
        }
        let relative_range = range - self.data.range.start();
        self.data
            .green
            .child_at_range(relative_range)
            .map(|(node, offset, index)| self.new_child(index as u32, node, offset))
    }

    #[inline]
    /// Replace this node with new green node. It returns new *root* green node with replaced node.
    pub fn replace_with(&self, replacement: GreenNode) -> GreenNode {
        if let Some(parent) = &self.data.parent {
            let parent = SyntaxNode {
                data: Rc::clone(parent),
            };
            let new_parent = parent.data.green.replace_child(self.data.index as usize, replacement);
            parent.replace_with(new_parent)
        } else {
            replacement
        }
    }
}

impl fmt::Debug for SyntaxNode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_print_node(self, f, 0)
    }
}
fn debug_print_node(node: &SyntaxNode, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
    writeln!(f, "{}{:?}@{:?}", "  ".repeat(level), node.kind(), node.text_range())?;
    node.children_with_tokens()
        .try_for_each(|node_or_token| match node_or_token {
            NodeOrToken::Node(node) => debug_print_node(&node, f, level + 1),
            NodeOrToken::Token(token) => writeln!(f, "{}{token:?}", "  ".repeat(level + 1)),
        })
}

impl fmt::Display for SyntaxNode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.green.fmt(f)
    }
}

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct NodeData<'a> {
    green: &'a GreenNode,
    range: TextRange,
    parent: Option<Rc<NodeData<'a>>>,
    index: u32,
}

impl<'a> NodeData<'a> {
    #[inline]
    pub fn next_children(self: &Rc<Self>, index: u32) -> impl Iterator<Item = SyntaxNode<'a>> {
        self.green
            .slice()
            .iter()
            .enumerate()
            .skip(index as usize + 1)
            .filter_map(|(i, child)| match child {
                GreenChild::Node { offset, node } => Some(SyntaxNode::new(
                    i as u32,
                    node,
                    self.range.start() + offset,
                    Rc::clone(self),
                )),
                _ => None,
            })
    }

    #[inline]
    pub fn next_child_or_token(self: &Rc<Self>, index: u32) -> Option<SyntaxElement<'a>> {
        let i = index + 1;
        self.green.slice().get(i as usize).map(|child| match child {
            GreenChild::Node { offset, node } => {
                SyntaxNode::new(i, node, self.range.start() + offset, Rc::clone(self)).into()
            }
            GreenChild::Token { offset, token } => {
                SyntaxToken::new(i, token, self.range.start() + offset, Rc::clone(self)).into()
            }
        })
    }

    #[inline]
    pub fn next_children_with_tokens(self: &Rc<Self>, index: u32) -> impl Iterator<Item = SyntaxElement<'a>> {
        self.green
            .slice()
            .iter()
            .enumerate()
            .skip(index as usize + 1)
            .map(|(i, child)| match child {
                GreenChild::Node { offset, node } => {
                    SyntaxNode::new(i as u32, node, self.range.start() + offset, Rc::clone(self)).into()
                }
                GreenChild::Token { offset, token } => {
                    SyntaxToken::new(i as u32, token, self.range.start() + offset, Rc::clone(self)).into()
                }
            })
    }

    #[inline]
    pub fn next_consecutive_tokens(self: &Rc<Self>, index: u32) -> impl Iterator<Item = SyntaxToken<'a>> {
        self.green
            .slice()
            .iter()
            .enumerate()
            .skip(index as usize + 1)
            .map_while(|(i, child)| match child {
                GreenChild::Node { .. } => None,
                GreenChild::Token { offset, token } => Some(SyntaxToken::new(
                    i as u32,
                    token,
                    self.range.start() + offset,
                    Rc::clone(self),
                )),
            })
    }

    #[inline]
    pub fn prev_children(self: &Rc<Self>, index: u32) -> impl Iterator<Item = SyntaxNode<'a>> {
        let slice = self.green.slice();
        slice
            .iter()
            .enumerate()
            .rev()
            .skip(slice.len() - index as usize)
            .filter_map(|(i, child)| match child {
                GreenChild::Node { offset, node } => Some(SyntaxNode::new(
                    i as u32,
                    node,
                    self.range.start() + offset,
                    Rc::clone(self),
                )),
                _ => None,
            })
    }

    #[inline]
    pub fn prev_child_or_token(self: &Rc<Self>, index: u32) -> Option<SyntaxElement<'a>> {
        let i = index.checked_sub(1)?;
        self.green.slice().get(i as usize).map(|child| match child {
            GreenChild::Node { offset, node } => {
                SyntaxNode::new(i, node, self.range.start() + offset, Rc::clone(self)).into()
            }
            GreenChild::Token { offset, token } => {
                SyntaxToken::new(i, token, self.range.start() + offset, Rc::clone(self)).into()
            }
        })
    }

    #[inline]
    pub fn prev_children_with_tokens(self: &Rc<Self>, index: u32) -> impl Iterator<Item = SyntaxElement<'a>> {
        let slice = self.green.slice();
        slice
            .iter()
            .enumerate()
            .rev()
            .skip(slice.len() - index as usize)
            .map(|(i, child)| match child {
                GreenChild::Node { offset, node } => {
                    SyntaxNode::new(i as u32, node, self.range.start() + offset, Rc::clone(self)).into()
                }
                GreenChild::Token { offset, token } => {
                    SyntaxToken::new(i as u32, token, self.range.start() + offset, Rc::clone(self)).into()
                }
            })
    }

    #[inline]
    pub fn prev_consecutive_tokens(self: &Rc<Self>, index: u32) -> impl Iterator<Item = SyntaxToken<'a>> {
        let slice = self.green.slice();
        slice
            .iter()
            .enumerate()
            .rev()
            .skip(slice.len() - index as usize)
            .map_while(|(i, child)| match child {
                GreenChild::Node { .. } => None,
                GreenChild::Token { offset, token } => Some(SyntaxToken::new(
                    i as u32,
                    token,
                    self.range.start() + offset,
                    Rc::clone(self),
                )),
            })
    }
}
