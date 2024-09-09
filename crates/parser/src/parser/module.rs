use super::{
    node, resume, retry,
    token::{ident, keyword, l_paren, r_paren, string, trivias, trivias_prefixed, word},
    ty::{func_type, memory_type, nat, param, result, table_type},
    GreenElement, GreenResult, Input,
};
use wat_syntax::SyntaxKind::{self, *};
use winnow::{
    combinator::{dispatch, fail, opt, peek, preceded, repeat, todo},
    error::{ContextError, StrContext, StrContextValue},
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
        "export" => module_field_export,
        "import" => module_field_import,
        "start" => module_field_start,
        "type" => module_field_type,
        _ => fail,
    }
    .parse_next(input)
    .map(|children| node(MODULE_FIELD, [children]))
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
        todo::<_, (), _>,
        resume(r_paren),
    )
        .parse_next(input)
        .map(|(l_paren, mut keyword, id, _global_type, r_paren)| {
            let mut children = Vec::with_capacity(5);
            children.push(l_paren);
            children.append(&mut keyword);
            if let Some(mut id) = id {
                children.append(&mut id);
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
) -> impl Parser<Input<'s>, GreenElement, ContextError> {
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

fn type_use(input: &mut Input) -> GreenResult {
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
    (resume(nat), resume(ident))
        .parse_next(input)
        .map(|(nat, id)| {
            let mut children = Vec::with_capacity(2);
            if let Some(mut nat) = nat {
                children.append(&mut nat);
            }
            if let Some(mut id) = id {
                children.append(&mut id);
            }
            node(INDEX, children)
        })
}
