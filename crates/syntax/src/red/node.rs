use crate::{
    AmberNode, Descendants, DescendantsWithTokens, GreenNode, GreenToken, NodeOrToken, SyntaxElement, SyntaxKind,
    SyntaxNodeChildren, SyntaxToken, TokenAtOffset, green::GreenChild,
};
use std::{fmt, ptr::NonNull, rc::Rc};
use text_size::{TextRange, TextSize};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SyntaxNode {
    pub(crate) data: Rc<NodeData>,
}

impl SyntaxNode {
    #[inline]
    pub fn new_root(green: GreenNode) -> Self {
        SyntaxNode {
            data: Rc::new(NodeData {
                range: TextRange::new(0.into(), green.text_len()),
                level: NodeLevel::Root { green },
                index: 0,
            }),
        }
    }

    #[inline]
    pub(crate) fn new(index: u32, green: &GreenNode, offset: TextSize, parent: Rc<NodeData>) -> Self {
        SyntaxNode {
            data: Rc::new(NodeData {
                range: TextRange::new(offset, offset + green.text_len()),
                level: NodeLevel::Child {
                    green: NonNull::from(green),
                    parent,
                },
                index,
            }),
        }
    }
    #[inline]
    pub(crate) fn new_child(&self, index: u32, green: &GreenNode, offset: TextSize) -> Self {
        SyntaxNode::new(index, green, self.data.range.start() + offset, Rc::clone(&self.data))
    }
    #[inline]
    pub(crate) fn new_token(&self, index: u32, green: &GreenToken, offset: TextSize) -> SyntaxToken {
        SyntaxToken::new(index, green, self.data.range.start() + offset, Rc::clone(&self.data))
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
    pub fn green(&self) -> &GreenNode {
        self.data.green()
    }

    #[inline]
    pub fn amber(&self) -> AmberNode<'_> {
        self.into()
    }

    #[inline]
    pub fn parent(&self) -> Option<SyntaxNode> {
        match &self.data.level {
            NodeLevel::Root { .. } => None,
            NodeLevel::Child { parent, .. } => Some(SyntaxNode {
                data: Rc::clone(parent),
            }),
        }
    }

    #[inline]
    pub fn ancestors(&self) -> impl Iterator<Item = SyntaxNode> {
        std::iter::successors(Some(self.clone()), SyntaxNode::parent)
    }

    #[inline]
    pub fn children(&self) -> SyntaxNodeChildren {
        SyntaxNodeChildren {
            parent: self.clone(),
            green: match &self.data.level {
                NodeLevel::Root { green } => NonNull::from(green),
                NodeLevel::Child { green, .. } => *green,
            },
            index: 0,
        }
    }

    #[inline]
    pub fn children_by_kind<F>(&self, matcher: F) -> impl DoubleEndedIterator<Item = SyntaxNode> + use<'_, F>
    where
        F: Fn(SyntaxKind) -> bool,
    {
        self.green()
            .slice()
            .iter()
            .enumerate()
            .filter_map(move |(i, child)| match child {
                GreenChild::Node { offset, node } if matcher(node.kind()) => {
                    Some(self.new_child(i as u32, node, *offset))
                }
                _ => None,
            })
    }

    #[inline]
    pub fn tokens_by_kind<F>(&self, matcher: F) -> impl DoubleEndedIterator<Item = SyntaxToken> + use<'_, F>
    where
        F: Fn(SyntaxKind) -> bool,
    {
        self.green()
            .slice()
            .iter()
            .enumerate()
            .filter_map(move |(i, child)| match child {
                GreenChild::Token { offset, token } if matcher(token.kind()) => {
                    Some(self.new_token(i as u32, token, *offset))
                }
                _ => None,
            })
    }

    #[inline]
    pub fn children_with_tokens(&self) -> impl DoubleEndedIterator<Item = SyntaxElement> {
        self.green().slice().iter().enumerate().map(|(i, child)| match child {
            GreenChild::Node { offset, node } => self.new_child(i as u32, node, *offset).into(),
            GreenChild::Token { offset, token } => self.new_token(i as u32, token, *offset).into(),
        })
    }

    #[inline]
    pub fn has_child_or_token_by_kind<F>(&self, matcher: F) -> bool
    where
        F: Fn(SyntaxKind) -> bool,
    {
        self.green().slice().iter().any(|child| match child {
            GreenChild::Node { node, .. } => matcher(node.kind()),
            GreenChild::Token { token, .. } => matcher(token.kind()),
        })
    }

    #[inline]
    pub fn first_child(&self) -> Option<SyntaxNode> {
        self.children().next()
    }

    #[inline]
    pub fn first_child_or_token(&self) -> Option<SyntaxElement> {
        self.green().slice().first().map(|child| match child {
            GreenChild::Node { offset, node } => self.new_child(0, node, *offset).into(),
            GreenChild::Token { offset, token } => self.new_token(0, token, *offset).into(),
        })
    }

    #[inline]
    pub fn last_child(&self) -> Option<SyntaxNode> {
        self.green()
            .slice()
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, child)| match child {
                GreenChild::Node { offset, node } => Some(self.new_child(i as u32, node, *offset)),
                _ => None,
            })
    }

    #[inline]
    pub fn last_child_or_token(&self) -> Option<SyntaxElement> {
        let slice = self.green().slice();
        slice.last().map(|child| match child {
            GreenChild::Node { offset, node } => self.new_child(slice.len() as u32 - 1, node, *offset).into(),
            GreenChild::Token { offset, token } => self.new_token(slice.len() as u32 - 1, token, *offset).into(),
        })
    }

    #[inline]
    pub fn next_sibling_or_token(&self) -> Option<SyntaxElement> {
        match &self.data.level {
            NodeLevel::Root { .. } => None,
            NodeLevel::Child { parent, .. } => parent.next_child_or_token(self.data.index),
        }
    }

    #[inline]
    pub fn next_siblings(&self) -> impl Iterator<Item = SyntaxNode> {
        match &self.data.level {
            NodeLevel::Root { .. } => None,
            NodeLevel::Child { parent, .. } => Some(parent),
        }
        .into_iter()
        .flat_map(|parent| parent.next_children(self.data.index))
    }

    #[inline]
    pub fn next_siblings_with_tokens(&self) -> impl Iterator<Item = SyntaxElement> {
        match &self.data.level {
            NodeLevel::Root { .. } => None,
            NodeLevel::Child { parent, .. } => Some(parent),
        }
        .into_iter()
        .flat_map(|parent| parent.next_children_with_tokens(self.data.index))
    }

    #[inline]
    pub fn next_consecutive_tokens(&self) -> impl Iterator<Item = SyntaxToken> {
        match &self.data.level {
            NodeLevel::Root { .. } => None,
            NodeLevel::Child { parent, .. } => Some(parent),
        }
        .into_iter()
        .flat_map(|parent| parent.next_consecutive_tokens(self.data.index))
    }

    #[inline]
    pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement> {
        match &self.data.level {
            NodeLevel::Root { .. } => None,
            NodeLevel::Child { parent, .. } => parent.prev_child_or_token(self.data.index),
        }
    }

    #[inline]
    pub fn prev_siblings(&self) -> impl Iterator<Item = SyntaxNode> {
        match &self.data.level {
            NodeLevel::Root { .. } => None,
            NodeLevel::Child { parent, .. } => Some(parent),
        }
        .into_iter()
        .flat_map(|parent| parent.prev_children(self.data.index))
    }

    #[inline]
    pub fn prev_siblings_with_tokens(&self) -> impl Iterator<Item = SyntaxElement> {
        match &self.data.level {
            NodeLevel::Root { .. } => None,
            NodeLevel::Child { parent, .. } => Some(parent),
        }
        .into_iter()
        .flat_map(|parent| parent.prev_children_with_tokens(self.data.index))
    }

    #[inline]
    pub fn prev_consecutive_tokens(&self) -> impl Iterator<Item = SyntaxToken> {
        match &self.data.level {
            NodeLevel::Root { .. } => None,
            NodeLevel::Child { parent, .. } => Some(parent),
        }
        .into_iter()
        .flat_map(|parent| parent.prev_consecutive_tokens(self.data.index))
    }

    #[inline]
    pub fn descendants(&self) -> Descendants {
        Descendants::new(self.clone())
    }

    #[inline]
    pub fn descendants_with_tokens(&self) -> DescendantsWithTokens {
        DescendantsWithTokens::new(self.clone())
    }

    /// Find a token in the subtree corresponding to this node, which covers the offset.
    // Copied from rowan with modification.
    pub fn token_at_offset(&self, offset: TextSize) -> TokenAtOffset {
        let range = self.data.range;
        let relative_offset = offset - range.start();
        let mut children = self
            .green()
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
    pub fn child_or_token_at_range(&self, range: TextRange) -> Option<SyntaxElement> {
        if !self.data.range.contains_range(range) {
            return None;
        }
        let relative_range = range - self.data.range.start();
        let slice = self.green().slice();
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
                    Some(self.new_child(i as u32, node, *offset).into())
                } else {
                    None
                }
            }
            GreenChild::Token { offset, token } => {
                if TextRange::new(*offset, offset + token.text_len()).contains_range(relative_range) {
                    Some(self.new_token(i as u32, token, *offset).into())
                } else {
                    None
                }
            }
        })
    }

    #[inline]
    /// Returns new *root* green node with replaced node.
    pub fn replace_with(&self, replacement: GreenNode) -> GreenNode {
        match &self.data.level {
            NodeLevel::Root { .. } => replacement,
            NodeLevel::Child { parent, .. } => {
                let parent = SyntaxNode {
                    data: Rc::clone(parent),
                };
                let new_parent = parent.green().replace_child(self.data.index as usize, replacement);
                parent.replace_with(new_parent)
            }
        }
    }
}

impl fmt::Debug for SyntaxNode {
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

impl fmt::Display for SyntaxNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.green().fmt(f)
    }
}

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct NodeData {
    range: TextRange,
    level: NodeLevel,
    index: u32,
}

impl NodeData {
    #[inline]
    fn green(&self) -> &GreenNode {
        match &self.level {
            NodeLevel::Root { green } => green,
            NodeLevel::Child { green, .. } => unsafe { green.as_ref() },
        }
    }

    #[inline]
    pub fn next_children(self: &Rc<Self>, index: u32) -> impl Iterator<Item = SyntaxNode> {
        self.green()
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
    pub fn next_child_or_token(self: &Rc<Self>, index: u32) -> Option<SyntaxElement> {
        let i = index + 1;
        self.green().slice().get(i as usize).map(|child| match child {
            GreenChild::Node { offset, node } => {
                SyntaxNode::new(i, node, self.range.start() + offset, Rc::clone(self)).into()
            }
            GreenChild::Token { offset, token } => {
                SyntaxToken::new(i, token, self.range.start() + offset, Rc::clone(self)).into()
            }
        })
    }

    #[inline]
    pub fn next_children_with_tokens(self: &Rc<Self>, index: u32) -> impl Iterator<Item = SyntaxElement> {
        self.green()
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
    pub fn next_consecutive_tokens(self: &Rc<Self>, index: u32) -> impl Iterator<Item = SyntaxToken> {
        self.green()
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
    pub fn prev_children(self: &Rc<Self>, index: u32) -> impl Iterator<Item = SyntaxNode> {
        let slice = self.green().slice();
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
    pub fn prev_child_or_token(self: &Rc<Self>, index: u32) -> Option<SyntaxElement> {
        let i = index.checked_sub(1)?;
        self.green().slice().get(i as usize).map(|child| match child {
            GreenChild::Node { offset, node } => {
                SyntaxNode::new(i, node, self.range.start() + offset, Rc::clone(self)).into()
            }
            GreenChild::Token { offset, token } => {
                SyntaxToken::new(i, token, self.range.start() + offset, Rc::clone(self)).into()
            }
        })
    }

    #[inline]
    pub fn prev_children_with_tokens(self: &Rc<Self>, index: u32) -> impl Iterator<Item = SyntaxElement> {
        let slice = self.green().slice();
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
    pub fn prev_consecutive_tokens(self: &Rc<Self>, index: u32) -> impl Iterator<Item = SyntaxToken> {
        let slice = self.green().slice();
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

#[derive(PartialEq, Eq, Hash)]
enum NodeLevel {
    Root {
        green: GreenNode,
    },
    Child {
        green: NonNull<GreenNode>,
        parent: Rc<NodeData>,
    },
}
