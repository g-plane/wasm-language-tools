use crate::{GreenNode, SyntaxNode, green::GreenChild};
use std::{iter::FusedIterator, ptr::NonNull};

/// The iterator over the child nodes of a [`SyntaxNode`](crate::SyntaxNode).
pub struct SyntaxNodeChildren {
    pub(super) parent: SyntaxNode,
    pub(super) green: NonNull<GreenNode>,
    pub(super) index: u32,
}
impl Iterator for SyntaxNodeChildren {
    type Item = SyntaxNode;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let index = self.index;
            self.index += 1;
            let child = unsafe { self.green.as_ref() }.slice().get(index as usize)?;
            if let GreenChild::Node { offset, node } = child {
                return Some(self.parent.new_child(index, node, *offset));
            }
        }
    }
}
impl FusedIterator for SyntaxNodeChildren {}
