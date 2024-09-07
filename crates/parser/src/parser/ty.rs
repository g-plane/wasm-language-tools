use super::{node, resume, retry, tok, token::*, GreenResult, Input};
use crate::SyntaxKind::*;
use winnow::{
    combinator::{empty, opt, repeat},
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
        opt(trivias_prefixed(ident)),
        trivias_prefixed(val_type).resume_after(empty),
        resume(r_paren),
    )
        .parse_next(input)
        .map(|(l_paren, mut keyword, id, ty, r_paren)| {
            let mut children = Vec::with_capacity(5);
            children.push(l_paren);
            children.append(&mut keyword);
            if let Some(mut id) = id {
                children.append(&mut id);
            }
            if let Some(mut ty) = ty {
                children.append(&mut ty);
            }
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
