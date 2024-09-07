use super::{node, resume, retry, token::*, ty::func_type, GreenResult, Input};
use crate::SyntaxKind::*;
use winnow::{
    combinator::{opt, repeat},
    Parser,
};

pub(super) fn module(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("module")),
        opt(trivias_prefixed(ident)),
        repeat::<_, _, Vec<_>, _, _>(0.., retry(module_field)),
        resume(r_paren),
    )
        .map(|(l_paren, mut keyword, id, fields, r_paren)| {
            let mut children = Vec::with_capacity(4);
            children.push(l_paren);
            children.append(&mut keyword);
            if let Some(mut id) = id {
                children.append(&mut id);
            }
            fields
                .into_iter()
                .for_each(|mut field| children.append(&mut field));
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(MODULE, children)
        })
        .parse_next(input)
}

fn module_field(input: &mut Input) -> GreenResult {
    module_field_type
        .parse_next(input)
        .map(|children| node(MODULE_FIELD, [children]))
}

fn module_field_type(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("type")),
        opt(trivias_prefixed(ident)),
        retry(func_type),
        resume(r_paren),
    )
        .map(|(l_paren, mut keyword, id, mut ty, r_paren)| {
            let mut children = Vec::with_capacity(5);
            children.push(l_paren);
            children.append(&mut keyword);
            if let Some(mut id) = id {
                children.append(&mut id);
            }
            children.append(&mut ty);
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(MODULE_FIELD_TYPE, children)
        })
        .parse_next(input)
}
