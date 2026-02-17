//! Abstract Syntax Tree, layered on top of untyped `SyntaxNode`s.

use self::support::children;
pub use self::{instr::*, module::*, ty::*};
use crate::{SyntaxKind, SyntaxNode, SyntaxNodeChildren};
use std::{iter, marker::PhantomData};

mod instr;
mod module;
mod ty;

pub trait AstNode {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxNode;
}

pub mod support {
    use super::{AstChildren, AstNode};
    use crate::{SyntaxKind, SyntaxNode, SyntaxToken};

    pub fn child<N: AstNode>(parent: &SyntaxNode) -> Option<N> {
        parent.children_by_kind(N::can_cast).find_map(N::cast)
    }
    pub fn children<N: AstNode>(parent: &SyntaxNode) -> AstChildren<N> {
        AstChildren::new(parent)
    }
    pub fn token(parent: &SyntaxNode, kind: SyntaxKind) -> Option<SyntaxToken> {
        parent.tokens_by_kind(kind).next()
    }
}

pub struct AstChildren<N: AstNode> {
    inner: SyntaxNodeChildren,
    _marker: PhantomData<N>,
}
impl<N: AstNode> AstChildren<N> {
    pub(crate) fn new(parent: &SyntaxNode) -> Self {
        Self {
            inner: parent.children(),
            _marker: PhantomData,
        }
    }
}
impl<N: AstNode> Iterator for AstChildren<N> {
    type Item = N;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.find_map(N::cast)
    }
}
impl<N: AstNode> iter::FusedIterator for AstChildren<N> {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Root {
    syntax: SyntaxNode,
}
impl Root {
    #[inline]
    pub fn modules(&self) -> AstChildren<Module> {
        children(&self.syntax)
    }
}
impl AstNode for Root {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::ROOT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(Root { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
