use crate::{AmberNode, AmberToken, NodeOrToken};
use std::iter::FusedIterator;

pub(crate) type DescendantToken<'a> = (AmberToken<'a>, AmberNode<'a>, Option<AmberNode<'a>>);
pub(crate) struct DescendantTokens<'a> {
    stack: Vec<(AmberNode<'a>, usize)>,
}
impl<'a> DescendantTokens<'a> {
    pub(crate) fn new(start: AmberNode<'a>) -> Self {
        let mut stack = Vec::with_capacity(7);
        stack.push((start, 0));
        Self { stack }
    }
}
impl<'a> Iterator for DescendantTokens<'a> {
    type Item = DescendantToken<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((node, index)) = self.stack.last_mut() {
            match node.child_or_token_at(*index) {
                Some(NodeOrToken::Node(node)) => {
                    self.stack.push((node, 0));
                }
                Some(NodeOrToken::Token(token)) => {
                    *index += 1;
                    return Some((
                        token,
                        *node,
                        self.stack
                            .len()
                            .checked_sub(2)
                            .and_then(|i| self.stack.get(i))
                            .map(|(node, _)| *node),
                    ));
                }
                None => {
                    self.stack.pop();
                    if let Some((_, index)) = self.stack.last_mut() {
                        *index += 1;
                    }
                }
            }
        }
        None
    }
}
impl FusedIterator for DescendantTokens<'_> {}
