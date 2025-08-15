use super::{lexer, GreenElement, Parser};
use rowan::GreenNode;
use wat_syntax::SyntaxKind;

impl<'s> Parser<'s> {
    pub(super) fn start_node(&self) -> NodeMark {
        NodeMark(self.elements.len())
    }

    pub(super) fn finish_node(&mut self, kind: SyntaxKind, mark: NodeMark) -> GreenNode {
        GreenNode::new(kind.into(), self.elements.drain(mark.0..))
    }

    pub(super) fn add_child<T>(&mut self, node_or_token: T)
    where
        T: Into<GreenElement>,
    {
        self.elements.push(node_or_token.into());
    }

    pub(super) fn checkpoint(&self) -> Checkpoint<'s> {
        Checkpoint {
            elements: self.elements.len(),
            lexer: self.lexer.checkpoint(),
        }
    }

    pub(super) fn reset(&mut self, checkpoint: Checkpoint<'s>) {
        self.elements.truncate(checkpoint.elements);
        self.lexer.reset(checkpoint.lexer);
    }
}

pub(super) struct NodeMark(usize);

#[derive(Clone, Copy)]
pub(super) struct Checkpoint<'s> {
    pub elements: usize,
    pub lexer: lexer::Checkpoint<'s>,
}
