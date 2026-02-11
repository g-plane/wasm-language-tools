use crate::{
    Descendants, DescendantsWithTokens, GreenNode, GreenToken, NodeOrToken, SyntaxElement, SyntaxElementChildren,
    SyntaxKind, SyntaxNodeChildren, SyntaxToken, TokenAtOffset, green::GreenChild,
};
use std::{fmt, ptr::NonNull, rc::Rc};
use text_size::{TextRange, TextSize};

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct NodeData {
    range: TextRange,
    level: NodeLevel,
    index: u32,
}
impl NodeData {
    #[inline]
    pub(crate) fn range(&self) -> TextRange {
        self.range
    }
    #[inline]
    pub(crate) fn green(&self) -> &GreenNode {
        match &self.level {
            NodeLevel::Root { green } => green,
            NodeLevel::Child { green, .. } => unsafe { green.as_ref() },
        }
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
    pub fn children_with_tokens(&self) -> SyntaxElementChildren {
        SyntaxElementChildren {
            parent: self.clone(),
            green: match &self.data.level {
                NodeLevel::Root { green } => NonNull::from(green),
                NodeLevel::Child { green, .. } => *green,
            },
            index: 0,
        }
    }

    #[inline]
    pub fn first_child(&self) -> Option<SyntaxNode> {
        self.children().next()
    }

    #[inline]
    pub fn first_child_by_kind<F>(&self, matcher: F) -> Option<SyntaxNode>
    where
        F: Fn(SyntaxKind) -> bool,
    {
        self.green()
            .slice()
            .iter()
            .enumerate()
            .find_map(|(i, child)| match child {
                GreenChild::Node { offset, node } if matcher(node.kind()) => {
                    Some(self.new_child(i as u32, node, *offset))
                }
                _ => None,
            })
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
    pub fn last_child_by_kind<F>(&self, matcher: F) -> Option<SyntaxNode>
    where
        F: Fn(SyntaxKind) -> bool,
    {
        self.green()
            .slice()
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, child)| match child {
                GreenChild::Node { offset, node } if matcher(node.kind()) => {
                    Some(self.new_child(i as u32, node, *offset))
                }
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
    pub fn next_sibling(&self) -> Option<SyntaxNode> {
        self.next_siblings().nth(1)
    }

    #[inline]
    pub fn next_sibling_or_token(&self) -> Option<SyntaxElement> {
        let i = self.data.index + 1;
        let parent = self.parent()?;
        parent.green().slice().get(i as usize).map(|child| match child {
            GreenChild::Node { offset, node } => parent.new_child(i, node, *offset).into(),
            GreenChild::Token { offset, token } => parent.new_token(i, token, *offset).into(),
        })
    }

    #[inline]
    /// Including current node.
    pub fn next_siblings(&self) -> impl Iterator<Item = SyntaxNode> {
        match &self.data.level {
            NodeLevel::Root { .. } => None,
            NodeLevel::Child { parent, .. } => Some(parent),
        }
        .into_iter()
        .flat_map(|parent| {
            parent
                .green()
                .slice()
                .iter()
                .enumerate()
                .skip(self.data.index as usize)
                .filter_map(|(i, child)| match child {
                    GreenChild::Node { offset, node } => Some(SyntaxNode::new(
                        i as u32,
                        node,
                        parent.range.start() + offset,
                        Rc::clone(parent),
                    )),
                    _ => None,
                })
        })
    }

    #[inline]
    /// Including current node.
    pub fn next_siblings_with_tokens(&self) -> impl Iterator<Item = SyntaxElement> {
        match &self.data.level {
            NodeLevel::Root { .. } => None,
            NodeLevel::Child { parent, .. } => Some(parent),
        }
        .into_iter()
        .flat_map(|parent| {
            parent
                .green()
                .slice()
                .iter()
                .enumerate()
                .skip(self.data.index as usize)
                .map(|(i, child)| match child {
                    GreenChild::Node { offset, node } => {
                        SyntaxNode::new(i as u32, node, parent.range.start() + offset, Rc::clone(parent)).into()
                    }
                    GreenChild::Token { offset, token } => {
                        SyntaxToken::new(i as u32, token, parent.range.start() + offset, Rc::clone(parent)).into()
                    }
                })
        })
    }

    #[inline]
    pub fn prev_sibling(&self) -> Option<SyntaxNode> {
        self.prev_siblings().nth(1)
    }

    #[inline]
    pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement> {
        let i = self.data.index.checked_sub(1)?;
        let parent = self.parent()?;
        parent.green().slice().get(i as usize).map(|child| match child {
            GreenChild::Node { offset, node } => parent.new_child(i, node, *offset).into(),
            GreenChild::Token { offset, token } => parent.new_token(i, token, *offset).into(),
        })
    }

    #[inline]
    /// Including current node.
    pub fn prev_siblings(&self) -> impl Iterator<Item = SyntaxNode> {
        match &self.data.level {
            NodeLevel::Root { .. } => None,
            NodeLevel::Child { parent, .. } => Some(parent),
        }
        .into_iter()
        .flat_map(|parent| {
            let slice = parent.green().slice();
            slice
                .iter()
                .enumerate()
                .rev()
                .skip(slice.len() - self.data.index as usize - 1)
                .filter_map(|(i, child)| match child {
                    GreenChild::Node { offset, node } => Some(SyntaxNode::new(
                        i as u32,
                        node,
                        parent.range.start() + offset,
                        Rc::clone(parent),
                    )),
                    _ => None,
                })
        })
    }

    #[inline]
    /// Including current node.
    pub fn prev_siblings_with_tokens(&self) -> impl Iterator<Item = SyntaxElement> {
        match &self.data.level {
            NodeLevel::Root { .. } => None,
            NodeLevel::Child { parent, .. } => Some(parent),
        }
        .into_iter()
        .flat_map(|parent| {
            let slice = parent.green().slice();
            slice
                .iter()
                .enumerate()
                .rev()
                .skip(slice.len() - self.data.index as usize - 1)
                .map(|(i, child)| match child {
                    GreenChild::Node { offset, node } => {
                        SyntaxNode::new(i as u32, node, parent.range.start() + offset, Rc::clone(parent)).into()
                    }
                    GreenChild::Token { offset, token } => {
                        SyntaxToken::new(i as u32, token, parent.range.start() + offset, Rc::clone(parent)).into()
                    }
                })
        })
    }

    #[inline]
    pub fn descendants(&self) -> Descendants {
        Descendants::new(self.clone())
    }

    #[inline]
    pub fn descendants_with_tokens(&self) -> DescendantsWithTokens {
        DescendantsWithTokens::new(self.clone().into())
    }

    /// Find a token in the subtree corresponding to this node, which covers the offset.
    // Copied from rowan with modification.
    pub fn token_at_offset(&self, offset: TextSize) -> TokenAtOffset {
        let range = self.data.range;
        if !range.contains(offset) {
            return TokenAtOffset::None;
        }
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
        slice
            .binary_search_by(|child| match child {
                GreenChild::Node { offset, node } => {
                    TextRange::new(*offset, offset + node.text_len()).ordering(relative_range)
                }
                GreenChild::Token { offset, token } => {
                    TextRange::new(*offset, offset + token.text_len()).ordering(relative_range)
                }
            })
            .ok()
            .and_then(|i| match slice.get(i)? {
                GreenChild::Node { offset, node } => Some(self.new_child(i as u32, node, *offset).into()),
                GreenChild::Token { offset, token } => Some(self.new_token(i as u32, token, *offset).into()),
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
