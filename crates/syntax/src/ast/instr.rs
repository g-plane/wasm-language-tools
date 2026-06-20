use super::{
    AstChildren, AstNode,
    module::{Index, TypeUse},
    support::*,
    ty::{HeapType, RefType},
};
use crate::{SyntaxKind, SyntaxNode, SyntaxToken};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockBlock<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> BlockBlock<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn ident_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn type_use(&self) -> Option<TypeUse<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn instrs(&self) -> AstChildren<'a, Instr<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn end_keyword(&self) -> Option<SyntaxToken<'a>> {
        self.syntax
            .tokens_by_kind(SyntaxKind::KEYWORD)
            .find(|token| token.text() == "end")
    }
    #[inline]
    pub fn end_ident_token(&self) -> Option<SyntaxToken<'a>> {
        self.syntax.tokens_by_kind(SyntaxKind::IDENT).nth(1)
    }
}
impl<'a> AstNode<'a> for BlockBlock<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::BLOCK_BLOCK
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(BlockBlock { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockIf<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> BlockIf<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn ident_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn type_use(&self) -> Option<TypeUse<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn instrs(&self) -> AstChildren<'a, Instr<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn then_block(&self) -> Option<BlockIfThen<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn else_block(&self) -> Option<BlockIfElse<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn end_keyword(&self) -> Option<SyntaxToken<'a>> {
        self.syntax
            .tokens_by_kind(SyntaxKind::KEYWORD)
            .find(|token| token.text() == "end")
    }
    #[inline]
    pub fn end_ident_token(&self) -> Option<SyntaxToken<'a>> {
        self.syntax.tokens_by_kind(SyntaxKind::IDENT).nth(1)
    }
}
impl<'a> AstNode<'a> for BlockIf<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::BLOCK_IF
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(BlockIf { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockIfElse<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> BlockIfElse<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn ident_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn instrs(&self) -> AstChildren<'a, Instr<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for BlockIfElse<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::BLOCK_IF_ELSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(BlockIfElse { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockIfThen<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> BlockIfThen<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn ident_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn instrs(&self) -> AstChildren<'a, Instr<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for BlockIfThen<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::BLOCK_IF_THEN
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(BlockIfThen { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BlockInstr<'a> {
    Block(BlockBlock<'a>),
    Loop(BlockLoop<'a>),
    If(BlockIf<'a>),
    TryTable(BlockTryTable<'a>),
}
impl<'a> AstNode<'a> for BlockInstr<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            SyntaxKind::BLOCK_BLOCK | SyntaxKind::BLOCK_LOOP | SyntaxKind::BLOCK_IF | SyntaxKind::BLOCK_TRY_TABLE
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind() {
            SyntaxKind::BLOCK_BLOCK => Some(BlockInstr::Block(BlockBlock { syntax })),
            SyntaxKind::BLOCK_LOOP => Some(BlockInstr::Loop(BlockLoop { syntax })),
            SyntaxKind::BLOCK_IF => Some(BlockInstr::If(BlockIf { syntax })),
            SyntaxKind::BLOCK_TRY_TABLE => Some(BlockInstr::TryTable(BlockTryTable { syntax })),
            _ => None,
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        match self {
            BlockInstr::Block(it) => it.syntax(),
            BlockInstr::Loop(it) => it.syntax(),
            BlockInstr::If(it) => it.syntax(),
            BlockInstr::TryTable(it) => it.syntax(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockLoop<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> BlockLoop<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn ident_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn type_use(&self) -> Option<TypeUse<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn instrs(&self) -> AstChildren<'a, Instr<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn end_keyword(&self) -> Option<SyntaxToken<'a>> {
        self.syntax
            .tokens_by_kind(SyntaxKind::KEYWORD)
            .find(|token| token.text() == "end")
    }
    #[inline]
    pub fn end_ident_token(&self) -> Option<SyntaxToken<'a>> {
        self.syntax.tokens_by_kind(SyntaxKind::IDENT).nth(1)
    }
}
impl<'a> AstNode<'a> for BlockLoop<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::BLOCK_LOOP
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(BlockLoop { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockTryTable<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> BlockTryTable<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn ident_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn type_use(&self) -> Option<TypeUse<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn catches(&self) -> AstChildren<'a, Cat<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn instrs(&self) -> AstChildren<'a, Instr<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn end_keyword(&self) -> Option<SyntaxToken<'a>> {
        self.syntax
            .tokens_by_kind(SyntaxKind::KEYWORD)
            .find(|token| token.text() == "end")
    }
    #[inline]
    pub fn end_ident_token(&self) -> Option<SyntaxToken<'a>> {
        self.syntax.tokens_by_kind(SyntaxKind::IDENT).nth(1)
    }
}
impl<'a> AstNode<'a> for BlockTryTable<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::BLOCK_TRY_TABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(BlockTryTable { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Cat<'a> {
    Catch(Catch<'a>),
    CatchAll(CatchAll<'a>),
}
impl<'a> AstNode<'a> for Cat<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(kind, SyntaxKind::CATCH | SyntaxKind::CATCH_ALL)
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind() {
            SyntaxKind::CATCH => Some(Cat::Catch(Catch { syntax })),
            SyntaxKind::CATCH_ALL => Some(Cat::CatchAll(CatchAll { syntax })),
            _ => None,
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        match self {
            Cat::Catch(it) => it.syntax(),
            Cat::CatchAll(it) => it.syntax(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Catch<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> Catch<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn tag_index(&self) -> Option<Index<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn label_index(&self) -> Option<Index<'a>> {
        children(&self.syntax).nth(1)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for Catch<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::CATCH
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(Catch { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CatchAll<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> CatchAll<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn label_index(&self) -> Option<Index<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for CatchAll<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::CATCH_ALL
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(CatchAll { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instr<'a> {
    Block(BlockInstr<'a>),
    Plain(PlainInstr<'a>),
}
impl<'a> AstNode<'a> for Instr<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            SyntaxKind::PLAIN_INSTR
                | SyntaxKind::BLOCK_BLOCK
                | SyntaxKind::BLOCK_LOOP
                | SyntaxKind::BLOCK_IF
                | SyntaxKind::BLOCK_TRY_TABLE
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind() {
            SyntaxKind::PLAIN_INSTR => Some(Instr::Plain(PlainInstr { syntax })),
            SyntaxKind::BLOCK_BLOCK => Some(Instr::Block(BlockInstr::Block(BlockBlock { syntax }))),
            SyntaxKind::BLOCK_LOOP => Some(Instr::Block(BlockInstr::Loop(BlockLoop { syntax }))),
            SyntaxKind::BLOCK_IF => Some(Instr::Block(BlockInstr::If(BlockIf { syntax }))),
            SyntaxKind::BLOCK_TRY_TABLE => Some(Instr::Block(BlockInstr::TryTable(BlockTryTable { syntax }))),
            _ => None,
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        match self {
            Instr::Block(it) => it.syntax(),
            Instr::Plain(it) => it.syntax(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Immediate<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> Immediate<'a> {
    #[inline]
    pub fn float(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::FLOAT)
    }
    #[inline]
    pub fn int(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::INT)
    }
    #[inline]
    pub fn string(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::STRING)
    }
    #[inline]
    pub fn ident(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn shape_descriptor(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::SHAPE_DESCRIPTOR)
    }
    #[inline]
    pub fn type_use(&self) -> Option<TypeUse<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn mem_arg(&self) -> Option<MemArg<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn heap_type(&self) -> Option<HeapType<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn ref_type(&self) -> Option<RefType<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn on_clause(&self) -> Option<OnClause<'a>> {
        child(&self.syntax)
    }
}
impl<'a> AstNode<'a> for Immediate<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::IMMEDIATE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(Immediate { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MemArg<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> MemArg<'a> {
    #[inline]
    pub fn mem_arg_keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::MEM_ARG_KEYWORD)
    }
    #[inline]
    pub fn eq_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::EQ)
    }
    #[inline]
    pub fn unsigned_int(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::UNSIGNED_INT)
    }
}
impl<'a> AstNode<'a> for MemArg<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MEM_ARG
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(MemArg { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OnClause<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> OnClause<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn tag_index(&self) -> Option<Index<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn label_index(&self) -> Option<Index<'a>> {
        children(&self.syntax).nth(1)
    }
    #[inline]
    pub fn switch_keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::MODIFIER_KEYWORD)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for OnClause<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::ON_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(OnClause { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlainInstr<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> PlainInstr<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn instr_name(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::INSTR_NAME)
    }
    #[inline]
    pub fn immediates(&self) -> AstChildren<'a, Immediate<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn instrs(&self) -> AstChildren<'a, Instr<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for PlainInstr<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::PLAIN_INSTR
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(PlainInstr { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}
