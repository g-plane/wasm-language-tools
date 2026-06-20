use crate::SyntaxNode;
use std::iter::FusedIterator;

pub(crate) struct Descendants<'a> {
    start: SyntaxNode<'a>,
    next: Option<SyntaxNode<'a>>,
    child_entered: bool,
}
impl<'a> Descendants<'a> {
    pub(crate) fn new(start: SyntaxNode<'a>) -> Self {
        Self {
            start: start.clone(),
            next: Some(start),
            child_entered: false,
        }
    }
    fn exit_parent(&self, current: &SyntaxNode<'a>) -> Option<SyntaxNode<'a>> {
        let mut parent = current.parent();
        while let Some(p) = parent
            && p != self.start
        {
            let next = p.next_siblings().next();
            if next.is_some() {
                return next;
            }
            parent = p.parent();
        }
        None
    }
}
impl<'a> Iterator for Descendants<'a> {
    type Item = SyntaxNode<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().inspect(|next| {
            if let Some(child) = next.children().next() {
                self.next = Some(child);
                self.child_entered = true;
            } else if next != &self.start {
                self.next = next.next_siblings().next().or_else(|| self.exit_parent(next));
                self.child_entered = false;
            }
        })
    }
}
impl FusedIterator for Descendants<'_> {}
