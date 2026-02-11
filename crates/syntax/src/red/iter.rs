use crate::{GreenNode, SyntaxElement, SyntaxNode, green::GreenChild};
use std::{iter::FusedIterator, ptr::NonNull};

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

pub struct SyntaxElementChildren {
    pub(super) parent: SyntaxNode,
    pub(super) green: NonNull<GreenNode>,
    pub(super) index: u32,
}
impl Iterator for SyntaxElementChildren {
    type Item = SyntaxElement;
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;
        let child = unsafe { self.green.as_ref() }.slice().get(index as usize)?;
        match child {
            GreenChild::Node { offset, node } => Some(self.parent.new_child(index, node, *offset).into()),
            GreenChild::Token { offset, token } => Some(self.parent.new_token(index, token, *offset).into()),
        }
    }
}
impl FusedIterator for SyntaxElementChildren {}
