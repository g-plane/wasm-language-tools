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
    TYPE_KEYWORD,
    MODIFIER_KEYWORD,
    EQ,
    MEM_ARG_KEYWORD,
    SHAPE_DESCRIPTOR,
    ANNOT_START,
    ANNOT_ELEM,
    ANNOT_END,
    ERROR,

    // SyntaxNode
    MODULE_NAME,
    NAME,
    NUM_TYPE,
    VEC_TYPE,
    REF_TYPE,
    HEAP_TYPE,
    PACKED_TYPE,
    FIELD_TYPE,
    STRUCT_TYPE,
    ARRAY_TYPE,
    FUNC_TYPE,
    PARAM,
    RESULT,
    FIELD,
    SUB_TYPE,
    TABLE_TYPE,
    MEM_TYPE,
    ADDR_TYPE,
    GLOBAL_TYPE,
    BLOCK_TYPE,
    PLAIN_INSTR,
    BLOCK_BLOCK,
    BLOCK_LOOP,
    BLOCK_IF,
    BLOCK_IF_THEN,
    BLOCK_IF_ELSE,
    BLOCK_TRY_TABLE,
    CATCH,
    CATCH_ALL,
    MEM_ARG,
    IMMEDIATE,
    TYPE_USE,
    LIMITS,
    IMPORT,
    EXPORT,
    EXTERN_TYPE_FUNC,
    EXTERN_TYPE_TABLE,
    EXTERN_TYPE_MEMORY,
    EXTERN_TYPE_GLOBAL,
    EXTERN_TYPE_TAG,
    EXTERN_IDX_FUNC,
    EXTERN_IDX_TABLE,
    EXTERN_IDX_MEMORY,
    EXTERN_IDX_GLOBAL,
    EXTERN_IDX_TAG,
    INDEX,
    LOCAL,
    MEM_PAGE_SIZE,
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
    MODULE_FIELD_TAG,
    TYPE_DEF,
    REC_TYPE,
    ROOT,
}

impl SyntaxKind {
    #[inline]
    /// Checks if it is whitespace or comment or annotation.
    pub fn is_trivia(self) -> bool {
        matches!(
            self,
            SyntaxKind::WHITESPACE
                | SyntaxKind::LINE_COMMENT
                | SyntaxKind::BLOCK_COMMENT
                | SyntaxKind::ANNOT_START
                | SyntaxKind::ANNOT_ELEM
                | SyntaxKind::ANNOT_END
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
