use crate::{GreenNode, SyntaxNode, green::GreenChild};
use std::iter::FusedIterator;

/// The iterator over the child nodes of a [`SyntaxNode`](crate::SyntaxNode).
pub struct SyntaxNodeChildren<'a> {
    pub(super) parent: SyntaxNode<'a>,
    pub(super) green: &'a GreenNode,
    pub(super) index: u32,
}
impl<'a> Iterator for SyntaxNodeChildren<'a> {
    type Item = SyntaxNode<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let index = self.index;
            self.index += 1;
            let child = self.green.slice().get(index as usize)?;
            if let GreenChild::Node { offset, node } = child {
                return Some(self.parent.new_child(index, node, *offset));
            }
        }
    }
}
impl FusedIterator for SyntaxNodeChildren<'_> {}
