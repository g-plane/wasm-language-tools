use super::{GreenElement, node};
use std::sync::LazyLock;
use wat_syntax::{GreenNode, GreenToken, SyntaxKind};

pub static L_PAREN: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::L_PAREN, "(").into());
pub static R_PAREN: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::R_PAREN, ")").into());
pub static EQ: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::EQ, "=").into());

pub static SINGLE_SPACE: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::WHITESPACE, " ").into());
pub static INDENT: LazyLock<Vec<GreenElement>> = LazyLock::new(|| {
    (0..501)
        .map(|i| {
            let mut s = String::with_capacity(i * 2 + 1);
            s.push('\n');
            for _ in 0..i {
                s.push_str("  ");
            }
            GreenToken::new(SyntaxKind::WHITESPACE, &s).into()
        })
        .collect()
});

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
pub static KW_CONT: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "cont").into());
pub static KW_FIELD: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "field").into());
pub static KW_REF: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "ref").into());
pub static KW_ITEM: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "item").into());
pub static KW_OFFSET: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "offset").into());
pub static KW_ON: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::KEYWORD, "on").into());

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

pub static MEM_ARG_KW_OFFSET: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::MEM_ARG_KEYWORD, "offset").into());
pub static MEM_ARG_KW_ALIGN: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::MEM_ARG_KEYWORD, "align").into());

pub static INSTR_LOCAL_GET: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::INSTR_NAME, "local.get").into());
pub static INSTR_LOCAL_SET: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::INSTR_NAME, "local.set").into());
pub static INSTR_LOCAL_TEE: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::INSTR_NAME, "local.tee").into());
pub static INSTR_CALL: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::INSTR_NAME, "call").into());
pub static INSTR_BR: LazyLock<GreenElement> = LazyLock::new(|| GreenToken::new(SyntaxKind::INSTR_NAME, "br").into());
pub static INSTR_BR_IF: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::INSTR_NAME, "br_if").into());
pub static INSTR_I32_CONST: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::INSTR_NAME, "i32.const").into());
pub static INSTR_I32_ADD: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::INSTR_NAME, "i32.add").into());
pub static INSTR_I32_LOAD: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::INSTR_NAME, "i32.load").into());
pub static INSTR_I32_STORE: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::INSTR_NAME, "i32.store").into());
pub static INSTR_I64_CONST: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::INSTR_NAME, "i64.const").into());
pub static INSTR_I64_LOAD: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::INSTR_NAME, "i64.load").into());
pub static INSTR_I64_STORE: LazyLock<GreenElement> =
    LazyLock::new(|| GreenToken::new(SyntaxKind::INSTR_NAME, "i64.store").into());

pub static IMMEDIATE_INT_NEG_ONE: LazyLock<GreenNode> =
    LazyLock::new(|| GreenNode::new(SyntaxKind::IMMEDIATE, [GreenToken::new(SyntaxKind::INT, "-1").into()]));
pub static IMMEDIATE_INT: LazyLock<Vec<GreenNode>> = LazyLock::new(|| {
    (0..=255u8)
        .map(|i| {
            GreenNode::new(
                SyntaxKind::IMMEDIATE,
                [GreenToken::new(SyntaxKind::INT, &format!("{i}")).into()],
            )
        })
        .collect()
});

pub static LOCAL_GET: LazyLock<Vec<GreenNode>> = LazyLock::new(|| {
    IMMEDIATE_INT
        .iter()
        .map(|i| {
            GreenNode::new(
                SyntaxKind::PLAIN_INSTR,
                [INSTR_LOCAL_GET.clone(), SINGLE_SPACE.clone(), i.clone().into()],
            )
        })
        .collect()
});
pub static LOCAL_SET: LazyLock<Vec<GreenNode>> = LazyLock::new(|| {
    IMMEDIATE_INT
        .iter()
        .map(|i| {
            GreenNode::new(
                SyntaxKind::PLAIN_INSTR,
                [INSTR_LOCAL_SET.clone(), SINGLE_SPACE.clone(), i.clone().into()],
            )
        })
        .collect()
});
pub static LOCAL_TEE: LazyLock<Vec<GreenNode>> = LazyLock::new(|| {
    IMMEDIATE_INT
        .iter()
        .map(|i| {
            GreenNode::new(
                SyntaxKind::PLAIN_INSTR,
                [INSTR_LOCAL_TEE.clone(), SINGLE_SPACE.clone(), i.clone().into()],
            )
        })
        .collect()
});
