#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[expect(non_camel_case_types)]
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
    ABBR_REF_TYPE,
    ABS_HEAP_TYPE,
    SHARE,
    MEM_ARG,
    ERROR,

    // SyntaxNode
    MODULE_NAME,
    NAME,
    REF_TYPE,
    HEAP_TYPE,
    VAL_TYPE,
    FUNC_TYPE,
    PARAM,
    RESULT,
    TABLE_TYPE,
    MEMORY_TYPE,
    GLOBAL_TYPE,
    BLOCK_TYPE,
    PLAIN_INSTR,
    BLOCK_BLOCK,
    BLOCK_LOOP,
    BLOCK_IF,
    BLOCK_IF_THEN,
    BLOCK_IF_ELSE,
    IMMEDIATE,
    TYPE_USE,
    LIMITS,
    IMPORT,
    EXPORT,
    IMPORT_DESC_TYPE_USE,
    IMPORT_DESC_TABLE_TYPE,
    IMPORT_DESC_MEMORY_TYPE,
    IMPORT_DESC_GLOBAL_TYPE,
    EXPORT_DESC_FUNC,
    EXPORT_DESC_TABLE,
    EXPORT_DESC_MEMORY,
    EXPORT_DESC_GLOBAL,
    INDEX,
    LOCAL,
    MEM_USE,
    OFFSET,
    ELEM,
    ELEM_LIST,
    ELEM_EXPR,
    TABLE_USE,
    DATA,
    MODULE,
    MODULE_FIELD_DATA,
    MODULE_FIELD_ELEM,
    MODULE_FIELD_EXPORT,
    MODULE_FIELD_FUNC,
    MODULE_FIELD_GLOBAL,
    MODULE_FIELD_IMPORT,
    MODULE_FIELD_MEMORY,
    MODULE_FIELD_START,
    MODULE_FIELD_TABLE,
    MODULE_FIELD_TYPE,
    ROOT,
}

impl SyntaxKind {
    #[inline]
    /// Checks if it is whitespace or comment.
    pub fn is_trivia(self) -> bool {
        matches!(
            self,
            SyntaxKind::WHITESPACE | SyntaxKind::LINE_COMMENT | SyntaxKind::BLOCK_COMMENT
        )
    }

    #[inline]
    pub fn is_comment(self) -> bool {
        matches!(self, SyntaxKind::LINE_COMMENT | SyntaxKind::BLOCK_COMMENT)
    }

    #[inline]
    /// Checks if it is punctuation.
    pub fn is_punc(self) -> bool {
        matches!(self, SyntaxKind::L_PAREN | SyntaxKind::R_PAREN)
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}
