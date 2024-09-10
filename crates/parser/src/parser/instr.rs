use super::{
    module::type_use,
    node, resume, retry,
    token::{ident, keyword, trivias_prefixed, word},
    ty::result,
    GreenResult, Input,
};
use wat_syntax::SyntaxKind::*;
use winnow::{
    combinator::{alt, dispatch, fail, opt, peek, repeat},
    Parser,
};

pub(super) fn instr(input: &mut Input) -> GreenResult {
    block_instr.parse_next(input)
}

fn block_instr(input: &mut Input) -> GreenResult {
    dispatch! {peek(word);
        "if" => block_if,
        "loop" => block_loop,
        "block" => block_block,
        _ => fail,
    }
    .parse_next(input)
}

fn block_type(input: &mut Input) -> GreenResult {
    alt((
        opt(result).map(|child| {
            if let Some(child) = child {
                node(BLOCK_TYPE, [child])
            } else {
                node(BLOCK_TYPE, [])
            }
        }),
        type_use.map(|child| node(BLOCK_TYPE, [child])),
    ))
    .parse_next(input)
}

fn block_block(input: &mut Input) -> GreenResult {
    (
        keyword("block"),
        resume(ident),
        resume(block_type),
        repeat::<_, _, Vec<_>, _, _>(0.., retry(instr)),
        resume(keyword("end")),
        opt(trivias_prefixed(ident)),
    )
        .parse_next(input)
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
        })
}

fn block_loop(input: &mut Input) -> GreenResult {
    (
        keyword("loop"),
        resume(ident),
        resume(block_type),
        repeat::<_, _, Vec<_>, _, _>(0.., retry(instr)),
        resume(keyword("end")),
        opt(trivias_prefixed(ident)),
    )
        .parse_next(input)
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
        })
}

fn block_if(input: &mut Input) -> GreenResult {
    (
        keyword("if"),
        resume(ident),
        resume(block_type),
        repeat::<_, _, Vec<_>, _, _>(0.., retry(instr)),
        opt((
            trivias_prefixed(keyword("else")),
            opt(trivias_prefixed(ident)),
            repeat::<_, _, Vec<_>, _, _>(0.., retry(instr)),
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
