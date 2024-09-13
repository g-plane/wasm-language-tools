use super::{children, token, AstChildren, AstNode, SyntaxKind, SyntaxNode, SyntaxToken};

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
    pub fn module_fields(&self) -> AstChildren<ModuleField> {
        children(&self.syntax)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        token(&self.syntax, SyntaxKind::R_PAREN)
    }
}
impl AstNode for Module {
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
    Type(ModuleFieldType),
}
impl AstNode for ModuleField {
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
                | SyntaxKind::MODULE_FIELD_TYPE
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
            SyntaxKind::MODULE_FIELD_TYPE => Some(ModuleField::Type(ModuleFieldType { syntax })),
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
            ModuleField::Type(it) => it.syntax(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleFieldData {
    syntax: SyntaxNode,
}
impl AstNode for ModuleFieldData {
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
impl AstNode for ModuleFieldElem {
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
impl AstNode for ModuleFieldExport {
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
impl AstNode for ModuleFieldFunc {
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
impl AstNode for ModuleFieldGlobal {
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
impl AstNode for ModuleFieldImport {
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
impl AstNode for ModuleFieldMemory {
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
impl AstNode for ModuleFieldStart {
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
impl AstNode for ModuleFieldTable {
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
pub struct ModuleFieldType {
    syntax: SyntaxNode,
}
impl AstNode for ModuleFieldType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MODULE_FIELD_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(ModuleFieldType { syntax })
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
