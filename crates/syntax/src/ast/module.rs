use super::{
    SyntaxKind, SyntaxNode, SyntaxToken, WatLanguage,
    instr::Instr,
    ty::{ExternType, GlobalType, MemoryType, Param, RefType, Result, SubType, TableType, ValType},
};
use rowan::{
    NodeOrToken,
    ast::{
        AstChildren, AstNode,
        support::{child, children, token},
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Data {
    syntax: SyntaxNode,
}
impl Data {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn string_tokens(&self) -> impl Iterator<Item = SyntaxToken> {
        self.syntax
            .children_with_tokens()
            .filter_map(|it| match it {
                NodeOrToken::Token(it) if it.kind() == SyntaxKind::STRING => Some(it),
                _ => None,
            })
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for Data {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::DATA
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Elem {
    syntax: SyntaxNode,
}
impl Elem {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn elem_exprs(&self) -> AstChildren<ElemExpr> {
        children(&self.syntax)
    }
    #[inline]
    pub fn indexes(&self) -> AstChildren<Index> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for Elem {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::ELEM
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElemExpr {
    syntax: SyntaxNode,
}
impl ElemExpr {
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
impl AstNode for ElemExpr {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::ELEM_EXPR
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElemList {
    syntax: SyntaxNode,
}
impl ElemList {
    #[inline]
    pub fn func_keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn indexes(&self) -> AstChildren<Index> {
        children(&self.syntax)
    }
    #[inline]
    pub fn ref_type(&self) -> Option<RefType> {
        child(&self.syntax)
    }
    #[inline]
    pub fn elem_exprs(&self) -> AstChildren<ElemExpr> {
        children(&self.syntax)
    }
}
impl AstNode for ElemList {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::ELEM_LIST
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Export {
    syntax: SyntaxNode,
}
impl Export {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn name(&self) -> Option<Name> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for Export {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::EXPORT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExternIdx {
    Func(ExternIdxFunc),
    Global(ExternIdxGlobal),
    Memory(ExternIdxMemory),
    Table(ExternIdxTable),
    Tag(ExternIdxTag),
}
impl AstNode for ExternIdx {
    type Language = WatLanguage;
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
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
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
pub struct ExternIdxFunc {
    syntax: SyntaxNode,
}
impl ExternIdxFunc {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn index(&self) -> Option<Index> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for ExternIdxFunc {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::EXTERN_IDX_FUNC
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternIdxGlobal {
    syntax: SyntaxNode,
}
impl ExternIdxGlobal {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn index(&self) -> Option<Index> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for ExternIdxGlobal {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::EXTERN_IDX_GLOBAL
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternIdxMemory {
    syntax: SyntaxNode,
}
impl ExternIdxMemory {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn index(&self) -> Option<Index> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for ExternIdxMemory {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::EXTERN_IDX_MEMORY
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternIdxTable {
    syntax: SyntaxNode,
}
impl ExternIdxTable {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn index(&self) -> Option<Index> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for ExternIdxTable {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::EXTERN_IDX_TABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternIdxTag {
    syntax: SyntaxNode,
}
impl ExternIdxTag {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn index(&self) -> Option<Index> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for ExternIdxTag {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::EXTERN_IDX_TAG
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Import {
    syntax: SyntaxNode,
}
impl Import {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn module_name(&self) -> Option<ModuleName> {
        child(&self.syntax)
    }
    #[inline]
    pub fn name(&self) -> Option<Name> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for Import {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::IMPORT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Index {
    syntax: SyntaxNode,
}
impl Index {
    #[inline]
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn unsigned_int_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::UNSIGNED_INT)
    }
}
impl AstNode for Index {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::INDEX
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Local {
    syntax: SyntaxNode,
}
impl Local {
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
impl AstNode for Local {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::LOCAL
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MemUse {
    syntax: SyntaxNode,
}
impl MemUse {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn index(&self) -> Option<Index> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for MemUse {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MEM_USE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
    syntax: SyntaxNode,
}
impl Module {
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
    pub fn module_fields(&self) -> AstChildren<ModuleField> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for Module {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModuleField {
    Data(ModuleFieldData),
    Elem(ModuleFieldElem),
    Export(ModuleFieldExport),
    Func(ModuleFieldFunc),
    Global(ModuleFieldGlobal),
    Import(ModuleFieldImport),
    Memory(ModuleFieldMemory),
    Start(ModuleFieldStart),
    Table(ModuleFieldTable),
    Tag(ModuleFieldTag),
    Type(TypeDef),
    RecType(RecType),
}
impl AstNode for ModuleField {
    type Language = WatLanguage;
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
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind() {
            SyntaxKind::MODULE_FIELD_DATA => Some(ModuleField::Data(ModuleFieldData { syntax })),
            SyntaxKind::MODULE_FIELD_ELEM => Some(ModuleField::Elem(ModuleFieldElem { syntax })),
            SyntaxKind::MODULE_FIELD_EXPORT => {
                Some(ModuleField::Export(ModuleFieldExport { syntax }))
            }
            SyntaxKind::MODULE_FIELD_FUNC => Some(ModuleField::Func(ModuleFieldFunc { syntax })),
            SyntaxKind::MODULE_FIELD_GLOBAL => {
                Some(ModuleField::Global(ModuleFieldGlobal { syntax }))
            }
            SyntaxKind::MODULE_FIELD_IMPORT => {
                Some(ModuleField::Import(ModuleFieldImport { syntax }))
            }
            SyntaxKind::MODULE_FIELD_MEMORY => {
                Some(ModuleField::Memory(ModuleFieldMemory { syntax }))
            }
            SyntaxKind::MODULE_FIELD_START => Some(ModuleField::Start(ModuleFieldStart { syntax })),
            SyntaxKind::MODULE_FIELD_TABLE => Some(ModuleField::Table(ModuleFieldTable { syntax })),
            SyntaxKind::MODULE_FIELD_TAG => Some(ModuleField::Tag(ModuleFieldTag { syntax })),
            SyntaxKind::TYPE_DEF => Some(ModuleField::Type(TypeDef { syntax })),
            SyntaxKind::REC_TYPE => Some(ModuleField::RecType(RecType { syntax })),
            _ => None,
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
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
pub struct ModuleFieldData {
    syntax: SyntaxNode,
}
impl ModuleFieldData {
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
    pub fn mem_use(&self) -> Option<MemUse> {
        child(&self.syntax)
    }
    #[inline]
    pub fn offset(&self) -> Option<Offset> {
        child(&self.syntax)
    }
    #[inline]
    pub fn string_tokens(&self) -> impl Iterator<Item = SyntaxToken> {
        self.syntax
            .children_with_tokens()
            .filter_map(|it| match it {
                NodeOrToken::Token(it) if it.kind() == SyntaxKind::STRING => Some(it),
                _ => None,
            })
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for ModuleFieldData {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_FIELD_DATA
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleFieldElem {
    syntax: SyntaxNode,
}
impl ModuleFieldElem {
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
    pub fn declare_keyword(&self) -> Option<SyntaxToken> {
        self.syntax
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .find(|it| it.kind() == SyntaxKind::KEYWORD && it.text() == "declare")
    }
    #[inline]
    pub fn table_use(&self) -> Option<TableUse> {
        child(&self.syntax)
    }
    #[inline]
    pub fn offset(&self) -> Option<Offset> {
        child(&self.syntax)
    }
    #[inline]
    pub fn elem_list(&self) -> Option<ElemList> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for ModuleFieldElem {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_FIELD_ELEM
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleFieldExport {
    syntax: SyntaxNode,
}
impl ModuleFieldExport {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn name(&self) -> Option<Name> {
        child(&self.syntax)
    }
    #[inline]
    pub fn extern_idx(&self) -> Option<ExternIdx> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for ModuleFieldExport {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_FIELD_EXPORT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleFieldFunc {
    syntax: SyntaxNode,
}
impl ModuleFieldFunc {
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
    pub fn exports(&self) -> AstChildren<Export> {
        children(&self.syntax)
    }
    #[inline]
    pub fn import(&self) -> Option<Import> {
        child(&self.syntax)
    }
    #[inline]
    pub fn type_use(&self) -> Option<TypeUse> {
        child(&self.syntax)
    }
    #[inline]
    pub fn locals(&self) -> AstChildren<Local> {
        children(&self.syntax)
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
impl AstNode for ModuleFieldFunc {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_FIELD_FUNC
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleFieldGlobal {
    syntax: SyntaxNode,
}
impl ModuleFieldGlobal {
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
    pub fn exports(&self) -> AstChildren<Export> {
        children(&self.syntax)
    }
    #[inline]
    pub fn import(&self) -> Option<Import> {
        child(&self.syntax)
    }
    #[inline]
    pub fn global_type(&self) -> Option<GlobalType> {
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
impl AstNode for ModuleFieldGlobal {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_FIELD_GLOBAL
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleFieldImport {
    syntax: SyntaxNode,
}
impl ModuleFieldImport {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn module_name(&self) -> Option<ModuleName> {
        child(&self.syntax)
    }
    #[inline]
    pub fn name(&self) -> Option<Name> {
        child(&self.syntax)
    }
    #[inline]
    pub fn extern_type(&self) -> Option<ExternType> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for ModuleFieldImport {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_FIELD_IMPORT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleFieldMemory {
    syntax: SyntaxNode,
}
impl ModuleFieldMemory {
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
    pub fn exports(&self) -> AstChildren<Export> {
        children(&self.syntax)
    }
    #[inline]
    pub fn import(&self) -> Option<Import> {
        child(&self.syntax)
    }
    #[inline]
    pub fn memory_type(&self) -> Option<MemoryType> {
        child(&self.syntax)
    }
    #[inline]
    pub fn data(&self) -> Option<Data> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for ModuleFieldMemory {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_FIELD_MEMORY
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleFieldStart {
    syntax: SyntaxNode,
}
impl ModuleFieldStart {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn index(&self) -> Option<Index> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for ModuleFieldStart {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_FIELD_START
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleFieldTable {
    syntax: SyntaxNode,
}
impl ModuleFieldTable {
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
    pub fn exports(&self) -> AstChildren<Export> {
        children(&self.syntax)
    }
    #[inline]
    pub fn import(&self) -> Option<Import> {
        child(&self.syntax)
    }
    #[inline]
    pub fn table_type(&self) -> Option<TableType> {
        child(&self.syntax)
    }
    #[inline]
    pub fn ref_type(&self) -> Option<RefType> {
        child(&self.syntax)
    }
    #[inline]
    pub fn elem(&self) -> Option<Elem> {
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
impl AstNode for ModuleFieldTable {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_FIELD_TABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleFieldTag {
    syntax: SyntaxNode,
}
impl ModuleFieldTag {
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
    pub fn exports(&self) -> AstChildren<Export> {
        children(&self.syntax)
    }
    #[inline]
    pub fn import(&self) -> Option<Import> {
        child(&self.syntax)
    }
    #[inline]
    pub fn type_use(&self) -> Option<TypeUse> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for ModuleFieldTag {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_FIELD_TAG
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleName {
    syntax: SyntaxNode,
}
impl ModuleName {
    #[inline]
    pub fn string_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::STRING)
    }
}
impl AstNode for ModuleName {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_NAME
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    syntax: SyntaxNode,
}
impl Name {
    #[inline]
    pub fn string_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::STRING)
    }
}
impl AstNode for Name {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::NAME
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Offset {
    syntax: SyntaxNode,
}
impl Offset {
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
impl AstNode for Offset {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::OFFSET
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecType {
    syntax: SyntaxNode,
}
impl RecType {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn type_defs(&self) -> AstChildren<TypeDef> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for RecType {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::REC_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableUse {
    syntax: SyntaxNode,
}
impl TableUse {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn index(&self) -> Option<Index> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for TableUse {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::TABLE_USE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeDef {
    syntax: SyntaxNode,
}
impl TypeDef {
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
    pub fn sub_type(&self) -> Option<SubType> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for TypeDef {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::TYPE_DEF
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeUse {
    syntax: SyntaxNode,
}
impl TypeUse {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn index(&self) -> Option<Index> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn params(&self) -> AstChildren<Param> {
        children(&self.syntax)
    }
    #[inline]
    pub fn results(&self) -> AstChildren<Result> {
        children(&self.syntax)
    }
}
impl AstNode for TypeUse {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::TYPE_USE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
