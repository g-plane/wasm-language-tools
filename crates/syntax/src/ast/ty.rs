use super::{SyntaxKind, SyntaxNode, SyntaxToken, WatLanguage, module::Index};
use crate::SyntaxElement;
use rowan::{
    NodeOrToken,
    ast::{
        AstChildren, AstNode,
        support::{child, children, token},
    },
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
        kind == SyntaxKind::ARRAY_TYPE
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
        kind == SyntaxKind::FIELD_TYPE
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
    pub fn type_keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::TYPE_KEYWORD)
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
pub struct NumType {
    syntax: SyntaxNode,
}
impl NumType {
    #[inline]
    pub fn type_keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::TYPE_KEYWORD)
    }
}
impl AstNode for NumType {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::NUM_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PackedType {
    syntax: SyntaxNode,
}
impl PackedType {
    #[inline]
    pub fn type_keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::TYPE_KEYWORD)
    }
}
impl AstNode for PackedType {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::PACKED_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    pub fn type_keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::TYPE_KEYWORD)
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
pub enum StorageType {
    Val(ValType),
    Packed(PackedType),
}
impl AstNode for StorageType {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            SyntaxKind::NUM_TYPE
                | SyntaxKind::VEC_TYPE
                | SyntaxKind::REF_TYPE
                | SyntaxKind::PACKED_TYPE
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        match self {
            StorageType::Val(it) => it.syntax(),
            StorageType::Packed(it) => it.syntax(),
        }
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
        kind == SyntaxKind::STRUCT_TYPE
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
        kind == SyntaxKind::SUB_TYPE
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
pub enum ValType {
    Num(NumType),
    Vec(VecType),
    Ref(RefType),
}
impl AstNode for ValType {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            SyntaxKind::NUM_TYPE | SyntaxKind::VEC_TYPE | SyntaxKind::REF_TYPE
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        match self {
            ValType::Num(it) => it.syntax(),
            ValType::Vec(it) => it.syntax(),
            ValType::Ref(it) => it.syntax(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VecType {
    syntax: SyntaxNode,
}
impl VecType {
    #[inline]
    pub fn type_keyword(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::TYPE_KEYWORD)
    }
}
impl AstNode for VecType {
    type Language = WatLanguage;
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::VEC_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
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
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
