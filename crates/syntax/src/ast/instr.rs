use super::{module::TypeUse, ty::Result, SyntaxKind, SyntaxNode, SyntaxToken, WatLanguage};
use rowan::ast::{
    support::{child, children, token},
    AstChildren, AstNode,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockBlock {
    syntax: SyntaxNode,
}
impl BlockBlock {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn block_type(&self) -> Option<BlockType> {
        child(&self.syntax)
    }
    #[inline]
    pub fn instrs(&self) -> AstChildren<Instr> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for BlockBlock {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::BLOCK_BLOCK
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockIf {
    syntax: SyntaxNode,
}
impl BlockIf {
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
    pub fn block_type(&self) -> Option<BlockType> {
        child(&self.syntax)
    }
    #[inline]
    pub fn instrs(&self) -> AstChildren<Instr> {
        children(&self.syntax)
    }
    #[inline]
    pub fn then_block(&self) -> Option<BlockIfThen> {
        child(&self.syntax)
    }
    #[inline]
    pub fn else_block(&self) -> Option<BlockIfElse> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for BlockIf {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::BLOCK_IF
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockIfElse {
    syntax: SyntaxNode,
}
impl BlockIfElse {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn instrs(&self) -> AstChildren<Instr> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for BlockIfElse {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::BLOCK_IF_ELSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockIfThen {
    syntax: SyntaxNode,
}
impl BlockIfThen {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn instrs(&self) -> AstChildren<Instr> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for BlockIfThen {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::BLOCK_IF_THEN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BlockInstr {
    Block(BlockBlock),
    Loop(BlockLoop),
    If(BlockIf),
}
impl AstNode for BlockInstr {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            SyntaxKind::BLOCK_BLOCK | SyntaxKind::BLOCK_LOOP | SyntaxKind::BLOCK_IF
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind() {
            SyntaxKind::BLOCK_BLOCK => Some(BlockInstr::Block(BlockBlock { syntax })),
            SyntaxKind::BLOCK_LOOP => Some(BlockInstr::Loop(BlockLoop { syntax })),
            SyntaxKind::BLOCK_IF => Some(BlockInstr::If(BlockIf { syntax })),
            _ => None,
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            BlockInstr::Block(it) => it.syntax(),
            BlockInstr::Loop(it) => it.syntax(),
            BlockInstr::If(it) => it.syntax(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockLoop {
    syntax: SyntaxNode,
}
impl BlockLoop {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn block_type(&self) -> Option<BlockType> {
        child(&self.syntax)
    }
    #[inline]
    pub fn instrs(&self) -> AstChildren<Instr> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for BlockLoop {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::BLOCK_LOOP
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockType {
    syntax: SyntaxNode,
}
impl BlockType {
    #[inline]
    pub fn result(&self) -> Option<Result> {
        child(&self.syntax)
    }
    #[inline]
    pub fn type_use(&self) -> Option<TypeUse> {
        child(&self.syntax)
    }
}
impl AstNode for BlockType {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::BLOCK_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(BlockType { syntax })
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
pub enum Instr {
    Block(BlockInstr),
    Plain(PlainInstr),
}
impl AstNode for Instr {
    type Language = WatLanguage;
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
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind() {
            SyntaxKind::PLAIN_INSTR => Some(Instr::Plain(PlainInstr { syntax })),
            SyntaxKind::BLOCK_BLOCK => Some(Instr::Block(BlockInstr::Block(BlockBlock { syntax }))),
            SyntaxKind::BLOCK_LOOP => Some(Instr::Block(BlockInstr::Loop(BlockLoop { syntax }))),
            SyntaxKind::BLOCK_IF => Some(Instr::Block(BlockInstr::If(BlockIf { syntax }))),
            _ => None,
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Instr::Block(it) => it.syntax(),
            Instr::Plain(it) => it.syntax(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Operand {
    syntax: SyntaxNode,
}
impl Operand {
    #[inline]
    pub fn float(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::FLOAT)
    }
    #[inline]
    pub fn int(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::INT)
    }
    #[inline]
    pub fn string(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::STRING)
    }
    #[inline]
    pub fn ident(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn type_use(&self) -> Option<TypeUse> {
        child(&self.syntax)
    }
    #[inline]
    pub fn instr(&self) -> Option<Instr> {
        child(&self.syntax)
    }
    #[inline]
    pub fn mem_arg(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::MEM_ARG)
    }
    #[inline]
    pub fn heap_type(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::HEAP_TYPE)
    }
}
impl AstNode for Operand {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::OPERAND
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(Operand { syntax })
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
pub struct PlainInstr {
    syntax: SyntaxNode,
}
impl PlainInstr {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn instr_name(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::INSTR_NAME)
    }
    #[inline]
    pub fn operands(&self) -> AstChildren<Operand> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for PlainInstr {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::PLAIN_INSTR
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
