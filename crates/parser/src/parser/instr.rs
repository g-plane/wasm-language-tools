use super::{
    module::type_use,
    must, node, retry_once, tok,
    token::{
        float, ident, int, keyword, l_paren, r_paren, string, trivias, trivias_prefixed,
        unsigned_int_impl, word,
    },
    ty::{heap_type, ref_type},
    GreenResult, Input,
};
use crate::error::Message;
use wat_syntax::SyntaxKind::*;
use winnow::{
    combinator::{alt, dispatch, fail, opt, peek, preceded, repeat, repeat_till},
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
    type_use
        .parse_next(input)
        .map(|child| node(BLOCK_TYPE, [child]))
}

fn block_block(input: &mut Input) -> GreenResult {
    alt((
        (
            keyword("block"),
            opt(trivias_prefixed(ident)),
            opt(trivias_prefixed(block_type)),
            repeat_till::<_, _, Vec<_>, _, _, _, _>(
                0..,
                retry_once(instr, []),
                peek((trivias, alt((word.verify(|word: &str| word == "end"), ")")))),
            ),
            must(trivias_prefixed(keyword("end"))),
            opt(trivias_prefixed(ident)),
        )
            .map(|(keyword, label, block_type, (instrs, _), end, id)| {
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
            opt(trivias_prefixed(ident)),
            opt(trivias_prefixed(block_type)),
            repeat::<_, _, Vec<_>, _, _>(0.., retry_once(instr, [])),
            r_paren,
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
            opt(trivias_prefixed(ident)),
            opt(trivias_prefixed(block_type)),
            repeat_till::<_, _, Vec<_>, _, _, _, _>(
                0..,
                retry_once(instr, []),
                peek((trivias, alt((word.verify(|word: &str| word == "end"), ")")))),
            ),
            must(trivias_prefixed(keyword("end"))),
            opt(trivias_prefixed(ident)),
        )
            .map(|(keyword, label, block_type, (instrs, _), end, id)| {
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
            opt(trivias_prefixed(ident)),
            opt(trivias_prefixed(block_type)),
            repeat::<_, _, Vec<_>, _, _>(0.., retry_once(instr, [])),
            r_paren,
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
    alt((
        (
            keyword("if"),
            opt(trivias_prefixed(ident)),
            opt(trivias_prefixed(block_type)),
            repeat_till::<_, _, Vec<_>, _, _, _, _>(
                0..,
                retry_once(instr, []),
                peek((
                    trivias,
                    alt((word.verify(|word| matches!(word, "end" | "else")), ")")),
                )),
            ),
            opt((
                trivias,
                keyword("else"),
                opt(trivias_prefixed(ident)),
                repeat_till::<_, _, Vec<_>, _, _, _, _>(
                    0..,
                    retry_once(instr, []),
                    peek((trivias, alt((word.verify(|word: &str| word == "end"), ")")))),
                ),
            )),
            must(trivias_prefixed(keyword("end"))),
            opt(trivias_prefixed(ident)),
        )
            .map(
                |(keyword, label, block_type, (instrs, _), else_branch, end, id)| {
                    let mut children = Vec::with_capacity(6);
                    children.push(keyword);
                    if let Some(mut label) = label {
                        children.append(&mut label);
                    }
                    if let Some(mut block_type) = block_type {
                        children.append(&mut block_type);
                    }
                    children.push(node(
                        BLOCK_IF_THEN,
                        instrs
                            .into_iter()
                            .flat_map(|instr| instr.into_iter())
                            .collect::<Vec<_>>(),
                    ));
                    if let Some((mut trivias, keyword, id, (instrs, _))) = else_branch {
                        children.append(&mut trivias);
                        let mut else_children = Vec::with_capacity(3);
                        else_children.push(keyword);
                        if let Some(mut id) = id {
                            else_children.append(&mut id);
                        }
                        instrs
                            .into_iter()
                            .for_each(|mut instr| else_children.append(&mut instr));
                        children.push(node(BLOCK_IF_ELSE, else_children));
                    }
                    if let Some(mut end) = end {
                        children.append(&mut end);
                    }
                    if let Some(mut id) = id {
                        children.append(&mut id);
                    }
                    node(BLOCK_IF, children)
                },
            ),
        (
            l_paren,
            trivias_prefixed(keyword("if")),
            opt(trivias_prefixed(ident)),
            opt(trivias_prefixed(block_type)),
            repeat_till::<_, _, Vec<_>, _, _, _, _>(
                0..,
                retry_once(instr, []),
                peek(alt((
                    (trivias, '(', trivias_prefixed(keyword("then"))).void(),
                    (trivias, ')').void(),
                ))),
            ),
            (
                trivias,
                must((
                    l_paren,
                    trivias_prefixed(keyword("then")),
                    repeat::<_, _, Vec<_>, _, _>(0.., retry_once(instr, [])),
                    r_paren,
                )),
            ),
            opt((
                trivias,
                l_paren,
                trivias_prefixed(keyword("else")),
                repeat::<_, _, Vec<_>, _, _>(0.., retry_once(instr, [])),
                r_paren,
            )),
            r_paren,
        )
            .map(
                |(
                    l_paren,
                    mut if_keyword,
                    id,
                    block_type,
                    (cond_instrs, ..),
                    (mut trivias_before_then, then),
                    else_branch,
                    r_paren,
                )| {
                    let mut children = Vec::with_capacity(11);
                    children.push(l_paren);
                    children.append(&mut if_keyword);
                    if let Some(mut id) = id {
                        children.append(&mut id);
                    }
                    if let Some(mut block_type) = block_type {
                        children.append(&mut block_type);
                    }
                    cond_instrs
                        .into_iter()
                        .for_each(|mut instr| children.append(&mut instr));
                    children.append(&mut trivias_before_then);
                    if let Some((l_paren, mut keyword, instrs, r_paren)) = then {
                        let mut then_children = Vec::with_capacity(4);
                        then_children.push(l_paren);
                        then_children.append(&mut keyword);
                        instrs
                            .into_iter()
                            .for_each(|mut instr| then_children.append(&mut instr));
                        if let Some(mut r_paren) = r_paren {
                            then_children.append(&mut r_paren);
                        }
                        children.push(node(BLOCK_IF_THEN, then_children));
                    }

                    if let Some((mut trivias, l_paren, mut keyword, instrs, r_paren)) = else_branch
                    {
                        children.append(&mut trivias);
                        let mut else_children = Vec::with_capacity(4);
                        else_children.push(l_paren);
                        else_children.append(&mut keyword);
                        instrs
                            .into_iter()
                            .for_each(|mut instr| else_children.append(&mut instr));
                        if let Some(mut r_paren) = r_paren {
                            else_children.append(&mut r_paren);
                        }
                        children.push(node(BLOCK_IF_ELSE, else_children));
                    }

                    if let Some(mut r_paren) = r_paren {
                        children.append(&mut r_paren);
                    }

                    node(BLOCK_IF, children)
                },
            ),
    ))
    .parse_next(input)
}

fn plain_instr(input: &mut Input) -> GreenResult {
    alt((
        (
            l_paren,
            trivias_prefixed(instr_name),
            repeat::<_, _, Vec<_>, _, _>(0.., trivias_prefixed(immediate)),
            repeat::<_, _, Vec<_>, _, _>(
                0..,
                retry_once(
                    preceded(peek('('), instr).context(Message::Name("instruction")),
                    [],
                ),
            ),
            r_paren,
        )
            .map(|(l_paren, mut instr_name, immediates, operands, r_paren)| {
                let mut children = Vec::with_capacity(4);
                children.push(l_paren);
                children.append(&mut instr_name);
                immediates
                    .into_iter()
                    .for_each(|mut immediate| children.append(&mut immediate));
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
            repeat::<_, _, Vec<_>, _, _>(0.., trivias_prefixed(immediate)),
        )
            .map(|(instr_name, immediates)| {
                let mut children = Vec::with_capacity(2);
                children.push(instr_name);
                immediates
                    .into_iter()
                    .for_each(|mut immediate| children.append(&mut immediate));
                node(PLAIN_INSTR, children)
            }),
    ))
    .parse_next(input)
}

fn instr_name(input: &mut Input) -> GreenResult {
    word.context(Message::Description("invalid instruction name"))
        .parse_next(input)
        .map(|text| tok(INSTR_NAME, text))
}

fn immediate(input: &mut Input) -> GreenResult {
    dispatch! {peek(any);
        '0'..='9' | '+' | '-' => alt((int, float)),
        '.' => float,
        '"' => string,
        '$' => ident,
        '(' => alt((type_use, ref_type)),
        'a'..='z' => alt((float, mem_arg, heap_type, ref_type)),
        _ => fail,
    }
    .context(Message::Name("immediate"))
    .parse_next(input)
    .map(|child| node(IMMEDIATE, [child]))
}

fn mem_arg(input: &mut Input) -> GreenResult {
    (alt(("offset", "align")), '=', unsigned_int_impl)
        .take()
        .parse_next(input)
        .map(|text| tok(MEM_ARG, text))
}
