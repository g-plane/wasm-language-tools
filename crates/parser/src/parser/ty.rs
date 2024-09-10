use super::{
    node, resume, retry, tok,
    token::{ident, keyword, l_paren, r_paren, trivias_prefixed, unsigned_int, word},
    GreenResult, Input,
};
use wat_syntax::SyntaxKind::*;
use winnow::{
    combinator::{alt, opt, repeat},
    error::{StrContext, StrContextValue},
    stream::AsChar,
    token::{one_of, take_while},
    Parser,
};

fn ref_type(input: &mut Input) -> GreenResult {
    word.verify_map(|word| match word {
        "funcref" | "externref" => Some(tok(REF_TYPE, word)),
        _ => None,
    })
    .context(StrContext::Expected(StrContextValue::Description(
        "ref type",
    )))
    .parse_next(input)
}

fn val_type(input: &mut Input) -> GreenResult {
    word.verify_map(|word| match word {
        "i32" | "i64" | "f32" | "f64" => Some(tok(NUM_TYPE, word)),
        "v128" => Some(tok(VEC_TYPE, word)),
        "funcref" | "externref" => Some(tok(REF_TYPE, word)),
        _ => None,
    })
    .context(StrContext::Expected(StrContextValue::Description(
        "value type",
    )))
    .parse_next(input)
    .map(|ty| node(VAL_TYPE, [ty]))
}

pub(super) fn func_type(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("func")),
        repeat::<_, _, Vec<_>, _, _>(0.., retry(param)),
        repeat::<_, _, Vec<_>, _, _>(0.., retry(result)),
        resume(r_paren),
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
            repeat::<_, _, Vec<_>, _, _>(1.., trivias_prefixed(val_type))
                .map(|types| types.into_iter().flatten().collect()),
            (opt(trivias_prefixed(ident)), retry(val_type)).map(|(id, mut ty)| {
                if let Some(mut id) = id {
                    id.append(&mut ty);
                    id
                } else {
                    ty
                }
            }),
        )),
        resume(r_paren),
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
        repeat::<_, _, Vec<_>, _, _>(0.., trivias_prefixed(val_type)),
        resume(r_paren),
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

pub(super) fn table_type(input: &mut Input) -> GreenResult {
    (limits, resume(ref_type))
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
        .parse_next(input)
        .map(|limits| node(MEMORY_TYPE, [limits]))
}

pub(super) fn global_type(input: &mut Input) -> GreenResult {
    alt((
        val_type.map(|child| node(GLOBAL_TYPE, [child])),
        (
            l_paren,
            trivias_prefixed(keyword("mut")),
            resume(val_type),
            resume(r_paren),
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
    (
        nat,
        opt(trivias_prefixed(nat)),
        opt(trivias_prefixed(share)),
    )
        .parse_next(input)
        .map(|(min, max, share)| {
            let mut children = Vec::with_capacity(3);
            children.push(min);
            if let Some(mut max) = max {
                children.append(&mut max);
            }
            if let Some(mut share) = share {
                children.append(&mut share);
            }
            node(LIMITS, children)
        })
}

pub(super) fn nat(input: &mut Input) -> GreenResult {
    unsigned_int
        .parse_next(input)
        .map(|child| node(NAT, [child]))
}

fn share(input: &mut Input) -> GreenResult {
    word.verify(|word: &str| word == "shared" || word == "unshared")
        .parse_next(input)
        .map(|text| tok(SHARE, text))
}
