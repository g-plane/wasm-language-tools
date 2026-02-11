use crate::{NodeOrToken, SyntaxElement, SyntaxNode};
use std::iter::FusedIterator;

pub struct Descendants {
    next: Option<SyntaxNode>,
    child_entered: bool,
}
impl Descendants {
    pub(crate) fn new(start: SyntaxNode) -> Self {
        Self {
            next: Some(start),
            child_entered: false,
        }
    }
    /// This should be considered as optimization only, no semantics guaranteed.
    pub fn skip_subtree(&mut self) {
        if self.child_entered
            && let Some(next) = &self.next
        {
            self.next = self.exit_parent(next);
        }
    }
    fn exit_parent(&self, current: &SyntaxNode) -> Option<SyntaxNode> {
        let mut parent = current.parent();
        while let Some(p) = parent {
            let next = p.next_sibling();
            if next.is_some() {
                return next;
            }
            parent = p.parent();
        }
        None
    }
}
impl Iterator for Descendants {
    type Item = SyntaxNode;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().inspect(|next| {
            if let Some(child) = next.first_child() {
                self.next = Some(child);
                self.child_entered = true;
            } else {
                self.next = next.next_sibling().or_else(|| self.exit_parent(next));
                self.child_entered = false;
            }
        })
    }
}
impl FusedIterator for Descendants {}

pub struct DescendantsWithTokens {
    pub(crate) next: Option<SyntaxElement>,
}
impl DescendantsWithTokens {
    pub(crate) fn new(start: SyntaxElement) -> Self {
        Self { next: Some(start) }
    }
    fn exit_parent(&mut self, mut parent: Option<SyntaxNode>) -> Option<SyntaxElement> {
        while let Some(p) = parent {
            let next = p.next_sibling_or_token();
            if next.is_some() {
                return next;
            }
            parent = p.parent();
        }
        None
    }
}
impl Iterator for DescendantsWithTokens {
    type Item = SyntaxElement;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().inspect(|next| match next {
            NodeOrToken::Node(next) => {
                self.next = next
                    .first_child_or_token()
                    .or_else(|| next.next_sibling_or_token())
                    .or_else(|| self.exit_parent(next.parent()));
            }
            NodeOrToken::Token(next) => {
                self.next = next
                    .next_sibling_or_token()
                    .or_else(|| self.exit_parent(Some(next.parent())));
            }
        })
    }
}
impl FusedIterator for DescendantsWithTokens {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GreenNode, GreenToken, SyntaxKind::*};

    #[test]
    fn skip_subtree_in_deepest_node() {
        let green = GreenNode::new(
            ROOT,
            [
                GreenNode::new(
                    MODULE,
                    [GreenNode::new(
                        MODULE_FIELD_FUNC,
                        [GreenNode::new(PLAIN_INSTR, [GreenToken::new(INSTR_NAME, "local.get").into()]).into()],
                    )
                    .into()],
                )
                .into(),
                GreenNode::new(MODULE, [GreenToken::new(KEYWORD, "module").into()]).into(),
            ],
        );
        let mut descendants = SyntaxNode::new_root(green).descendants();
        assert_eq!(descendants.next().unwrap().kind(), ROOT);
        assert_eq!(descendants.next().unwrap().kind(), MODULE);
        assert_eq!(descendants.next().unwrap().kind(), MODULE_FIELD_FUNC);
        descendants.skip_subtree();
        assert_eq!(descendants.next().unwrap().kind(), MODULE);
        assert!(descendants.next().is_none());
    }

    #[test]
    fn skip_subtree_for_zero_children() {
        let green = GreenNode::new(
            MODULE,
            [
                GreenNode::new(MODULE_FIELD_FUNC, [GreenToken::new(KEYWORD, "func").into()]).into(),
                GreenNode::new(MODULE_FIELD_DATA, [GreenToken::new(KEYWORD, "data").into()]).into(),
                GreenNode::new(MODULE_FIELD_GLOBAL, [GreenToken::new(KEYWORD, "global").into()]).into(),
            ],
        )
        .into();
        let mut descendants = SyntaxNode::new_root(green).descendants();
        assert_eq!(descendants.next().unwrap().kind(), MODULE);
        assert_eq!(descendants.next().unwrap().kind(), MODULE_FIELD_FUNC);
        descendants.skip_subtree();
        assert_eq!(descendants.next().unwrap().kind(), MODULE_FIELD_DATA);
        assert_eq!(descendants.next().unwrap().kind(), MODULE_FIELD_GLOBAL);
        assert!(descendants.next().is_none());
    }
}
