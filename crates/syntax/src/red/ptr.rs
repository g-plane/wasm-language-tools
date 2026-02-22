use crate::{SyntaxKind, SyntaxNode, TextRange};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// A "pointer" to a [`SyntaxNode`](crate::SyntaxNode), carrying syntax kind and text range.
pub struct SyntaxNodePtr {
    pub(crate) kind: SyntaxKind,
    pub(crate) range: TextRange,
}

impl SyntaxNodePtr {
    #[inline]
    /// Create a new pointer with the given node.
    pub fn new(node: &SyntaxNode) -> Self {
        Self {
            kind: node.kind(),
            range: node.text_range(),
        }
    }

    #[inline]
    /// Kind of this corresponding node.
    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }

    #[inline]
    /// The range that this corresponding node covers in the original text.
    pub fn text_range(&self) -> TextRange {
        self.range
    }

    #[inline]
    /// Resolve this pointer to a [`SyntaxNode`](crate::SyntaxNode) under the given ancestor node.
    pub fn try_to_node(&self, ancestor: &SyntaxNode) -> Option<SyntaxNode> {
        std::iter::successors(Some(ancestor.clone()), |node| node.child_at_range(self.range))
            .find(|it| it.text_range() == self.range && it.kind() == self.kind)
    }

    #[inline]
    /// Like [`try_to_node`](Self::try_to_node), but panics when the node can't be resolved.
    pub fn to_node(&self, ancestor: &SyntaxNode) -> SyntaxNode {
        self.try_to_node(ancestor)
            .unwrap_or_else(|| panic!("can't resolve {self:?}"))
    }
}
