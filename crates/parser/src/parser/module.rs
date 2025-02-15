use super::{
    instr::instr,
    must, node, retry, retry_once,
    token::{
        ident, keyword, l_paren, r_paren, string, trivias, trivias_prefixed, unsigned_int, word,
    },
    ty::{global_type, memory_type, param, ref_type, result, sub_type, table_type, val_type},
    GreenElement, GreenResult, Input,
};
use crate::error::{Message, SyntaxError};
use wat_syntax::SyntaxKind::{self, *};
use winnow::{
    combinator::{alt, dispatch, empty, fail, opt, peek, preceded, repeat},
    token::any,
    Parser,
};

pub(super) fn module(input: &mut Input) -> GreenResult {
    alt((
        (
            l_paren,
            trivias_prefixed(keyword("module")),
            opt(trivias_prefixed(ident)),
            repeat::<_, _, Vec<_>, _, _>(0.., retry_once(module_field, [])),
            r_paren,
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
            }),
        // wabt allows top-level module fields
        (
            module_field,
            repeat::<_, _, Vec<_>, _, _>(0.., retry_once(module_field, [])),
        )
            .map(|(first_field, fields)| {
                let mut children = vec![first_field];
                fields
                    .into_iter()
                    .for_each(|mut field| children.append(&mut field));
                node(MODULE, children)
            }),
    ))
    .parse_next(input)
}

fn module_field(input: &mut Input) -> GreenResult {
    dispatch! {peek(preceded(('(', trivias), word));
        "func" => module_field_func,
        "type" => type_def,
        "export" => module_field_export,
        "import" => module_field_import,
        "start" => module_field_start,
        "data" => module_field_data,
        "table" => module_field_table,
        "memory" => module_field_memory,
        "global" => module_field_global,
        "elem" => module_field_elem,
        _ => fail,
    }
    .context(Message::Name("module field"))
    .parse_next(input)
}

fn module_field_data(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("data")),
        opt(trivias_prefixed(ident)),
        opt(trivias_prefixed(mem_use)),
        opt(trivias_prefixed(offset)),
        repeat::<_, _, Vec<_>, _, _>(0.., retry_once(string, [])),
        r_paren,
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

fn module_field_elem(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("elem")),
        opt(trivias_prefixed(ident)),
        alt((
            (
                trivias_prefixed(keyword("declare")),
                must(retry_once(elem_list, [])),
            )
                .map(|(declare, elem_list)| (Some(declare), elem_list, None)),
            trivias_prefixed(elem_list).map(|elem_list| (Some(elem_list), None, None)),
            (
                opt(trivias_prefixed(table_use)),
                must(retry_once(offset, [])),
                opt(retry_once(elem_list, [])),
            ),
        )),
        r_paren,
    )
        .parse_next(input)
        .map(|(l_paren, mut keyword, id, (a, b, elem_list), r_paren)| {
            let mut children = Vec::with_capacity(8);
            children.push(l_paren);
            children.append(&mut keyword);
            if let Some(mut id) = id {
                children.append(&mut id);
            }
            if let Some(mut a) = a {
                children.append(&mut a);
            }
            if let Some(mut b) = b {
                children.append(&mut b);
            }
            if let Some(mut elem_list) = elem_list {
                children.append(&mut elem_list);
            }
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(MODULE_FIELD_ELEM, children)
        })
}

fn module_field_export(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("export")),
        must(trivias_prefixed(name)),
        must(retry_once(export_desc, [])),
        r_paren,
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
        repeat::<_, _, Vec<_>, _, _>(0.., trivias_prefixed(alt((import, export)))), // postpone syntax error for using import with export or instr
        opt(trivias_prefixed(type_use)),
        repeat::<_, _, Vec<_>, _, _>(0.., trivias_prefixed(local)),
        repeat::<_, _, Vec<_>, _, _>(0.., retry_once(instr, [])),
        r_paren,
    )
        .parse_next(input)
        .map(
            |(l_paren, mut keyword, id, imports_or_exports, type_use, locals, instrs, r_paren)| {
                let mut children = Vec::with_capacity(7);
                children.push(l_paren);
                children.append(&mut keyword);
                if let Some(mut id) = id {
                    children.append(&mut id);
                }
                imports_or_exports
                    .into_iter()
                    .for_each(|mut import_or_export| children.append(&mut import_or_export));
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
        must(retry_once(global_type, [])),
        repeat::<_, _, Vec<_>, _, _>(0.., retry_once(instr, [])),
        r_paren,
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
        must(retry(module_name, [])),
        must(retry(name, [])),
        must(retry_once(import_desc, [])),
        r_paren,
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

fn module_field_memory(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("memory")),
        opt(trivias_prefixed(ident)),
        opt(trivias_prefixed(import)), // postpone syntax error for using import with export or instr
        opt(trivias_prefixed(export)),
        dispatch! {peek(preceded(trivias, any));
            '(' => must(retry_once(data, [])),
            _ => must(retry_once(memory_type, [])),
        },
        r_paren,
    )
        .parse_next(input)
        .map(
            |(l_paren, mut keyword, id, import, export, mem_type_or_data, r_paren)| {
                let mut children = Vec::with_capacity(8);
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
                if let Some(mut mem_type_or_data) = mem_type_or_data {
                    children.append(&mut mem_type_or_data);
                }
                if let Some(mut r_paren) = r_paren {
                    children.append(&mut r_paren);
                }
                node(MODULE_FIELD_MEMORY, children)
            },
        )
}

fn module_field_start(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("start")),
        must(retry_once(index, [])),
        r_paren,
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

fn module_field_table(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("table")),
        opt(trivias_prefixed(ident)),
        opt(trivias_prefixed(import)), // postpone syntax error for using import with export or instr
        opt(trivias_prefixed(export)),
        must(alt((
            (trivias_prefixed(ref_type), must(retry_once(elem, []))),
            (retry(table_type, []), empty.value(None)),
        ))),
        r_paren,
    )
        .parse_next(input)
        .map(
            |(l_paren, mut keyword, id, import, export, ty_and_elem, r_paren)| {
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
                if let Some((mut ty, elem)) = ty_and_elem {
                    children.append(&mut ty);
                    if let Some(mut elem) = elem {
                        children.append(&mut elem);
                    }
                }
                if let Some(mut r_paren) = r_paren {
                    children.append(&mut r_paren);
                }
                node(MODULE_FIELD_TABLE, children)
            },
        )
}

fn type_def(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("type")),
        opt(trivias_prefixed(ident)),
        must(retry_once(sub_type, [])),
        r_paren,
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
            node(TYPE_DEF, children)
        })
}

pub(super) fn local(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("local")),
        alt((
            (trivias_prefixed(ident), retry(val_type, [])).map(|(mut children, mut ty)| {
                children.append(&mut ty);
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
            node(LOCAL, children)
        })
}

fn import(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("import")),
        must(trivias_prefixed(module_name)),
        must(trivias_prefixed(name)),
        r_paren,
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
        must(trivias_prefixed(name)),
        r_paren,
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
    .context(Message::Name("import descriptor"))
    .parse_next(input)
}

fn import_desc_global_type(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("global")),
        opt(trivias_prefixed(ident)),
        must(retry_once(global_type, [])),
        r_paren,
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
        must(retry(memory_type, [])),
        r_paren,
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
        must(retry(table_type, [])),
        r_paren,
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
        opt(retry_once(type_use, [])),
        r_paren,
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
    .context(Message::Name("export descriptor"))
    .parse_next(input)
}

fn export_desc_variant<'s>(
    keyword_literal: &'static str,
    kind: SyntaxKind,
) -> impl Parser<Input<'s>, GreenElement, SyntaxError> {
    (
        l_paren,
        trivias_prefixed(keyword(keyword_literal)),
        must(retry_once(index, [])),
        r_paren,
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
    alt((
        (
            l_paren,
            trivias_prefixed(keyword("type")),
            must(retry_once(index, [])),
            r_paren,
            repeat::<_, _, Vec<_>, _, _>(0.., trivias_prefixed(param)),
            repeat::<_, _, Vec<_>, _, _>(0.., trivias_prefixed(result)),
        )
            .map(|(l_paren, mut keyword, index, r_paren, params, results)| {
                let mut children = Vec::with_capacity(4);
                children.push(l_paren);
                children.append(&mut keyword);
                if let Some(mut index) = index {
                    children.append(&mut index);
                }
                if let Some(mut r_paren) = r_paren {
                    children.append(&mut r_paren);
                }
                params
                    .into_iter()
                    .for_each(|mut param| children.append(&mut param));
                results
                    .into_iter()
                    .for_each(|mut result| children.append(&mut result));
                node(TYPE_USE, children)
            }),
        (
            repeat::<_, _, Vec<_>, _, _>(1.., trivias_prefixed(param)),
            repeat::<_, _, Vec<_>, _, _>(0.., trivias_prefixed(result)),
        )
            .map(|(params, results)| {
                let mut children = Vec::with_capacity(2);
                params
                    .into_iter()
                    .for_each(|mut param| children.append(&mut param));
                results
                    .into_iter()
                    .for_each(|mut result| children.append(&mut result));
                node(TYPE_USE, children)
            }),
        repeat::<_, _, Vec<_>, _, _>(1.., trivias_prefixed(result))
            .map(|results| node(TYPE_USE, results.into_iter().flatten().collect::<Vec<_>>())),
    ))
    .parse_next(input)
}

fn elem(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("elem")),
        dispatch! {peek(preceded(trivias, any));
            '(' => repeat::<_, _, Vec<_>, _, _>(0.., retry_once(elem_expr, [])),
            ')' => empty.value(vec![]),
            _ => repeat::<_, _, Vec<_>, _, _>(0.., retry_once(index, [])),
        },
        r_paren,
    )
        .parse_next(input)
        .map(|(l_paren, mut keyword, items, r_paren)| {
            let mut children = Vec::with_capacity(4);
            children.push(l_paren);
            children.append(&mut keyword);
            items
                .into_iter()
                .for_each(|mut item| children.append(&mut item));
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(ELEM, children)
        })
}

fn elem_list(input: &mut Input) -> GreenResult {
    alt((
        (
            alt((keyword("func"), index)),
            repeat::<_, _, Vec<_>, _, _>(0.., retry_once(index, [])),
        )
            .map(|(keyword_or_first_idx, indexes)| {
                let mut children = Vec::with_capacity(2);
                children.push(keyword_or_first_idx);
                indexes
                    .into_iter()
                    .for_each(|mut idx| children.append(&mut idx));
                node(ELEM_LIST, children)
            }),
        (
            ref_type,
            repeat::<_, _, Vec<_>, _, _>(0.., retry_once(elem_expr, [])),
        )
            .map(|(ty, exprs)| {
                let mut children = Vec::with_capacity(2);
                children.push(ty);
                exprs
                    .into_iter()
                    .for_each(|mut expr| children.append(&mut expr));
                node(ELEM_LIST, children)
            }),
    ))
    .parse_next(input)
}

fn elem_expr(input: &mut Input) -> GreenResult {
    alt((
        (
            l_paren,
            trivias_prefixed(keyword("item")),
            repeat::<_, _, Vec<_>, _, _>(0.., retry(instr, [])),
            r_paren,
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
                node(ELEM_EXPR, children)
            }),
        preceded(peek('('), instr).map(|child| node(ELEM_EXPR, [child])),
    ))
    .parse_next(input)
}

pub(super) fn index(input: &mut Input) -> GreenResult {
    alt((ident, unsigned_int))
        .context(Message::Name("idx"))
        .parse_next(input)
        .map(|child| node(INDEX, [child]))
}

fn mem_use(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("memory")),
        must(trivias_prefixed(index)),
        r_paren,
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

fn table_use(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("table")),
        must(trivias_prefixed(index)),
        r_paren,
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
            node(TABLE_USE, children)
        })
}

fn offset(input: &mut Input) -> GreenResult {
    alt((
        (
            l_paren,
            trivias_prefixed(keyword("offset")),
            repeat::<_, _, Vec<_>, _, _>(0.., retry(instr, [])),
            r_paren,
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
    .context(Message::Name("offset"))
    .parse_next(input)
}

fn data(input: &mut Input) -> GreenResult {
    (
        l_paren,
        trivias_prefixed(keyword("data")),
        repeat::<_, _, Vec<_>, _, _>(0.., retry_once(string, [])),
        r_paren,
    )
        .parse_next(input)
        .map(|(l_paren, mut keyword, strings, r_paren)| {
            let mut children = Vec::with_capacity(4);
            children.push(l_paren);
            children.append(&mut keyword);
            strings
                .into_iter()
                .for_each(|mut string| children.append(&mut string));
            if let Some(mut r_paren) = r_paren {
                children.append(&mut r_paren);
            }
            node(DATA, children)
        })
}
