use crate::{SyntaxKind, SyntaxNode, TextRange};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SyntaxNodePtr {
    pub(crate) kind: SyntaxKind,
    pub(crate) range: TextRange,
}

impl SyntaxNodePtr {
    #[inline]
    pub fn new(node: &SyntaxNode) -> Self {
        Self {
            kind: node.kind(),
            range: node.text_range(),
        }
    }

    #[inline]
    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }

    #[inline]
    pub fn text_range(&self) -> TextRange {
        self.range
    }

    #[inline]
    pub fn try_to_node(&self, ancestor: &SyntaxNode) -> Option<SyntaxNode> {
        std::iter::successors(Some(ancestor.clone()), |node| node.child_at_range(self.range))
            .find(|it| it.text_range() == self.range && it.kind() == self.kind)
    }

    #[inline]
    pub fn to_node(&self, ancestor: &SyntaxNode) -> SyntaxNode {
        self.try_to_node(ancestor)
            .unwrap_or_else(|| panic!("can't resolve {self:?}"))
    }
}
