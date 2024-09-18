use super::{
    module::type_use,
    node, resume, retry, tok,
    token::{
        float, ident, int, keyword, l_paren, r_paren, string, trivias, trivias_prefixed,
        unsigned_int_impl, word,
    },
    ty::{heap_type, result},
    GreenElement, GreenResult, Input,
};
use crate::error::SyntaxError;
use wat_syntax::SyntaxKind::*;
use winnow::{
    combinator::{alt, dispatch, fail, opt, peek, preceded, repeat},
    token::any,
    Parser,
};

pub(super) fn instr(input: &mut Input) -> GreenResult {
    alt((block_instr, plain_instr)).parse_next(input)
}

fn block_instr(input: &mut Input) -> GreenResult {
    dispatch! {peek(preceded(opt(('(', trivias)), word));
        "if" => block_if,
        "loop" => block_loop,
        "block" => block_block,
        _ => fail,
    }
    .parse_next(input)
}

fn block_type(input: &mut Input) -> GreenResult {
    alt((
        result.map(|child| node(BLOCK_TYPE, [child])),
        type_use.map(|child| node(BLOCK_TYPE, [child])),
    ))
    .parse_next(input)
}

fn block_block(input: &mut Input) -> GreenResult {
    alt((
        (
            keyword("block"),
            resume(ident),
            resume(block_type),
            repeat::<_, _, Vec<_>, _, _>(0.., retry(instr, [])),
            resume(keyword("end")),
            opt(trivias_prefixed(ident)),
        )
            .map(|(keyword, label, block_type, instrs, end, id)| {
                let mut children = Vec::with_capacity(6);
                children.push(keyword);
                if let Some(mut label) = label {
                    children.append(&mut label);
                }
                if let Some(mut block_type) = block_type {
                    children.append(&mut block_type);
                }
                instrs
                    .into_iter()
                    .for_each(|mut instr| children.append(&mut instr));
                if let Some(mut end) = end {
                    children.append(&mut end);
                }
                if let Some(mut id) = id {
                    children.append(&mut id);
                }
                node(BLOCK_BLOCK, children)
            }),
        (
            l_paren,
            trivias_prefixed(keyword("block")),
            resume(ident),
            resume(block_type),
            repeat::<_, _, Vec<_>, _, _>(0.., retry(instr, [])),
            resume(r_paren),
            opt(trivias_prefixed(ident)),
        )
            .map(
                |(l_paren, mut keyword, label, block_type, instrs, r_paren, id)| {
                    let mut children = Vec::with_capacity(6);
                    children.push(l_paren);
                    children.append(&mut keyword);
                    if let Some(mut label) = label {
                        children.append(&mut label);
                    }
                    if let Some(mut block_type) = block_type {
                        children.append(&mut block_type);
                    }
                    instrs
                        .into_iter()
                        .for_each(|mut instr| children.append(&mut instr));
                    if let Some(mut r_paren) = r_paren {
                        children.append(&mut r_paren);
                    }
                    if let Some(mut id) = id {
                        children.append(&mut id);
                    }
                    node(BLOCK_BLOCK, children)
                },
            ),
    ))
    .parse_next(input)
}

fn block_loop(input: &mut Input) -> GreenResult {
    alt((
        (
            keyword("loop"),
            resume(ident),
            resume(block_type),
            repeat::<_, _, Vec<_>, _, _>(0.., retry(instr, [])),
            resume(keyword("end")),
            opt(trivias_prefixed(ident)),
        )
            .map(|(keyword, label, block_type, instrs, end, id)| {
                let mut children = Vec::with_capacity(6);
                children.push(keyword);
                if let Some(mut label) = label {
                    children.append(&mut label);
                }
                if let Some(mut block_type) = block_type {
                    children.append(&mut block_type);
                }
                instrs
                    .into_iter()
                    .for_each(|mut instr| children.append(&mut instr));
                if let Some(mut end) = end {
                    children.append(&mut end);
                }
                if let Some(mut id) = id {
                    children.append(&mut id);
                }
                node(BLOCK_LOOP, children)
            }),
        (
            l_paren,
            trivias_prefixed(keyword("loop")),
            resume(ident),
            resume(block_type),
            repeat::<_, _, Vec<_>, _, _>(0.., retry(instr, [])),
            resume(r_paren),
            opt(trivias_prefixed(ident)),
        )
            .map(
                |(l_paren, mut keyword, label, block_type, instrs, r_paren, id)| {
                    let mut children = Vec::with_capacity(6);
                    children.push(l_paren);
                    children.append(&mut keyword);
                    if let Some(mut label) = label {
                        children.append(&mut label);
                    }
                    if let Some(mut block_type) = block_type {
                        children.append(&mut block_type);
                    }
                    instrs
                        .into_iter()
                        .for_each(|mut instr| children.append(&mut instr));
                    if let Some(mut r_paren) = r_paren {
                        children.append(&mut r_paren);
                    }
                    if let Some(mut id) = id {
                        children.append(&mut id);
                    }
                    node(BLOCK_LOOP, children)
                },
            ),
    ))
    .parse_next(input)
}

fn block_if(input: &mut Input) -> GreenResult {
    (
        keyword("if"),
        resume(ident),
        opt(trivias_prefixed(block_type)),
        repeat::<_, _, Vec<_>, _, _>(0.., retry(instr, [])),
        opt((
            trivias_prefixed(keyword("else")),
            opt(trivias_prefixed(ident)),
            repeat::<_, _, Vec<_>, _, _>(0.., retry(instr, [])),
        )),
        resume(keyword("end")),
        opt(trivias_prefixed(ident)),
    )
        .parse_next(input)
        .map(
            |(keyword, label, block_type, instrs, else_branch, end, id)| {
                let mut children = Vec::with_capacity(6);
                children.push(keyword);
                if let Some(mut label) = label {
                    children.append(&mut label);
                }
                if let Some(mut block_type) = block_type {
                    children.append(&mut block_type);
                }
                instrs
                    .into_iter()
                    .for_each(|mut instr| children.append(&mut instr));
                if let Some((mut keyword, id, instrs)) = else_branch {
                    children.append(&mut keyword);
                    if let Some(mut id) = id {
                        children.append(&mut id);
                    }
                    instrs
                        .into_iter()
                        .for_each(|mut instr| children.append(&mut instr));
                }
                if let Some(mut end) = end {
                    children.append(&mut end);
                }
                if let Some(mut id) = id {
                    children.append(&mut id);
                }
                node(BLOCK_IF, children)
            },
        )
}

fn plain_instr(input: &mut Input) -> GreenResult {
    alt((
        (
            l_paren,
            trivias_prefixed(instr_name),
            repeat::<_, _, Vec<_>, _, _>(0.., trivias_prefixed(operand(true))),
            resume(r_paren),
        )
            .map(|(l_paren, mut instr_name, operands, r_paren)| {
                let mut children = Vec::with_capacity(4);
                children.push(l_paren);
                children.append(&mut instr_name);
                operands
                    .into_iter()
                    .for_each(|mut operand| children.append(&mut operand));
                if let Some(mut r_paren) = r_paren {
                    children.append(&mut r_paren);
                }
                node(PLAIN_INSTR, children)
            }),
        (
            instr_name,
            repeat::<_, _, Vec<_>, _, _>(0.., trivias_prefixed(operand(false))),
        )
            .map(|(instr_name, operands)| {
                let mut children = Vec::with_capacity(2);
                children.push(instr_name);
                operands
                    .into_iter()
                    .for_each(|mut operand| children.append(&mut operand));
                node(PLAIN_INSTR, children)
            }),
    ))
    .parse_next(input)
}

fn instr_name(input: &mut Input) -> GreenResult {
    word.parse_next(input).map(|text| tok(INSTR_NAME, text))
}

fn operand<'s>(allow_instr: bool) -> impl Parser<Input<'s>, GreenElement, SyntaxError> {
    dispatch! {peek(any);
        '0'..='9' | '+' | '-' => alt((float, int)),
        '.' | 'i' | 'n' => float,
        '"' => string,
        '$' => ident,
        '(' => dispatch! {peek(preceded(('(', trivias), word));
            "type" => type_use,
            _ if allow_instr => instr,
            _ => fail,
        },
        'o' | 'a' => mem_arg,
        'f' | 'e' => heap_type,
        _ => fail,
    }
    .map(|child| node(OPERAND, [child]))
}

fn mem_arg(input: &mut Input) -> GreenResult {
    (alt(("offset", "align")), '=', unsigned_int_impl)
        .take()
        .parse_next(input)
        .map(|text| tok(MEM_ARG, text))
}
