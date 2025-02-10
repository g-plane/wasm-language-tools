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
pub struct ArrayType {
    syntax: SyntaxNode,
}
impl ArrayType {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn field_type(&self) -> Option<FieldType> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for ArrayType {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(kind, SyntaxKind::ARRAY_TYPE)
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompType {
    Array(ArrayType),
    Struct(StructType),
    Func(FuncType),
}
impl AstNode for CompType {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            SyntaxKind::ARRAY_TYPE | SyntaxKind::STRUCT_TYPE | SyntaxKind::FUNC_TYPE
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind() {
            SyntaxKind::ARRAY_TYPE => Some(CompType::Array(ArrayType { syntax })),
            SyntaxKind::STRUCT_TYPE => Some(CompType::Struct(StructType { syntax })),
            SyntaxKind::FUNC_TYPE => Some(CompType::Func(FuncType { syntax })),
            _ => None,
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            CompType::Array(it) => it.syntax(),
            CompType::Struct(it) => it.syntax(),
            CompType::Func(it) => it.syntax(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Field {
    syntax: SyntaxNode,
}
impl Field {
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
    pub fn field_types(&self) -> AstChildren<FieldType> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for Field {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::FIELD
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldType {
    syntax: SyntaxNode,
}
impl FieldType {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn mut_keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn storage_type(&self) -> Option<StorageType> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for FieldType {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(kind, SyntaxKind::FIELD_TYPE)
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

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
        matches!(kind, SyntaxKind::REC_TYPE)
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
pub struct StorageType {
    syntax: SyntaxNode,
}
impl StorageType {
    #[inline]
    pub fn val_type(&self) -> Option<ValType> {
        child(&self.syntax)
    }
    #[inline]
    pub fn packed_type(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::PACKED_TYPE)
    }
}
impl AstNode for StorageType {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(kind, SyntaxKind::STORAGE_TYPE)
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(StorageType { syntax })
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
pub struct StructType {
    syntax: SyntaxNode,
}
impl StructType {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn fields(&self) -> AstChildren<Field> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for StructType {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(kind, SyntaxKind::STRUCT_TYPE)
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubType {
    syntax: SyntaxNode,
}
impl SubType {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::KEYWORD)
    }
    #[inline]
    pub fn final_keyword(&self) -> Option<SyntaxToken> {
        self.syntax.children_with_tokens().find_map(|it| match it {
            SyntaxElement::Token(token)
                if token.kind() == SyntaxKind::KEYWORD && token.text() == "final" =>
            {
                Some(token)
            }
            _ => None,
        })
    }
    #[inline]
    pub fn indexes(&self) -> AstChildren<Index> {
        children(&self.syntax)
    }
    #[inline]
    pub fn comp_type(&self) -> Option<CompType> {
        child(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for SubType {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(kind, SyntaxKind::SUB_TYPE)
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
        matches!(kind, SyntaxKind::TYPE_DEF)
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
