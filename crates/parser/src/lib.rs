mod parser;

pub use crate::parser::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[repr(u16)]
/// Syntax kind enum for nodes and tokens.
pub enum SyntaxKind {
    // SyntaxToken
    WHITESPACE = 0,
    LINE_COMMENT,
    BLOCK_COMMENT,
    L_PAREN,
    R_PAREN,
    KEYWORD,
    IDENT,
    STRING,
    NUM_TYPE,
    VEC_TYPE,
    REF_TYPE,
    HEAP_TYPE,
    NAT,
    SHARE,
    ERROR,

    // SyntaxNode
    MODULE_NAME,
    NAME,
    VAL_TYPE,
    FUNC_TYPE,
    PARAM,
    RESULT,
    TABLE_TYPE,
    TABLE_USE,
    MEMORY_TYPE,
    TYPE_USE,
    LIMITS,
    IMPORT,
    IMPORT_DESC,
    IMPORT_DESC_TYPE_USE,
    IMPORT_DESC_TABLE_TYPE,
    IMPORT_DESC_MEMORY_TYPE,
    IMPORT_DESC_GLOBAL_TYPE,
    INDEX,
    MODULE,
    MODULE_FIELD,
    MODULE_FIELD_IMPORT,
    MODULE_FIELD_START,
    MODULE_FIELD_TYPE,
    ROOT,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WatLanguage {}
impl rowan::Language for WatLanguage {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::ROOT as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub type SyntaxNode = rowan::SyntaxNode<WatLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<WatLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<WatLanguage>;
