use super::{
    AstChildren, AstNode,
    module::{Index, TypeUse},
    support::*,
};
use crate::{SyntaxKind, SyntaxNode, SyntaxToken};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AddrType<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> AddrType<'a> {
    #[inline]
    pub fn type_keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::TYPE_KEYWORD)
    }
}
impl<'a> AstNode<'a> for AddrType<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::ADDR_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(AddrType { syntax })
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
pub struct ArrayType<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ArrayType<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn field_type(&self) -> Option<FieldType<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for ArrayType<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::ARRAY_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ArrayType { syntax })
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
pub enum CompType<'a> {
    Array(ArrayType<'a>),
    Struct(StructType<'a>),
    Func(FuncType<'a>),
    Cont(ContType<'a>),
}
impl<'a> AstNode<'a> for CompType<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            SyntaxKind::ARRAY_TYPE | SyntaxKind::STRUCT_TYPE | SyntaxKind::FUNC_TYPE | SyntaxKind::CONT_TYPE
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind() {
            SyntaxKind::ARRAY_TYPE => Some(CompType::Array(ArrayType { syntax })),
            SyntaxKind::STRUCT_TYPE => Some(CompType::Struct(StructType { syntax })),
            SyntaxKind::FUNC_TYPE => Some(CompType::Func(FuncType { syntax })),
            SyntaxKind::CONT_TYPE => Some(CompType::Cont(ContType { syntax })),
            _ => None,
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        match self {
            CompType::Array(it) => it.syntax(),
            CompType::Struct(it) => it.syntax(),
            CompType::Func(it) => it.syntax(),
            CompType::Cont(it) => it.syntax(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContType<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ContType<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn index(&self) -> Option<Index<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for ContType<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::CONT_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ContType { syntax })
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
pub enum ExternType<'a> {
    Func(ExternTypeFunc<'a>),
    Global(ExternTypeGlobal<'a>),
    Memory(ExternTypeMemory<'a>),
    Table(ExternTypeTable<'a>),
    Tag(ExternTypeTag<'a>),
}
impl<'a> AstNode<'a> for ExternType<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            SyntaxKind::EXTERN_TYPE_FUNC
                | SyntaxKind::EXTERN_TYPE_GLOBAL
                | SyntaxKind::EXTERN_TYPE_MEMORY
                | SyntaxKind::EXTERN_TYPE_TABLE
                | SyntaxKind::EXTERN_TYPE_TAG
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind() {
            SyntaxKind::EXTERN_TYPE_FUNC => Some(ExternType::Func(ExternTypeFunc { syntax })),
            SyntaxKind::EXTERN_TYPE_GLOBAL => Some(ExternType::Global(ExternTypeGlobal { syntax })),
            SyntaxKind::EXTERN_TYPE_MEMORY => Some(ExternType::Memory(ExternTypeMemory { syntax })),
            SyntaxKind::EXTERN_TYPE_TABLE => Some(ExternType::Table(ExternTypeTable { syntax })),
            SyntaxKind::EXTERN_TYPE_TAG => Some(ExternType::Tag(ExternTypeTag { syntax })),
            _ => None,
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        match self {
            ExternType::Func(it) => it.syntax(),
            ExternType::Global(it) => it.syntax(),
            ExternType::Memory(it) => it.syntax(),
            ExternType::Table(it) => it.syntax(),
            ExternType::Tag(it) => it.syntax(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternTypeFunc<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ExternTypeFunc<'a> {
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
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for ExternTypeFunc<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::EXTERN_TYPE_FUNC
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ExternTypeFunc { syntax })
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
pub struct ExternTypeGlobal<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ExternTypeGlobal<'a> {
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
    pub fn global_type(&self) -> Option<GlobalType<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for ExternTypeGlobal<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::EXTERN_TYPE_GLOBAL
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ExternTypeGlobal { syntax })
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
pub struct ExternTypeMemory<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ExternTypeMemory<'a> {
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
    pub fn mem_type(&self) -> Option<MemType<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for ExternTypeMemory<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::EXTERN_TYPE_MEMORY
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ExternTypeMemory { syntax })
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
pub struct ExternTypeTable<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ExternTypeTable<'a> {
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
    pub fn table_type(&self) -> Option<TableType<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for ExternTypeTable<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::EXTERN_TYPE_TABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ExternTypeTable { syntax })
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
pub struct ExternTypeTag<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ExternTypeTag<'a> {
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
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for ExternTypeTag<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::EXTERN_TYPE_TAG
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ExternTypeTag { syntax })
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
pub struct Field<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> Field<'a> {
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
    pub fn field_types(&self) -> AstChildren<'a, FieldType<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for Field<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::FIELD
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(Field { syntax })
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
pub struct FieldType<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> FieldType<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn mut_keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn storage_type(&self) -> Option<StorageType<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for FieldType<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::FIELD_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(FieldType { syntax })
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
pub struct FuncType<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> FuncType<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn params(&self) -> AstChildren<'a, Param<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn results(&self) -> AstChildren<'a, Result<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for FuncType<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::FUNC_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GlobalType<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> GlobalType<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn mut_keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn val_type(&self) -> Option<ValType<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for GlobalType<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::GLOBAL_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HeapType<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> HeapType<'a> {
    #[inline]
    pub fn type_keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::TYPE_KEYWORD)
    }
    #[inline]
    pub fn index(&self) -> Option<Index<'a>> {
        child(&self.syntax)
    }
}
impl<'a> AstNode<'a> for HeapType<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::HEAP_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Limits<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> Limits<'a> {
    #[inline]
    pub fn min(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::UNSIGNED_INT)
    }
    #[inline]
    pub fn max(&self) -> Option<SyntaxToken<'a>> {
        self.syntax.tokens_by_kind(SyntaxKind::UNSIGNED_INT).nth(1)
    }
}
impl<'a> AstNode<'a> for Limits<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::LIMITS
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MemPageSize<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> MemPageSize<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn unsigned_int_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::UNSIGNED_INT)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for MemPageSize<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MEM_PAGE_SIZE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(MemPageSize { syntax })
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
pub struct MemType<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> MemType<'a> {
    #[inline]
    pub fn addr_type(&self) -> Option<AddrType<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn limits(&self) -> Option<Limits<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn share_keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn mem_page_size(&self) -> Option<MemPageSize<'a>> {
        child(&self.syntax)
    }
}
impl<'a> AstNode<'a> for MemType<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MEM_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(MemType { syntax })
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
pub struct NumType<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> NumType<'a> {
    #[inline]
    pub fn type_keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::TYPE_KEYWORD)
    }
}
impl<'a> AstNode<'a> for NumType<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::NUM_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(NumType { syntax })
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
pub struct PackedType<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> PackedType<'a> {
    #[inline]
    pub fn type_keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::TYPE_KEYWORD)
    }
}
impl<'a> AstNode<'a> for PackedType<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::PACKED_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(PackedType { syntax })
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
pub struct Param<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> Param<'a> {
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
    pub fn val_types(&self) -> AstChildren<'a, ValType<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for Param<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::PARAM
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefType<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> RefType<'a> {
    #[inline]
    pub fn type_keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::TYPE_KEYWORD)
    }
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn null_keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::MODIFIER_KEYWORD)
    }
    #[inline]
    pub fn heap_type(&self) -> Option<HeapType<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for RefType<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::REF_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Result<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> Result<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn val_types(&self) -> AstChildren<'a, ValType<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for Result<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::RESULT
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StorageType<'a> {
    Val(ValType<'a>),
    Packed(PackedType<'a>),
}
impl<'a> AstNode<'a> for StorageType<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            SyntaxKind::NUM_TYPE | SyntaxKind::VEC_TYPE | SyntaxKind::REF_TYPE | SyntaxKind::PACKED_TYPE
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind() {
            SyntaxKind::NUM_TYPE => Some(StorageType::Val(ValType::Num(NumType { syntax }))),
            SyntaxKind::VEC_TYPE => Some(StorageType::Val(ValType::Vec(VecType { syntax }))),
            SyntaxKind::REF_TYPE => Some(StorageType::Val(ValType::Ref(RefType { syntax }))),
            SyntaxKind::PACKED_TYPE => Some(StorageType::Packed(PackedType { syntax })),
            _ => None,
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        match self {
            StorageType::Val(it) => it.syntax(),
            StorageType::Packed(it) => it.syntax(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructType<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> StructType<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn fields(&self) -> AstChildren<'a, Field<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for StructType<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::STRUCT_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(StructType { syntax })
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
pub struct SubType<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> SubType<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn final_keyword(&self) -> Option<SyntaxToken<'a>> {
        self.syntax.tokens_by_kind(SyntaxKind::MODIFIER_KEYWORD).next()
    }
    #[inline]
    pub fn indexes(&self) -> AstChildren<'a, Index<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn comp_type(&self) -> Option<CompType<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for SubType<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::SUB_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(SubType { syntax })
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
pub struct TableType<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> TableType<'a> {
    #[inline]
    pub fn addr_type(&self) -> Option<AddrType<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn limits(&self) -> Option<Limits<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn ref_type(&self) -> Option<RefType<'a>> {
        child(&self.syntax)
    }
}
impl<'a> AstNode<'a> for TableType<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::TABLE_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValType<'a> {
    Num(NumType<'a>),
    Vec(VecType<'a>),
    Ref(RefType<'a>),
}
impl<'a> AstNode<'a> for ValType<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(kind, SyntaxKind::NUM_TYPE | SyntaxKind::VEC_TYPE | SyntaxKind::REF_TYPE)
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind() {
            SyntaxKind::NUM_TYPE => Some(ValType::Num(NumType { syntax })),
            SyntaxKind::VEC_TYPE => Some(ValType::Vec(VecType { syntax })),
            SyntaxKind::REF_TYPE => Some(ValType::Ref(RefType { syntax })),
            _ => None,
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        match self {
            ValType::Num(it) => it.syntax(),
            ValType::Vec(it) => it.syntax(),
            ValType::Ref(it) => it.syntax(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VecType<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> VecType<'a> {
    #[inline]
    pub fn type_keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::TYPE_KEYWORD)
    }
}
impl<'a> AstNode<'a> for VecType<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::VEC_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(VecType { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}
