//! Abstract Syntax Tree, layered on top of untyped `SyntaxNode`s.

use self::support::children;
pub use self::{instr::*, module::*, ty::*};
use crate::{SyntaxKind, SyntaxNode, SyntaxNodeChildren};
use std::{iter, marker::PhantomData};

mod instr;
mod module;
mod ty;

pub trait AstNode<'a> {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(node: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxNode<'a>;
}

pub mod support {
    use super::{AstChildren, AstNode};
    use crate::{SyntaxKind, SyntaxNode, SyntaxToken};

    pub fn child<'a, N: AstNode<'a>>(parent: &SyntaxNode<'a>) -> Option<N> {
        parent.children_by_kind(N::can_cast).find_map(N::cast)
    }
    pub fn children<'a, N: AstNode<'a>>(parent: &SyntaxNode<'a>) -> AstChildren<'a, N> {
        AstChildren::new(parent)
    }
    pub fn token<'a>(parent: &SyntaxNode<'a>, kind: SyntaxKind) -> Option<SyntaxToken<'a>> {
        parent.tokens_by_kind(kind).next()
    }
}

pub struct AstChildren<'a, N: AstNode<'a>> {
    inner: SyntaxNodeChildren<'a>,
    _marker: PhantomData<N>,
}
impl<'a, N: AstNode<'a>> AstChildren<'a, N> {
    pub(crate) fn new(parent: &SyntaxNode<'a>) -> Self {
        Self {
            inner: parent.children(),
            _marker: PhantomData,
        }
    }
}
impl<'a, N: AstNode<'a>> Iterator for AstChildren<'a, N> {
    type Item = N;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.find_map(N::cast)
    }
}
impl<'a, N: AstNode<'a>> iter::FusedIterator for AstChildren<'a, N> {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Root<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> Root<'a> {
    #[inline]
    pub fn modules(&self) -> AstChildren<'a, Module<'a>> {
        children(&self.syntax)
    }
}
impl<'a> AstNode<'a> for Root<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::ROOT
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}
