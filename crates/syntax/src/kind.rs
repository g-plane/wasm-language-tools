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
