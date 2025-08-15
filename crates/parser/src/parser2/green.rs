use super::{node, GreenElement};
use rowan::GreenToken;
use std::sync::LazyLock;
use wat_syntax::SyntaxKind;

pub static L_PAREN: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::L_PAREN.into(), "(").into());
pub static R_PAREN: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::R_PAREN.into(), ")").into());

pub static SINGLE_SPACE: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::WHITESPACE.into(), " ").into());

pub static KW_FUNC: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "func").into());
pub static KW_GLOBAL: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "global").into());
pub static KW_TYPE: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "type").into());
pub static KW_MEMORY: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "memory").into());
pub static KW_TABLE: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "table").into());
pub static KW_DATA: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "data").into());
pub static KW_ELEM: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "elem").into());
pub static KW_REC: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "rec").into());
pub static KW_EXPORT: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "export").into());
pub static KW_IMPORT: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "import").into());
pub static KW_PARAM: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "param").into());
pub static KW_RESULT: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "result").into());
pub static KW_LOCAL: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "local").into());
pub static KW_MUT: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "mut").into());
pub static KW_SUB: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "sub").into());
pub static KW_STRUCT: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "struct").into());
pub static KW_ARRAY: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "array").into());
pub static KW_FIELD: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "field").into());
pub static KW_REF: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "ref").into());
pub static KW_ITEM: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "item").into());
pub static KW_OFFSET: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "offset").into());

pub static KW_BLOCK: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "block").into());
pub static KW_LOOP: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "loop").into());
pub static KW_IF: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "if").into());
pub static KW_THEN: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "then").into());
pub static KW_ELSE: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "else").into());
pub static KW_END: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD.into(), "end").into());

pub static TYPE_I32: LazyLock<GreenElement> = LazyLock::new(|| {
    node(
        SyntaxKind::NUM_TYPE,
        [GreenToken::new(SyntaxKind::TYPE_KEYWORD.into(), "i32").into()],
    )
    .into()
});
pub static TYPE_I64: LazyLock<GreenElement> = LazyLock::new(|| {
    node(
        SyntaxKind::NUM_TYPE,
        [GreenToken::new(SyntaxKind::TYPE_KEYWORD.into(), "i64").into()],
    )
    .into()
});
pub static TYPE_F32: LazyLock<GreenElement> = LazyLock::new(|| {
    node(
        SyntaxKind::NUM_TYPE,
        [GreenToken::new(SyntaxKind::TYPE_KEYWORD.into(), "f32").into()],
    )
    .into()
});
pub static TYPE_F64: LazyLock<GreenElement> = LazyLock::new(|| {
    node(
        SyntaxKind::NUM_TYPE,
        [GreenToken::new(SyntaxKind::TYPE_KEYWORD.into(), "f64").into()],
    )
    .into()
});
