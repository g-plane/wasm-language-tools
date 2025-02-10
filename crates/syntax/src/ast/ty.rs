use super::{module::Index, SyntaxKind, SyntaxNode, SyntaxToken, WatLanguage};
use crate::SyntaxElement;
use rowan::{
    ast::{
        support::{child, children, token},
        AstChildren, AstNode,
    },
    NodeOrToken,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncType {
    syntax: SyntaxNode,
}
impl FuncType {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn params(&self) -> AstChildren<Param> {
        children(&self.syntax)
    }
    #[inline]
    pub fn results(&self) -> AstChildren<Result> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for FuncType {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::FUNC_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(FuncType { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GlobalType {
    syntax: SyntaxNode,
}
impl GlobalType {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn mut_keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn val_type(&self) -> Option<ValType> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for GlobalType {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::GLOBAL_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(GlobalType { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HeapType {
    syntax: SyntaxNode,
}
impl HeapType {
    #[inline]
    pub fn abs_heap_type(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::ABS_HEAP_TYPE)
    }
    #[inline]
    pub fn index(&self) -> Option<Index> {
        child(&self.syntax)
    }
}
impl AstNode for HeapType {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::HEAP_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(HeapType { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Limits {
    syntax: SyntaxNode,
}
impl Limits {
    #[inline]
    pub fn min(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::UNSIGNED_INT)
    }
    #[inline]
    pub fn max(&self) -> Option<SyntaxToken> {
        self.syntax
            .children_with_tokens()
            .filter_map(|it| match it {
                NodeOrToken::Token(token) if token.kind() == SyntaxKind::UNSIGNED_INT => {
                    Some(token)
                }
                _ => None,
            })
            .nth(1)
    }
}
impl AstNode for Limits {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::LIMITS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(Limits { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MemoryType {
    syntax: SyntaxNode,
}
impl MemoryType {
    #[inline]
    pub fn limits(&self) -> Option<Limits> {
        child(&self.syntax)
    }
}
impl AstNode for MemoryType {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MEMORY_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(MemoryType { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    syntax: SyntaxNode,
}
impl Param {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn val_types(&self) -> AstChildren<ValType> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for Param {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::PARAM
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(Param { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefType {
    syntax: SyntaxNode,
}
impl RefType {
    #[inline]
    pub fn abbr_ref_type(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::ABBR_REF_TYPE)
    }
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn null_keyword(&self) -> Option<SyntaxToken> {
        self.syntax.children_with_tokens().find_map(|it| match it {
            SyntaxElement::Token(token)
                if token.kind() == SyntaxKind::KEYWORD && token.text() == "null" =>
            {
                Some(token)
            }
            _ => None,
        })
    }
    #[inline]
    pub fn heap_type(&self) -> Option<HeapType> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for RefType {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::REF_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(RefType { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Result {
    syntax: SyntaxNode,
}
impl Result {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn val_types(&self) -> AstChildren<ValType> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for Result {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::RESULT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(Result { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableType {
    syntax: SyntaxNode,
}
impl TableType {
    #[inline]
    pub fn limits(&self) -> Option<Limits> {
        child(&self.syntax)
    }
    #[inline]
    pub fn ref_type(&self) -> Option<RefType> {
        child(&self.syntax)
    }
}
impl AstNode for TableType {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::TABLE_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(TableType { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValType {
    syntax: SyntaxNode,
}
impl ValType {
    #[inline]
    pub fn num_type(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::NUM_TYPE)
    }
    #[inline]
    pub fn vec_type(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::VEC_TYPE)
    }
    #[inline]
    pub fn ref_type(&self) -> Option<RefType> {
        child(&self.syntax)
    }
}
impl AstNode for ValType {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::VAL_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ValType { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
