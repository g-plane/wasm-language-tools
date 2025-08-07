use super::{
    module::index,
    must, node, retry, retry_once, tok,
    token::{ident, keyword, l_paren, r_paren, trivias_prefixed, unsigned_int, word},
    GreenElement, GreenResult, Input,
};
use crate::error::Message;
use wat_syntax::SyntaxKind::*;
use winnow::{
    combinator::{alt, dispatch, opt, peek, repeat},
    token::any,
    Parser,
};

fn abbr_ref_type(input: &mut Input) -> GreenResult {
    word.verify_map(try_into_abbr_ref_type)
        .context(Message::Name("ref type"))
        .parse_next(input)
        .map(|ty| node(REF_TYPE, [ty]))
}
fn try_into_abbr_ref_type(word: &str) -> Option<GreenElement> {
    match word {
        "anyref" | "eqref" | "i31ref" | "structref" | "arrayref" | "nullref" | "funcref"
        | "nullfuncref" | "externref" | "nullexternref" => Some(tok(TYPE_KEYWORD, word)),
        _ => None,
    }
}

pub(super) fn ref_type(input: &mut Input) -> GreenResult {
    dispatch! {peek(any);
        '(' => detailed_ref_type,
        _ => abbr_ref_type,
    }
    .context(Message::Name("ref type"))
    .parse_next(input)
}
fn detailed_ref_type(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("ref")),
        opt(trivias_prefixed(keyword("null"))),
        must(trivias_prefixed(heap_type)),
        r_paren,
    )
        .parse_next(input)
        .map(|(l_paren, mut keyword, null_keyword, heap_type, r_paren)| {
            let mut children = Vec::with_capacity(5);
            children.push(l_paren);
            children.append(&mut keyword);
            if let Some(mut null_keyword) = null_keyword {
                children.append(&mut null_keyword);
            }
            if let Some(mut heap_type) = heap_type {
                children.append(&mut heap_type);
            }
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(REF_TYPE, children)
        })
}

pub(super) fn val_type(input: &mut Input) -> GreenResult {
    dispatch! {peek(any);
        '(' => ref_type,
        _ => word.verify_map(|word| match word {
            "i32" | "i64" | "f32" | "f64" => Some(node(NUM_TYPE, [tok(TYPE_KEYWORD, word)])),
            "v128" => Some(node(VEC_TYPE, [tok(TYPE_KEYWORD, word)])),
            word => try_into_abbr_ref_type(word).map(|ty| node(REF_TYPE, [ty])),
        }),
    }
    .context(Message::Name("value type"))
    .parse_next(input)
}

pub(super) fn heap_type(input: &mut Input) -> GreenResult {
    alt((abs_heap_type, index))
        .context(Message::Name("heap type"))
        .parse_next(input)
        .map(|ty| node(HEAP_TYPE, [ty]))
}

fn abs_heap_type(input: &mut Input) -> GreenResult {
    word.verify_map(|word| match word {
        "any" | "eq" | "i31" | "struct" | "array" | "none" | "func" | "nofunc" | "extern"
        | "noextern" => Some(tok(TYPE_KEYWORD, word)),
        _ => None,
    })
    .parse_next(input)
}

fn func_type(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("func")),
        repeat::<_, _, Vec<_>, _, _>(0.., retry_once(param, ["result", "ref"])),
        repeat::<_, _, Vec<_>, _, _>(0.., retry_once(result, [])),
        r_paren,
    )
        .parse_next(input)
        .map(|(l_paren, mut keyword, params, results, r_paren)| {
            let mut children = Vec::with_capacity(5);
            children.push(l_paren);
            children.append(&mut keyword);
            params
                .into_iter()
                .for_each(|mut param| children.append(&mut param));
            results
                .into_iter()
                .for_each(|mut result| children.append(&mut result));
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(FUNC_TYPE, children)
        })
}

pub(super) fn param(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("param")),
        alt((
            (trivias_prefixed(ident), must(retry_once(val_type, []))).map(|(mut children, ty)| {
                if let Some(mut ty) = ty {
                    children.append(&mut ty);
                }
                children
            }),
            repeat::<_, _, Vec<_>, _, _>(0.., retry_once(val_type, []))
                .map(|types| types.into_iter().flatten().collect()),
        )),
        r_paren,
    )
        .parse_next(input)
        .map(|(l_paren, mut keyword, mut types, r_paren)| {
            let mut children = Vec::with_capacity(5);
            children.push(l_paren);
            children.append(&mut keyword);
            children.append(&mut types);
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(PARAM, children)
        })
}

pub(super) fn result(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("result")),
        repeat::<_, _, Vec<_>, _, _>(0.., retry_once(val_type, [])),
        r_paren,
    )
        .parse_next(input)
        .map(|(l_paren, mut keyword, types, r_paren)| {
            let mut children = Vec::with_capacity(5);
            children.push(l_paren);
            children.append(&mut keyword);
            types
                .into_iter()
                .for_each(|mut ty| children.append(&mut ty));
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(RESULT, children)
        })
}

fn array_type(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("array")),
        must(retry(field_type, [])),
        r_paren,
    )
        .parse_next(input)
        .map(|(l_paren, mut keyword, field_type, r_paren)| {
            let mut children = Vec::with_capacity(4);
            children.push(l_paren);
            children.append(&mut keyword);
            if let Some(mut field_type) = field_type {
                children.append(&mut field_type);
            }
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(ARRAY_TYPE, children)
        })
}

fn struct_type(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("struct")),
        repeat::<_, _, Vec<_>, _, _>(0.., retry_once(field, [])),
        r_paren,
    )
        .parse_next(input)
        .map(|(l_paren, mut keyword, fields, r_paren)| {
            let mut children = Vec::with_capacity(4);
            children.push(l_paren);
            children.append(&mut keyword);
            fields
                .into_iter()
                .for_each(|mut field| children.append(&mut field));
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(STRUCT_TYPE, children)
        })
}

fn field(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("field")),
        alt((
            (trivias_prefixed(ident), must(retry(field_type, []))).map(|(mut children, ty)| {
                if let Some(mut ty) = ty {
                    children.append(&mut ty);
                }
                children
            }),
            repeat::<_, _, Vec<_>, _, _>(0.., retry_once(field_type, []))
                .map(|types| types.into_iter().flatten().collect()),
        )),
        r_paren,
    )
        .parse_next(input)
        .map(|(l_paren, mut keyword, mut types, r_paren)| {
            let mut children = Vec::with_capacity(5);
            children.push(l_paren);
            children.append(&mut keyword);
            children.append(&mut types);
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(FIELD, children)
        })
}

fn field_type(input: &mut Input) -> GreenResult {
    alt((
        (
            l_paren,
            trivias_prefixed(keyword("mut")),
            must(retry(storage_type, [])),
            r_paren,
        )
            .map(|(l_paren, mut keyword, storage_type, r_paren)| {
                let mut children = Vec::with_capacity(4);
                children.push(l_paren);
                children.append(&mut keyword);
                if let Some(mut storage_type) = storage_type {
                    children.append(&mut storage_type);
                }
                if let Some(mut r_paren) = r_paren {
                    children.append(&mut r_paren);
                }
                node(FIELD_TYPE, children)
            }),
        storage_type.map(|ty| node(FIELD_TYPE, [ty])),
    ))
    .context(Message::Name("field type"))
    .parse_next(input)
}

fn storage_type(input: &mut Input) -> GreenResult {
    alt((val_type, packed_type))
        .context(Message::Name("storage type"))
        .parse_next(input)
}

fn packed_type(input: &mut Input) -> GreenResult {
    word.verify_map(|word| match word {
        "i8" | "i16" => Some(node(PACKED_TYPE, [tok(TYPE_KEYWORD, word)])),
        _ => None,
    })
    .parse_next(input)
}

fn comp_type(input: &mut Input) -> GreenResult {
    alt((func_type, struct_type, array_type)).parse_next(input)
}

pub(super) fn sub_type(input: &mut Input) -> GreenResult {
    alt((
        comp_type.map(|ty| node(SUB_TYPE, [ty])),
        (
            l_paren,
            trivias_prefixed(keyword("sub")),
            opt(trivias_prefixed(keyword("final"))),
            repeat::<_, _, Vec<_>, _, _>(0.., trivias_prefixed(index)),
            must(retry(comp_type, [])),
            r_paren,
        )
            .map(
                |(l_paren, mut keyword, final_keyword, indexes, comp_type, r_paren)| {
                    let mut children = Vec::with_capacity(6);
                    children.push(l_paren);
                    children.append(&mut keyword);
                    if let Some(mut final_keyword) = final_keyword {
                        children.append(&mut final_keyword);
                    }
                    indexes
                        .into_iter()
                        .for_each(|mut index| children.append(&mut index));
                    if let Some(mut comp_type) = comp_type {
                        children.append(&mut comp_type);
                    }
                    if let Some(mut r_paren) = r_paren {
                        children.append(&mut r_paren);
                    }
                    node(SUB_TYPE, children)
                },
            ),
    ))
    .parse_next(input)
}

pub(super) fn table_type(input: &mut Input) -> GreenResult {
    (limits, must(retry_once(ref_type, [])))
        .context(Message::Name("table type"))
        .parse_next(input)
        .map(|(limits, ref_type)| {
            if let Some(mut ref_type) = ref_type {
                let mut children = vec![limits];
                children.append(&mut ref_type);
                node(TABLE_TYPE, children)
            } else {
                node(TABLE_TYPE, [limits])
            }
        })
}

pub(super) fn memory_type(input: &mut Input) -> GreenResult {
    limits
        .context(Message::Name("memory type"))
        .parse_next(input)
        .map(|limits| node(MEMORY_TYPE, [limits]))
}

pub(super) fn global_type(input: &mut Input) -> GreenResult {
    alt((
        val_type.map(|child| node(GLOBAL_TYPE, [child])),
        (
            l_paren,
            trivias_prefixed(keyword("mut")),
            must(retry(val_type, [])),
            r_paren,
        )
            .map(|(l_paren, mut keyword, val_type, r_paren)| {
                let mut children = Vec::with_capacity(4);
                children.push(l_paren);
                children.append(&mut keyword);
                if let Some(mut val_type) = val_type {
                    children.append(&mut val_type);
                }
                if let Some(mut r_paren) = r_paren {
                    children.append(&mut r_paren);
                }
                node(GLOBAL_TYPE, children)
            }),
    ))
    .parse_next(input)
}

fn limits(input: &mut Input) -> GreenResult {
    (unsigned_int, opt(trivias_prefixed(unsigned_int)))
        .parse_next(input)
        .map(|(min, max)| {
            let mut children = Vec::with_capacity(3);
            children.push(min);
            if let Some(mut max) = max {
                children.append(&mut max);
            }
            node(LIMITS, children)
        })
}
