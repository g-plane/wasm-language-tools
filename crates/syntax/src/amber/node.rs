use crate::{AmberToken, GreenNode, NodeOrToken, SyntaxKind, SyntaxNode, SyntaxNodePtr, green::GreenChild};
use text_size::{TextRange, TextSize};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
/// `AmberNode` is a lightweight version of [`SyntaxNode`](crate::SyntaxNode) that doesn't allocate on the heap.
/// It's pretty cheaper than `SyntaxNode`, but you can't visit parent and siblings.
pub struct AmberNode<'a> {
    green: &'a GreenNode,
    range: TextRange,
}

impl<'a> AmberNode<'a> {
    #[inline]
    pub(crate) fn new(green: &'a GreenNode, start: TextSize) -> Self {
        Self {
            green,
            range: TextRange::new(start, start + green.text_len()),
        }
    }

    #[inline]
    pub fn kind(&self) -> SyntaxKind {
        self.green.kind()
    }

    #[inline]
    pub fn text_range(&self) -> TextRange {
        self.range
    }

    #[inline]
    pub fn green(&self) -> &'a GreenNode {
        self.green
    }

    #[inline]
    pub fn to_ptr(&self) -> SyntaxNodePtr {
        SyntaxNodePtr {
            kind: self.green.kind(),
            range: self.range,
        }
    }

    #[inline]
    pub fn children(&self) -> impl DoubleEndedIterator<Item = AmberNode<'a>> + Clone {
        self.green.slice().iter().filter_map(|child| match child {
            GreenChild::Node { offset, node } => Some(AmberNode::new(node, self.range.start() + offset)),
            GreenChild::Token { .. } => None,
        })
    }

    #[inline]
    pub fn children_with_tokens(&self) -> impl DoubleEndedIterator<Item = NodeOrToken<AmberNode<'a>, AmberToken<'a>>> {
        self.green.slice().iter().map(|child| match child {
            GreenChild::Node { offset, node } => NodeOrToken::Node(AmberNode::new(node, self.range.start() + offset)),
            GreenChild::Token { offset, token } => {
                NodeOrToken::Token(AmberToken::new(token, self.range.start() + offset))
            }
        })
    }
}

impl<'a> From<&'a SyntaxNode> for AmberNode<'a> {
    #[inline]
    fn from(node: &'a SyntaxNode) -> Self {
        Self {
            green: node.green(),
            range: node.text_range(),
        }
    }
}
