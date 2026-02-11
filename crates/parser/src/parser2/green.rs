use super::{GreenElement, node};
use std::sync::LazyLock;
use wat_syntax::{GreenToken, SyntaxKind};

pub static L_PAREN: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::L_PAREN, "(").into());
pub static R_PAREN: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::R_PAREN, ")").into());

pub static SINGLE_SPACE: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::WHITESPACE, " ").into());

pub static KW_FUNC: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "func").into());
pub static KW_GLOBAL: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "global").into());
pub static KW_TYPE: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "type").into());
pub static KW_MEMORY: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "memory").into());
pub static KW_TABLE: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "table").into());
pub static KW_TAG: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "tag").into());
pub static KW_DATA: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "data").into());
pub static KW_ELEM: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "elem").into());
pub static KW_REC: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "rec").into());
pub static KW_EXPORT: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "export").into());
pub static KW_IMPORT: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "import").into());
pub static KW_PARAM: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "param").into());
pub static KW_RESULT: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "result").into());
pub static KW_LOCAL: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "local").into());
pub static KW_MUT: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "mut").into());
pub static KW_SUB: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "sub").into());
pub static KW_STRUCT: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "struct").into());
pub static KW_ARRAY: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "array").into());
pub static KW_FIELD: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "field").into());
pub static KW_REF: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "ref").into());
pub static KW_ITEM: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "item").into());
pub static KW_OFFSET: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "offset").into());

pub static KW_BLOCK: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "block").into());
pub static KW_LOOP: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "loop").into());
pub static KW_IF: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "if").into());
pub static KW_THEN: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "then").into());
pub static KW_ELSE: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "else").into());
pub static KW_TRY_TABLE: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "try_table").into());
pub static KW_END: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "end").into());

pub static TYPE_KW_I32: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::TYPE_KEYWORD, "i32").into());
pub static TYPE_KW_I64: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::TYPE_KEYWORD, "i64").into());
pub static TYPE_I32: LazyLock<GreenElement> =
    LazyLock::new(|| node(SyntaxKind::NUM_TYPE, [TYPE_KW_I32.clone()]).into());
pub static TYPE_I64: LazyLock<GreenElement> =
    LazyLock::new(|| node(SyntaxKind::NUM_TYPE, [TYPE_KW_I64.clone()]).into());
pub static TYPE_F32: LazyLock<GreenElement> = LazyLock::new(|| {
    node(
        SyntaxKind::NUM_TYPE,
        [GreenToken::new(SyntaxKind::TYPE_KEYWORD, "f32").into()],
    )
    .into()
});
pub static TYPE_F64: LazyLock<GreenElement> = LazyLock::new(|| {
    node(
        SyntaxKind::NUM_TYPE,
        [GreenToken::new(SyntaxKind::TYPE_KEYWORD, "f64").into()],
    )
    .into()
});

pub static MODIFIER_KW_NULL: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::MODIFIER_KEYWORD, "null").into());
