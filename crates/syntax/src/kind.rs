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
    INSTR_NAME,
    IDENT,
    STRING,
    INT,
    UNSIGNED_INT,
    FLOAT,
    NUM_TYPE,
    VEC_TYPE,
    REF_TYPE,
    HEAP_TYPE,
    SHARE,
    MEM_ARG,
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
    GLOBAL_TYPE,
    BLOCK_TYPE,
    INSTR,
    PLAIN_INSTR,
    BLOCK_INSTR,
    BLOCK_BLOCK,
    BLOCK_LOOP,
    BLOCK_IF,
    OPERAND,
    TYPE_USE,
    LIMITS,
    IMPORT,
    IMPORT_DESC,
    IMPORT_DESC_TYPE_USE,
    IMPORT_DESC_TABLE_TYPE,
    IMPORT_DESC_MEMORY_TYPE,
    IMPORT_DESC_GLOBAL_TYPE,
    EXPORT_DESC,
    EXPORT_DESC_FUNC,
    EXPORT_DESC_TABLE,
    EXPORT_DESC_MEMORY,
    EXPORT_DESC_GLOBAL,
    INDEX,
    MODULE,
    MODULE_FIELD,
    MODULE_FIELD_EXPORT,
    MODULE_FIELD_FUNC,
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
