use super::{
    AstChildren, AstNode,
    instr::Instr,
    support::*,
    ty::{ExternType, GlobalType, MemType, Param, RefType, Result, SubType, TableType, ValType},
};
use crate::{SyntaxKind, SyntaxNode, SyntaxToken};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Data<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> Data<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn string_tokens(&self) -> impl Iterator<Item = SyntaxToken<'a>> {
        self.syntax.tokens_by_kind(SyntaxKind::STRING)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for Data<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::DATA
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(Data { syntax })
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
pub struct Elem<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> Elem<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn elem_exprs(&self) -> AstChildren<'a, ElemExpr<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn indexes(&self) -> AstChildren<'a, Index<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for Elem<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::ELEM
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(Elem { syntax })
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
pub struct ElemExpr<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ElemExpr<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
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
impl<'a> AstNode<'a> for ElemExpr<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::ELEM_EXPR
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ElemExpr { syntax })
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
pub struct ElemList<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ElemList<'a> {
    #[inline]
    pub fn func_keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn indexes(&self) -> AstChildren<'a, Index<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn ref_type(&self) -> Option<RefType<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn elem_exprs(&self) -> AstChildren<'a, ElemExpr<'a>> {
        children(&self.syntax)
    }
}
impl<'a> AstNode<'a> for ElemList<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::ELEM_LIST
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ElemList { syntax })
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
pub struct Export<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> Export<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn name(&self) -> Option<Name<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for Export<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::EXPORT
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(Export { syntax })
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
pub enum ExternIdx<'a> {
    Func(ExternIdxFunc<'a>),
    Global(ExternIdxGlobal<'a>),
    Memory(ExternIdxMemory<'a>),
    Table(ExternIdxTable<'a>),
    Tag(ExternIdxTag<'a>),
}
impl<'a> AstNode<'a> for ExternIdx<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            SyntaxKind::EXTERN_IDX_FUNC
                | SyntaxKind::EXTERN_IDX_GLOBAL
                | SyntaxKind::EXTERN_IDX_MEMORY
                | SyntaxKind::EXTERN_IDX_TABLE
                | SyntaxKind::EXTERN_IDX_TAG
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind() {
            SyntaxKind::EXTERN_IDX_FUNC => Some(ExternIdx::Func(ExternIdxFunc { syntax })),
            SyntaxKind::EXTERN_IDX_GLOBAL => Some(ExternIdx::Global(ExternIdxGlobal { syntax })),
            SyntaxKind::EXTERN_IDX_MEMORY => Some(ExternIdx::Memory(ExternIdxMemory { syntax })),
            SyntaxKind::EXTERN_IDX_TABLE => Some(ExternIdx::Table(ExternIdxTable { syntax })),
            SyntaxKind::EXTERN_IDX_TAG => Some(ExternIdx::Tag(ExternIdxTag { syntax })),
            _ => None,
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        match self {
            ExternIdx::Func(it) => it.syntax(),
            ExternIdx::Global(it) => it.syntax(),
            ExternIdx::Memory(it) => it.syntax(),
            ExternIdx::Table(it) => it.syntax(),
            ExternIdx::Tag(it) => it.syntax(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternIdxFunc<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ExternIdxFunc<'a> {
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
impl<'a> AstNode<'a> for ExternIdxFunc<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::EXTERN_IDX_FUNC
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ExternIdxFunc { syntax })
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
pub struct ExternIdxGlobal<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ExternIdxGlobal<'a> {
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
impl<'a> AstNode<'a> for ExternIdxGlobal<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::EXTERN_IDX_GLOBAL
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ExternIdxGlobal { syntax })
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
pub struct ExternIdxMemory<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ExternIdxMemory<'a> {
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
impl<'a> AstNode<'a> for ExternIdxMemory<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::EXTERN_IDX_MEMORY
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ExternIdxMemory { syntax })
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
pub struct ExternIdxTable<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ExternIdxTable<'a> {
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
impl<'a> AstNode<'a> for ExternIdxTable<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::EXTERN_IDX_TABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ExternIdxTable { syntax })
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
pub struct ExternIdxTag<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ExternIdxTag<'a> {
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
impl<'a> AstNode<'a> for ExternIdxTag<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::EXTERN_IDX_TAG
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ExternIdxTag { syntax })
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
pub struct Import<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> Import<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn module_name(&self) -> Option<ModuleName<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn name(&self) -> Option<Name<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for Import<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::IMPORT
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(Import { syntax })
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
pub struct ImportItem<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ImportItem<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn name(&self) -> Option<Name<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn extern_type(&self) -> Option<ExternType<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for ImportItem<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::IMPORT_ITEM
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ImportItem { syntax })
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
pub struct Index<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> Index<'a> {
    #[inline]
    pub fn ident_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn unsigned_int_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::UNSIGNED_INT)
    }
}
impl<'a> AstNode<'a> for Index<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::INDEX
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(Index { syntax })
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
pub struct Local<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> Local<'a> {
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
impl<'a> AstNode<'a> for Local<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::LOCAL
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(Local { syntax })
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
pub struct MemUse<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> MemUse<'a> {
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
impl<'a> AstNode<'a> for MemUse<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MEM_USE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(MemUse { syntax })
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
pub struct Module<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> Module<'a> {
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
    pub fn module_fields(&self) -> AstChildren<'a, ModuleField<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for Module<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(Module { syntax })
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
pub enum ModuleField<'a> {
    Data(ModuleFieldData<'a>),
    Elem(ModuleFieldElem<'a>),
    Export(ModuleFieldExport<'a>),
    Func(ModuleFieldFunc<'a>),
    Global(ModuleFieldGlobal<'a>),
    Import(ModuleFieldImport<'a>),
    Memory(ModuleFieldMemory<'a>),
    Start(ModuleFieldStart<'a>),
    Table(ModuleFieldTable<'a>),
    Tag(ModuleFieldTag<'a>),
    Type(TypeDef<'a>),
    RecType(RecType<'a>),
}
impl<'a> AstNode<'a> for ModuleField<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            SyntaxKind::MODULE_FIELD_DATA
                | SyntaxKind::MODULE_FIELD_ELEM
                | SyntaxKind::MODULE_FIELD_EXPORT
                | SyntaxKind::MODULE_FIELD_FUNC
                | SyntaxKind::MODULE_FIELD_GLOBAL
                | SyntaxKind::MODULE_FIELD_IMPORT
                | SyntaxKind::MODULE_FIELD_MEMORY
                | SyntaxKind::MODULE_FIELD_START
                | SyntaxKind::MODULE_FIELD_TABLE
                | SyntaxKind::MODULE_FIELD_TAG
                | SyntaxKind::TYPE_DEF
                | SyntaxKind::REC_TYPE
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind() {
            SyntaxKind::MODULE_FIELD_DATA => Some(ModuleField::Data(ModuleFieldData { syntax })),
            SyntaxKind::MODULE_FIELD_ELEM => Some(ModuleField::Elem(ModuleFieldElem { syntax })),
            SyntaxKind::MODULE_FIELD_EXPORT => Some(ModuleField::Export(ModuleFieldExport { syntax })),
            SyntaxKind::MODULE_FIELD_FUNC => Some(ModuleField::Func(ModuleFieldFunc { syntax })),
            SyntaxKind::MODULE_FIELD_GLOBAL => Some(ModuleField::Global(ModuleFieldGlobal { syntax })),
            SyntaxKind::MODULE_FIELD_IMPORT => Some(ModuleField::Import(ModuleFieldImport { syntax })),
            SyntaxKind::MODULE_FIELD_MEMORY => Some(ModuleField::Memory(ModuleFieldMemory { syntax })),
            SyntaxKind::MODULE_FIELD_START => Some(ModuleField::Start(ModuleFieldStart { syntax })),
            SyntaxKind::MODULE_FIELD_TABLE => Some(ModuleField::Table(ModuleFieldTable { syntax })),
            SyntaxKind::MODULE_FIELD_TAG => Some(ModuleField::Tag(ModuleFieldTag { syntax })),
            SyntaxKind::TYPE_DEF => Some(ModuleField::Type(TypeDef { syntax })),
            SyntaxKind::REC_TYPE => Some(ModuleField::RecType(RecType { syntax })),
            _ => None,
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        match self {
            ModuleField::Data(it) => it.syntax(),
            ModuleField::Elem(it) => it.syntax(),
            ModuleField::Export(it) => it.syntax(),
            ModuleField::Func(it) => it.syntax(),
            ModuleField::Global(it) => it.syntax(),
            ModuleField::Import(it) => it.syntax(),
            ModuleField::Memory(it) => it.syntax(),
            ModuleField::Start(it) => it.syntax(),
            ModuleField::Table(it) => it.syntax(),
            ModuleField::Tag(it) => it.syntax(),
            ModuleField::Type(it) => it.syntax(),
            ModuleField::RecType(it) => it.syntax(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleFieldData<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ModuleFieldData<'a> {
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
    pub fn mem_use(&self) -> Option<MemUse<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn offset(&self) -> Option<Offset<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn string_tokens(&self) -> impl Iterator<Item = SyntaxToken<'a>> {
        self.syntax.tokens_by_kind(SyntaxKind::STRING)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for ModuleFieldData<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_FIELD_DATA
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ModuleFieldData { syntax })
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
pub struct ModuleFieldElem<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ModuleFieldElem<'a> {
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
    pub fn declare_keyword(&self) -> Option<SyntaxToken<'a>> {
        self.syntax.tokens_by_kind(SyntaxKind::MODIFIER_KEYWORD).next()
    }
    #[inline]
    pub fn table_use(&self) -> Option<TableUse<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn offset(&self) -> Option<Offset<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn elem_list(&self) -> Option<ElemList<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for ModuleFieldElem<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_FIELD_ELEM
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ModuleFieldElem { syntax })
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
pub struct ModuleFieldExport<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ModuleFieldExport<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn name(&self) -> Option<Name<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn extern_idx(&self) -> Option<ExternIdx<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for ModuleFieldExport<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_FIELD_EXPORT
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ModuleFieldExport { syntax })
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
pub struct ModuleFieldFunc<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ModuleFieldFunc<'a> {
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
    pub fn exports(&self) -> AstChildren<'a, Export<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn import(&self) -> Option<Import<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn type_use(&self) -> Option<TypeUse<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn locals(&self) -> AstChildren<'a, Local<'a>> {
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
impl<'a> AstNode<'a> for ModuleFieldFunc<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_FIELD_FUNC
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ModuleFieldFunc { syntax })
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
pub struct ModuleFieldGlobal<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ModuleFieldGlobal<'a> {
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
    pub fn exports(&self) -> AstChildren<'a, Export<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn import(&self) -> Option<Import<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn global_type(&self) -> Option<GlobalType<'a>> {
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
}
impl<'a> AstNode<'a> for ModuleFieldGlobal<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_FIELD_GLOBAL
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ModuleFieldGlobal { syntax })
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
pub struct ModuleFieldImport<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ModuleFieldImport<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn module_name(&self) -> Option<ModuleName<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn name(&self) -> Option<Name<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn import_items(&self) -> AstChildren<'a, ImportItem<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn extern_type(&self) -> Option<ExternType<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for ModuleFieldImport<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_FIELD_IMPORT
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ModuleFieldImport { syntax })
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
pub struct ModuleFieldMemory<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ModuleFieldMemory<'a> {
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
    pub fn exports(&self) -> AstChildren<'a, Export<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn import(&self) -> Option<Import<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn mem_type(&self) -> Option<MemType<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn data(&self) -> Option<Data<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for ModuleFieldMemory<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_FIELD_MEMORY
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ModuleFieldMemory { syntax })
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
pub struct ModuleFieldStart<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ModuleFieldStart<'a> {
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
impl<'a> AstNode<'a> for ModuleFieldStart<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_FIELD_START
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ModuleFieldStart { syntax })
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
pub struct ModuleFieldTable<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ModuleFieldTable<'a> {
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
    pub fn exports(&self) -> AstChildren<'a, Export<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn import(&self) -> Option<Import<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn table_type(&self) -> Option<TableType<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn ref_type(&self) -> Option<RefType<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn elem(&self) -> Option<Elem<'a>> {
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
}
impl<'a> AstNode<'a> for ModuleFieldTable<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_FIELD_TABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ModuleFieldTable { syntax })
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
pub struct ModuleFieldTag<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ModuleFieldTag<'a> {
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
    pub fn exports(&self) -> AstChildren<'a, Export<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn import(&self) -> Option<Import<'a>> {
        child(&self.syntax)
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
impl<'a> AstNode<'a> for ModuleFieldTag<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_FIELD_TAG
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ModuleFieldTag { syntax })
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
pub struct ModuleName<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> ModuleName<'a> {
    #[inline]
    pub fn string_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::STRING)
    }
}
impl<'a> AstNode<'a> for ModuleName<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_NAME
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ModuleName { syntax })
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
pub struct Name<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> Name<'a> {
    #[inline]
    pub fn string_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::STRING)
    }
}
impl<'a> AstNode<'a> for Name<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::NAME
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(Name { syntax })
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
pub struct Offset<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> Offset<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
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
impl<'a> AstNode<'a> for Offset<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::OFFSET
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(Offset { syntax })
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
pub struct RecType<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> RecType<'a> {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn type_defs(&self) -> AstChildren<'a, TypeDef<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for RecType<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::REC_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(RecType { syntax })
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
pub struct TableUse<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> TableUse<'a> {
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
impl<'a> AstNode<'a> for TableUse<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::TABLE_USE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(TableUse { syntax })
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
pub struct TypeDef<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> TypeDef<'a> {
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
    pub fn sub_type(&self) -> Option<SubType<'a>> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken<'a>> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl<'a> AstNode<'a> for TypeDef<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::TYPE_DEF
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(TypeDef { syntax })
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
pub struct TypeUse<'a> {
    syntax: SyntaxNode<'a>,
}
impl<'a> TypeUse<'a> {
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
    #[inline]
    pub fn params(&self) -> AstChildren<'a, Param<'a>> {
        children(&self.syntax)
    }
    #[inline]
    pub fn results(&self) -> AstChildren<'a, Result<'a>> {
        children(&self.syntax)
    }
}
impl<'a> AstNode<'a> for TypeUse<'a> {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::TYPE_USE
    }
    #[inline]
    fn cast(syntax: SyntaxNode<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(TypeUse { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode<'a> {
        &self.syntax
    }
}
