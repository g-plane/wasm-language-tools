//! Abstract Syntax Tree, layered on top of untyped `SyntaxNode`s.

pub use self::{instr::*, module::*, ty::*};
use super::{SyntaxKind, SyntaxNode, SyntaxToken, WatLanguage};
use rowan::ast::{support::children, AstChildren, AstNode};

mod instr;
mod module;
mod ty;

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
    type Language = WatLanguage;
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
