use super::{
    node, resume, retry, tok,
    token::{ident, keyword, l_paren, r_paren, trivias_prefixed, word},
    GreenResult, Input,
};
use crate::SyntaxKind::*;
use winnow::{
    combinator::{alt, opt, repeat},
    error::{StrContext, StrContextValue},
    Parser,
};

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

fn param(input: &mut Input) -> GreenResult {
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

fn result(input: &mut Input) -> GreenResult {
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
