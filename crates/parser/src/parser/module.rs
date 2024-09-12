use super::{
    instr::instr,
    node, resume, retry,
    token::{
        ident, keyword, l_paren, r_paren, string, trivias, trivias_prefixed, unsigned_int, word,
    },
    ty::{func_type, global_type, memory_type, param, result, table_type, val_type},
    GreenElement, GreenResult, Input,
};
use crate::error::SyntaxError;
use wat_syntax::SyntaxKind::{self, *};
use winnow::{
    combinator::{alt, dispatch, fail, opt, peek, preceded, repeat},
    error::{StrContext, StrContextValue},
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
        .parse_next(input)
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
}

fn module_field(input: &mut Input) -> GreenResult {
    dispatch! {peek(preceded(('(', trivias), word));
        "func" => module_field_func,
        "type" => module_field_type,
        "export" => module_field_export,
        "import" => module_field_import,
        "start" => module_field_start,
        "data" => module_field_data,
        "global" => module_field_global,
        _ => fail,
    }
    .parse_next(input)
    .map(|children| node(MODULE_FIELD, [children]))
}

fn module_field_data(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("data")),
        opt(trivias_prefixed(ident)),
        opt(trivias_prefixed(mem_use)),
        opt(trivias_prefixed(offset)),
        repeat::<_, _, Vec<_>, _, _>(0.., trivias_prefixed(string)),
        resume(r_paren),
    )
        .parse_next(input)
        .map(
            |(l_paren, mut keyword, id, mem_use, offset, strings, r_paren)| {
                let mut children = Vec::with_capacity(7);
                children.push(l_paren);
                children.append(&mut keyword);
                if let Some(mut id) = id {
                    children.append(&mut id);
                }
                if let Some(mut mem_use) = mem_use {
                    children.append(&mut mem_use);
                }
                if let Some(mut offset) = offset {
                    children.append(&mut offset);
                }
                strings
                    .into_iter()
                    .for_each(|mut string| children.append(&mut string));
                if let Some(mut r_paren) = r_paren {
                    children.append(&mut r_paren);
                }
                node(MODULE_FIELD_DATA, children)
            },
        )
}

fn module_field_export(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("export")),
        resume(name),
        resume(
            export_desc.context(StrContext::Expected(StrContextValue::Description(
                "export desc",
            ))),
        ),
        resume(r_paren),
    )
        .parse_next(input)
        .map(|(l_paren, mut keyword, name, export_desc, r_paren)| {
            let mut children = Vec::with_capacity(5);
            children.push(l_paren);
            children.append(&mut keyword);
            if let Some(mut name) = name {
                children.append(&mut name);
            }
            if let Some(mut export_desc) = export_desc {
                children.append(&mut export_desc);
            }
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(MODULE_FIELD_EXPORT, children)
        })
}

fn module_field_func(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("func")),
        opt(trivias_prefixed(ident)),
        opt(trivias_prefixed(import)), // postpone syntax error for using import with export or instr
        opt(trivias_prefixed(export)),
        resume(type_use),
        repeat::<_, _, Vec<_>, _, _>(0.., retry(local)),
        repeat::<_, _, Vec<_>, _, _>(0.., trivias_prefixed(instr)),
        resume(r_paren),
    )
        .parse_next(input)
        .map(
            |(l_paren, mut keyword, id, import, export, type_use, locals, instrs, r_paren)| {
                let mut children = Vec::with_capacity(7);
                children.push(l_paren);
                children.append(&mut keyword);
                if let Some(mut id) = id {
                    children.append(&mut id);
                }
                if let Some(mut import) = import {
                    children.append(&mut import);
                }
                if let Some(mut export) = export {
                    children.append(&mut export);
                }
                if let Some(mut type_use) = type_use {
                    children.append(&mut type_use);
                }
                locals
                    .into_iter()
                    .for_each(|mut local| children.append(&mut local));
                instrs
                    .into_iter()
                    .for_each(|mut instr| children.append(&mut instr));
                if let Some(mut r_paren) = r_paren {
                    children.append(&mut r_paren);
                }
                node(MODULE_FIELD_FUNC, children)
            },
        )
}

fn module_field_global(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("global")),
        opt(trivias_prefixed(ident)),
        opt(trivias_prefixed(import)), // postpone syntax error for using import with export or instr
        opt(trivias_prefixed(export)),
        resume(global_type),
        repeat::<_, _, Vec<_>, _, _>(0.., trivias_prefixed(instr)),
        resume(r_paren),
    )
        .parse_next(input)
        .map(
            |(l_paren, mut keyword, id, import, export, ty, instrs, r_paren)| {
                let mut children = Vec::with_capacity(7);
                children.push(l_paren);
                children.append(&mut keyword);
                if let Some(mut id) = id {
                    children.append(&mut id);
                }
                if let Some(mut import) = import {
                    children.append(&mut import);
                }
                if let Some(mut export) = export {
                    children.append(&mut export);
                }
                if let Some(mut ty) = ty {
                    children.append(&mut ty);
                }
                instrs
                    .into_iter()
                    .for_each(|mut instr| children.append(&mut instr));
                if let Some(mut r_paren) = r_paren {
                    children.append(&mut r_paren);
                }
                node(MODULE_FIELD_GLOBAL, children)
            },
        )
}

fn module_field_import(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("import")),
        resume(module_name),
        resume(name),
        resume(import_desc),
        resume(r_paren),
    )
        .parse_next(input)
        .map(|(l_paren, mut keyword, module_name, name, desc, r_paren)| {
            let mut children = Vec::with_capacity(6);
            children.push(l_paren);
            children.append(&mut keyword);
            if let Some(mut module_name) = module_name {
                children.append(&mut module_name);
            }
            if let Some(mut name) = name {
                children.append(&mut name);
            }
            if let Some(mut desc) = desc {
                children.append(&mut desc);
            }
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(MODULE_FIELD_IMPORT, children)
        })
}

fn module_field_start(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("start")),
        resume(index),
        resume(r_paren),
    )
        .parse_next(input)
        .map(|(l_paren, mut keyword, index, r_paren)| {
            let mut children = Vec::with_capacity(4);
            children.push(l_paren);
            children.append(&mut keyword);
            if let Some(mut index) = index {
                children.append(&mut index);
            }
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(MODULE_FIELD_START, children)
        })
}

fn module_field_type(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("type")),
        opt(trivias_prefixed(ident)),
        retry(func_type),
        resume(r_paren),
    )
        .parse_next(input)
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
}

pub(super) fn local(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("local")),
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
            node(LOCAL, children)
        })
}

fn import(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("import")),
        resume(module_name),
        resume(name),
        resume(r_paren),
    )
        .parse_next(input)
        .map(|(l_paren, mut keyword, module_name, name, r_paren)| {
            let mut children = Vec::with_capacity(5);
            children.push(l_paren);
            children.append(&mut keyword);
            if let Some(mut module_name) = module_name {
                children.append(&mut module_name);
            }
            if let Some(mut name) = name {
                children.append(&mut name);
            }
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(IMPORT, children)
        })
}

fn export(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("export")),
        resume(name),
        resume(r_paren),
    )
        .parse_next(input)
        .map(|(l_paren, mut keyword, name, r_paren)| {
            let mut children = Vec::with_capacity(4);
            children.push(l_paren);
            children.append(&mut keyword);
            if let Some(mut name) = name {
                children.append(&mut name);
            }
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(EXPORT, children)
        })
}

fn import_desc(input: &mut Input) -> GreenResult {
    dispatch! {peek(preceded(('(', trivias), word));
        "func" => import_desc_type_use,
        "global" => import_desc_global_type,
        "memory" => import_desc_memory_type,
        "table" => import_desc_table_type,
        _ => fail,
    }
    .parse_next(input)
    .map(|children| node(IMPORT_DESC, [children]))
}

fn import_desc_global_type(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("global")),
        opt(trivias_prefixed(ident)),
        resume(global_type),
        resume(r_paren),
    )
        .parse_next(input)
        .map(|(l_paren, mut keyword, id, global_type, r_paren)| {
            let mut children = Vec::with_capacity(5);
            children.push(l_paren);
            children.append(&mut keyword);
            if let Some(mut id) = id {
                children.append(&mut id);
            }
            if let Some(mut global_type) = global_type {
                children.append(&mut global_type);
            }
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(IMPORT_DESC_GLOBAL_TYPE, children)
        })
}

fn import_desc_memory_type(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("memory")),
        opt(trivias_prefixed(ident)),
        resume(memory_type),
        resume(r_paren),
    )
        .parse_next(input)
        .map(|(l_paren, mut keyword, id, memory_type, r_paren)| {
            let mut children = Vec::with_capacity(5);
            children.push(l_paren);
            children.append(&mut keyword);
            if let Some(mut id) = id {
                children.append(&mut id);
            }
            if let Some(mut memory_type) = memory_type {
                children.append(&mut memory_type);
            }
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(IMPORT_DESC_MEMORY_TYPE, children)
        })
}

fn import_desc_table_type(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("table")),
        opt(trivias_prefixed(ident)),
        resume(table_type),
        resume(r_paren),
    )
        .parse_next(input)
        .map(|(l_paren, mut keyword, id, table_type, r_paren)| {
            let mut children = Vec::with_capacity(5);
            children.push(l_paren);
            children.append(&mut keyword);
            if let Some(mut id) = id {
                children.append(&mut id);
            }
            if let Some(mut table_type) = table_type {
                children.append(&mut table_type);
            }
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(IMPORT_DESC_TABLE_TYPE, children)
        })
}

fn import_desc_type_use(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("func")),
        opt(trivias_prefixed(ident)),
        resume(type_use),
        resume(r_paren),
    )
        .parse_next(input)
        .map(|(l_paren, mut keyword, id, type_use, r_paren)| {
            let mut children = Vec::with_capacity(5);
            children.push(l_paren);
            children.append(&mut keyword);
            if let Some(mut id) = id {
                children.append(&mut id);
            }
            if let Some(mut type_use) = type_use {
                children.append(&mut type_use);
            }
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(IMPORT_DESC_TYPE_USE, children)
        })
}

fn export_desc(input: &mut Input) -> GreenResult {
    dispatch! {peek(preceded(('(', trivias), word));
        "func" => export_desc_variant("func", EXPORT_DESC_FUNC),
        "table" => export_desc_variant("table", EXPORT_DESC_TABLE),
        "memory" => export_desc_variant("memory", EXPORT_DESC_MEMORY),
        "global" => export_desc_variant("global", EXPORT_DESC_GLOBAL),
        _ => fail,
    }
    .parse_next(input)
    .map(|child| node(EXPORT_DESC, [child]))
}

fn export_desc_variant<'s>(
    keyword_literal: &'static str,
    kind: SyntaxKind,
) -> impl Parser<Input<'s>, GreenElement, SyntaxError> {
    (
        l_paren,
        trivias_prefixed(keyword(keyword_literal)),
        resume(index),
        resume(r_paren),
    )
        .map(move |(l_paren, mut keyword, index, r_paren)| {
            let mut children = Vec::with_capacity(4);
            children.push(l_paren);
            children.append(&mut keyword);
            if let Some(mut index) = index {
                children.append(&mut index);
            }
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(kind, children)
        })
}

fn module_name(input: &mut Input) -> GreenResult {
    string
        .parse_next(input)
        .map(|child| node(MODULE_NAME, [child]))
}

fn name(input: &mut Input) -> GreenResult {
    string.parse_next(input).map(|child| node(NAME, [child]))
}

pub(super) fn type_use(input: &mut Input) -> GreenResult {
    (
        opt((
            l_paren,
            trivias_prefixed(keyword("type")),
            resume(
                index.context(StrContext::Expected(StrContextValue::Description(
                    "type index",
                ))),
            ),
            resume(r_paren),
        )),
        repeat::<_, _, Vec<_>, _, _>(0.., retry(param)),
        repeat::<_, _, Vec<_>, _, _>(0.., retry(result)),
    )
        .parse_next(input)
        .map(|(type_index, params, results)| {
            let mut children = Vec::with_capacity(4);
            if let Some((l_paren, mut keyword, index, r_paren)) = type_index {
                children.push(l_paren);
                children.append(&mut keyword);
                if let Some(mut index) = index {
                    children.append(&mut index);
                }
                if let Some(mut r_paren) = r_paren {
                    children.append(&mut r_paren);
                }
            }
            params
                .into_iter()
                .for_each(|mut param| children.append(&mut param));
            results
                .into_iter()
                .for_each(|mut result| children.append(&mut result));
            node(TYPE_USE, children)
        })
}

fn index(input: &mut Input) -> GreenResult {
    alt((ident, unsigned_int))
        .context(StrContext::Expected(StrContextValue::Description(
            "index (identifier or unsigned integer)",
        )))
        .parse_next(input)
        .map(|child| node(INDEX, [child]))
}

fn mem_use(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("memory")),
        resume(unsigned_int),
        resume(r_paren),
    )
        .parse_next(input)
        .map(|(l_paren, mut keyword, idx, r_paren)| {
            let mut children = Vec::with_capacity(4);
            children.push(l_paren);
            children.append(&mut keyword);
            if let Some(mut idx) = idx {
                children.append(&mut idx);
            }
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(MEM_USE, children)
        })
}

fn offset(input: &mut Input) -> GreenResult {
    alt((
        (
            l_paren,
            trivias_prefixed(keyword("offset")),
            repeat::<_, _, Vec<_>, _, _>(0.., trivias_prefixed(instr)),
            resume(r_paren),
        )
            .map(|(l_paren, mut keyword, instrs, r_paren)| {
                let mut children = Vec::with_capacity(4);
                children.push(l_paren);
                children.append(&mut keyword);
                instrs
                    .into_iter()
                    .for_each(|mut instr| children.append(&mut instr));
                if let Some(mut r_paren) = r_paren {
                    children.append(&mut r_paren);
                }
                node(OFFSET, children)
            }),
        preceded(peek('('), instr).map(|child| node(OFFSET, [child])),
    ))
    .parse_next(input)
}
