use super::*;
use bumpalo::collections::Vec as BumpVec;
use tiny_pretty::Doc;
use wat_syntax::{SyntaxKind::*, ast::AstNode};

pub(crate) fn format_data<'a>(data: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = data.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, data, &mut trivias);
    }
    if let Some(keyword) = data.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("data"));
        ctx.format_trivias_after_token(keyword, data, &mut trivias);
    }
    data.tokens_by_kind(STRING).for_each(|string| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(string.text()));
        ctx.format_trivias_after_token(string, data, &mut trivias);
    });
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren_on_same_line(data),
    ]))
}

pub(crate) fn format_elem<'a>(elem: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = elem.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, elem, &mut trivias);
    }
    if let Some(keyword) = elem.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("elem"));
        ctx.format_trivias_after_token(keyword, elem, &mut trivias);
    }
    elem.children_by_kind(INDEX).for_each(|index| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_index(index));
        ctx.format_trivias_after_node(index, elem, &mut trivias);
    });
    elem.children_by_kind(ELEM_EXPR).for_each(|elem_expr| {
        let has_keyword = elem_expr.tokens_by_kind(KEYWORD).next().is_some();
        if trivias.is_empty() && has_keyword {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_elem_expr(elem_expr, ctx));
        ctx.format_trivias_after_node(elem_expr, elem, &mut trivias);
    });
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren(elem),
    ]))
    .group()
}

pub(crate) fn format_elem_expr<'a>(elem_expr: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = elem_expr.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, elem_expr, &mut trivias);
    }
    if let Some(keyword) = elem_expr.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("item"));
        ctx.format_trivias_after_token(keyword, elem_expr, &mut trivias);
    }
    format_const_expr(elem_expr, ctx, &mut docs, &mut trivias);
    if elem_expr.tokens_by_kind(R_PAREN).next().is_some() {
        docs.append(&mut trivias);
        Doc::slice(ctx.bump.alloc_slice_fill_iter([
            Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
            ctx.format_right_paren(elem_expr),
        ]))
        .group()
    } else {
        Doc::slice(docs.into_bump_slice())
    }
}

pub(crate) fn format_elem_list<'a>(elem_list: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(keyword) = elem_list.tokens_by_kind(KEYWORD).next() {
        docs.push(Doc::text("func"));
        ctx.format_trivias_after_token(keyword, elem_list, &mut trivias);
    }
    elem_list.children_by_kind(INDEX).for_each(|index| {
        if trivias.is_empty() {
            if !docs.is_empty() {
                docs.push(Doc::space());
            }
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_index(index));
        ctx.format_trivias_after_node(index, elem_list, &mut trivias);
    });
    if let Some(ref_type) = elem_list.children_by_kind(REF_TYPE).next() {
        docs.push(format_ref_type(ref_type, ctx));
        ctx.format_trivias_after_node(ref_type, elem_list, &mut trivias);
    }
    elem_list.children_by_kind(ELEM_EXPR).for_each(|elem_expr| {
        let has_keyword = elem_expr.tokens_by_kind(KEYWORD).next().is_some();
        if trivias.is_empty() && has_keyword {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_elem_expr(elem_expr, ctx));
        ctx.format_trivias_after_node(elem_expr, elem_list, &mut trivias);
    });
    docs.append(&mut trivias);
    Doc::slice(docs.into_bump_slice())
}

pub(crate) fn format_export<'a>(export: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = export.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, export, &mut trivias);
    }
    if let Some(keyword) = export.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("export"));
        ctx.format_trivias_after_token(keyword, export, &mut trivias);
    }
    if let Some(name) = export.children_by_kind(NAME).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_name(name));
        ctx.format_trivias_after_node(name, export, &mut trivias);
    }
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren_on_same_line(export),
    ]))
}

pub(crate) fn format_import<'a>(import: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = import.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, import, &mut trivias);
    }
    if let Some(keyword) = import.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("import"));
        ctx.format_trivias_after_token(keyword, import, &mut trivias);
    }
    if let Some(module_name) = import.children_by_kind(MODULE_NAME).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_module_name(module_name));
        ctx.format_trivias_after_node(module_name, import, &mut trivias);
    }
    if let Some(name) = import.children_by_kind(NAME).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_name(name));
        ctx.format_trivias_after_node(name, import, &mut trivias);
    }
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren_on_same_line(import),
    ]))
}

pub(crate) fn format_index<'a>(index: AmberNode<'a>) -> Doc<'a> {
    if let Some(token) = index.children_with_tokens().next().and_then(NodeOrToken::into_token) {
        Doc::text(token.text())
    } else {
        Doc::nil()
    }
}

pub(crate) fn format_local<'a>(local: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = local.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, local, &mut trivias);
    }
    if let Some(keyword) = local.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("local"));
        ctx.format_trivias_after_token(keyword, local, &mut trivias);
    }
    if let Some(ident) = local.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        ctx.format_trivias_after_token(ident, local, &mut trivias);
    }
    local.children_by_kind(ValType::can_cast).for_each(|val_type| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_val_type(val_type, ctx));
        ctx.format_trivias_after_node(val_type, local, &mut trivias);
    });
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren_on_same_line(local),
    ]))
}

pub(crate) fn format_mem_use<'a>(mem_use: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = mem_use.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, mem_use, &mut trivias);
    }
    if let Some(keyword) = mem_use.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("memory"));
        ctx.format_trivias_after_token(keyword, mem_use, &mut trivias);
    }
    if let Some(index) = mem_use.children_by_kind(INDEX).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_index(index));
        ctx.format_trivias_after_node(index, mem_use, &mut trivias);
    }
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren_on_same_line(mem_use),
    ]))
}

pub(crate) fn format_module<'a>(module: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    let mut is_explicit_module = true;
    if let Some(l_paren) = module.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, module, &mut trivias);
    } else {
        is_explicit_module = false;
    }
    if let Some(keyword) = module.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("module"));
        ctx.format_trivias_after_token(keyword, module, &mut trivias);
    }
    if let Some(ident) = module.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        ctx.format_trivias_after_token(ident, module, &mut trivias);
    }
    module.children_by_kind(ModuleField::can_cast).for_each(|module_field| {
        if trivias.is_empty() && !docs.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        if should_ignore(module_field, module, ctx) {
            reflow(module_field.green().to_string(), &mut docs);
        } else {
            match module_field.kind() {
                MODULE_FIELD_DATA => docs.push(format_module_field_data(module_field, ctx)),
                MODULE_FIELD_ELEM => docs.push(format_module_field_elem(module_field, ctx)),
                MODULE_FIELD_EXPORT => docs.push(format_module_field_export(module_field, ctx)),
                MODULE_FIELD_FUNC => docs.push(format_module_field_func(module_field, ctx)),
                MODULE_FIELD_GLOBAL => docs.push(format_module_field_global(module_field, ctx)),
                MODULE_FIELD_IMPORT => docs.push(format_module_field_import(module_field, ctx)),
                MODULE_FIELD_MEMORY => docs.push(format_module_field_memory(module_field, ctx)),
                MODULE_FIELD_START => docs.push(format_module_field_start(module_field, ctx)),
                MODULE_FIELD_TABLE => docs.push(format_module_field_table(module_field, ctx)),
                MODULE_FIELD_TAG => docs.push(format_module_field_tag(module_field, ctx)),
                TYPE_DEF => docs.push(format_type_def(module_field, ctx)),
                REC_TYPE => docs.push(format_rec_type(module_field, ctx)),
                _ => {}
            }
        }
        ctx.format_trivias_after_node(module_field, module, &mut trivias);
    });
    docs.append(&mut trivias);
    if is_explicit_module {
        Doc::slice(ctx.bump.alloc_slice_fill_iter([
            Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
            ctx.format_right_paren(module),
        ]))
        .group()
    } else {
        Doc::slice(docs.into_bump_slice())
    }
}

pub(crate) fn format_module_field_data<'a>(module_field_data: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = module_field_data.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, module_field_data, &mut trivias);
    }
    if let Some(keyword) = module_field_data.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("data"));
        ctx.format_trivias_after_token(keyword, module_field_data, &mut trivias);
    }
    if let Some(ident) = module_field_data.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        ctx.format_trivias_after_token(ident, module_field_data, &mut trivias);
    }
    if let Some(mem_use) = module_field_data.children_by_kind(MEM_USE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_mem_use(mem_use, ctx));
        ctx.format_trivias_after_node(mem_use, module_field_data, &mut trivias);
    }
    if let Some(offset) = module_field_data.children_by_kind(OFFSET).next() {
        let has_keyword = offset.tokens_by_kind(KEYWORD).next().is_some();
        if trivias.is_empty() && has_keyword {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_offset(offset, ctx));
        ctx.format_trivias_after_node(offset, module_field_data, &mut trivias);
    }
    module_field_data.tokens_by_kind(STRING).for_each(|string| {
        if trivias.is_empty() {
            docs.push(Doc::soft_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(string.text()));
        ctx.format_trivias_after_token(string, module_field_data, &mut trivias);
    });
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren(module_field_data),
    ]))
    .group()
}

pub(crate) fn format_module_field_elem<'a>(module_field_elem: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = module_field_elem.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, module_field_elem, &mut trivias);
    }
    if let Some(keyword) = module_field_elem.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("elem"));
        ctx.format_trivias_after_token(keyword, module_field_elem, &mut trivias);
    }
    if let Some(ident) = module_field_elem.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        ctx.format_trivias_after_token(ident, module_field_elem, &mut trivias);
    }
    if let Some(keyword) = module_field_elem.tokens_by_kind(MODIFIER_KEYWORD).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text("declare"));
        ctx.format_trivias_after_token(keyword, module_field_elem, &mut trivias);
    }
    if let Some(table_use) = module_field_elem.children_by_kind(TABLE_USE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_table_use(table_use, ctx));
        ctx.format_trivias_after_node(table_use, module_field_elem, &mut trivias);
    }
    if let Some(offset) = module_field_elem.children_by_kind(OFFSET).next() {
        let has_keyword = offset.tokens_by_kind(KEYWORD).next().is_some();
        if trivias.is_empty() && has_keyword {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_offset(offset, ctx));
        ctx.format_trivias_after_node(offset, module_field_elem, &mut trivias);
    }
    if let Some(elem_list) = module_field_elem.children_by_kind(ELEM_LIST).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_elem_list(elem_list, ctx));
        ctx.format_trivias_after_node(elem_list, module_field_elem, &mut trivias);
    }
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren(module_field_elem),
    ]))
    .group()
}

pub(crate) fn format_module_field_export<'a>(module_field_export: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = module_field_export.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, module_field_export, &mut trivias);
    }
    if let Some(keyword) = module_field_export.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("export"));
        ctx.format_trivias_after_token(keyword, module_field_export, &mut trivias);
    }
    if let Some(name) = module_field_export.children_by_kind(NAME).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_name(name));
        ctx.format_trivias_after_node(name, module_field_export, &mut trivias);
    }
    if let Some(extern_idx) = module_field_export.children_by_kind(ExternIdx::can_cast).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_extern_idx(extern_idx, ctx));
        ctx.format_trivias_after_node(extern_idx, module_field_export, &mut trivias);
    }
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren_on_same_line(module_field_export),
    ]))
}

pub(crate) fn format_module_field_func<'a>(module_field_func: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = module_field_func.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, module_field_func, &mut trivias);
    }
    if let Some(keyword) = module_field_func.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("func"));
        ctx.format_trivias_after_token(keyword, module_field_func, &mut trivias);
    }
    if let Some(ident) = module_field_func.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        ctx.format_trivias_after_token(ident, module_field_func, &mut trivias);
    }
    module_field_func.children_by_kind(EXPORT).for_each(|export| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_export(export, ctx));
        ctx.format_trivias_after_node(export, module_field_func, &mut trivias);
    });
    if let Some(import) = module_field_func.children_by_kind(IMPORT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_import(import, ctx));
        ctx.format_trivias_after_node(import, module_field_func, &mut trivias);
    }
    if let Some(type_use) = module_field_func.children_by_kind(TYPE_USE).next() {
        if trivias.is_empty() {
            if type_use.tokens_by_kind(KEYWORD).next().is_some() {
                docs.push(Doc::space());
            } else {
                docs.push(Doc::soft_line());
            }
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_type_use(type_use, ctx));
        ctx.format_trivias_after_node(type_use, module_field_func, &mut trivias);
    }
    let mut locals = module_field_func.children_by_kind(LOCAL).peekable();
    let mut locals_docs = BumpVec::with_capacity_in(1, &ctx.bump);
    let ws_of_multi_line =
        whitespace_of_multi_line(ctx.options.multi_line_locals, locals.peek().copied(), module_field_func);
    if let Some(local) = locals.next() {
        if trivias.is_empty() {
            docs.push(wrap_before(&mut locals, ctx.options.wrap_before_locals));
        } else {
            docs.append(&mut trivias);
        }
        locals_docs.push(format_local(local, ctx));
        ctx.format_trivias_after_node(local, module_field_func, &mut trivias);
    }
    locals.for_each(|local| {
        if trivias.is_empty() {
            locals_docs.push(ws_of_multi_line.clone());
        } else {
            locals_docs.append(&mut trivias);
        }
        locals_docs.push(format_local(local, ctx));
        ctx.format_trivias_after_node(local, module_field_func, &mut trivias);
    });
    docs.push(Doc::slice(locals_docs.into_bump_slice()).group());
    module_field_func.children_by_kind(Instr::can_cast).for_each(|instr| {
        if trivias.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_instr(instr, ctx));
        ctx.format_trivias_after_node(instr, module_field_func, &mut trivias);
    });
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren_after_instr(module_field_func),
    ]))
}

pub(crate) fn format_module_field_global<'a>(module_field_global: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = module_field_global.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, module_field_global, &mut trivias);
    }
    if let Some(keyword) = module_field_global.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("global"));
        ctx.format_trivias_after_token(keyword, module_field_global, &mut trivias);
    }
    if let Some(ident) = module_field_global.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        ctx.format_trivias_after_token(ident, module_field_global, &mut trivias);
    }
    module_field_global.children_by_kind(EXPORT).for_each(|export| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_export(export, ctx));
        ctx.format_trivias_after_node(export, module_field_global, &mut trivias);
    });
    if let Some(import) = module_field_global.children_by_kind(IMPORT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_import(import, ctx));
        ctx.format_trivias_after_node(import, module_field_global, &mut trivias);
    }
    if let Some(global_type) = module_field_global.children_by_kind(GLOBAL_TYPE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_global_type(global_type, ctx));
        ctx.format_trivias_after_node(global_type, module_field_global, &mut trivias);
    }
    format_const_expr(module_field_global, ctx, &mut docs, &mut trivias);
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren_after_instr(module_field_global),
    ]))
}

pub(crate) fn format_module_field_import<'a>(module_field_import: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = module_field_import.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, module_field_import, &mut trivias);
    }
    if let Some(keyword) = module_field_import.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("import"));
        ctx.format_trivias_after_token(keyword, module_field_import, &mut trivias);
    }
    if let Some(module_name) = module_field_import.children_by_kind(MODULE_NAME).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_module_name(module_name));
        ctx.format_trivias_after_node(module_name, module_field_import, &mut trivias);
    }
    if let Some(name) = module_field_import.children_by_kind(NAME).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_name(name));
        ctx.format_trivias_after_node(name, module_field_import, &mut trivias);
    }
    if let Some(extern_type) = module_field_import.children_by_kind(ExternType::can_cast).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        match extern_type.kind() {
            EXTERN_TYPE_FUNC => docs.push(format_extern_type_func(extern_type, ctx)),
            EXTERN_TYPE_GLOBAL => docs.push(format_extern_type_global(extern_type, ctx)),
            EXTERN_TYPE_MEMORY => docs.push(format_extern_type_memory(extern_type, ctx)),
            EXTERN_TYPE_TABLE => docs.push(format_extern_type_table(extern_type, ctx)),
            EXTERN_TYPE_TAG => docs.push(format_extern_type_tag(extern_type, ctx)),
            _ => {}
        }
        ctx.format_trivias_after_node(extern_type, module_field_import, &mut trivias);
    }
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren_on_same_line(module_field_import),
    ]))
}

pub(crate) fn format_module_field_memory<'a>(module_field_memory: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = module_field_memory.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, module_field_memory, &mut trivias);
    }
    if let Some(keyword) = module_field_memory.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("memory"));
        ctx.format_trivias_after_token(keyword, module_field_memory, &mut trivias);
    }
    if let Some(ident) = module_field_memory.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        ctx.format_trivias_after_token(ident, module_field_memory, &mut trivias);
    }
    module_field_memory.children_by_kind(EXPORT).for_each(|export| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_export(export, ctx));
        ctx.format_trivias_after_node(export, module_field_memory, &mut trivias);
    });
    if let Some(import) = module_field_memory.children_by_kind(IMPORT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_import(import, ctx));
        ctx.format_trivias_after_node(import, module_field_memory, &mut trivias);
    }
    if let Some(mem_type) = module_field_memory.children_by_kind(MEM_TYPE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_mem_type(mem_type, ctx));
        ctx.format_trivias_after_node(mem_type, module_field_memory, &mut trivias);
    }
    if let Some(data) = module_field_memory.children_by_kind(DATA).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_data(data, ctx));
        ctx.format_trivias_after_node(data, module_field_memory, &mut trivias);
    }
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren_on_same_line(module_field_memory),
    ]))
}

pub(crate) fn format_module_field_start<'a>(module_field_start: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = module_field_start.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, module_field_start, &mut trivias);
    }
    if let Some(keyword) = module_field_start.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("start"));
        ctx.format_trivias_after_token(keyword, module_field_start, &mut trivias);
    }
    if let Some(index) = module_field_start.children_by_kind(INDEX).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_index(index));
        ctx.format_trivias_after_node(index, module_field_start, &mut trivias);
    }
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren_on_same_line(module_field_start),
    ]))
}

pub(crate) fn format_module_field_table<'a>(module_field_table: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = module_field_table.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, module_field_table, &mut trivias);
    }
    if let Some(keyword) = module_field_table.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("table"));
        ctx.format_trivias_after_token(keyword, module_field_table, &mut trivias);
    }
    if let Some(ident) = module_field_table.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        ctx.format_trivias_after_token(ident, module_field_table, &mut trivias);
    }
    module_field_table.children_by_kind(EXPORT).for_each(|export| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_export(export, ctx));
        ctx.format_trivias_after_node(export, module_field_table, &mut trivias);
    });
    if let Some(import) = module_field_table.children_by_kind(IMPORT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_import(import, ctx));
        ctx.format_trivias_after_node(import, module_field_table, &mut trivias);
    }
    if let Some(table_type) = module_field_table.children_by_kind(TABLE_TYPE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_table_type(table_type, ctx));
        ctx.format_trivias_after_node(table_type, module_field_table, &mut trivias);
    }
    if let Some(ref_type) = module_field_table.children_by_kind(REF_TYPE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_ref_type(ref_type, ctx));
        ctx.format_trivias_after_node(ref_type, module_field_table, &mut trivias);
    }
    if let Some(elem) = module_field_table.children_by_kind(ELEM).next() {
        if trivias.is_empty() {
            docs.push(Doc::line_or_space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_elem(elem, ctx));
        ctx.format_trivias_after_node(elem, module_field_table, &mut trivias);
    }
    format_const_expr(module_field_table, ctx, &mut docs, &mut trivias);
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren(module_field_table),
    ]))
    .group()
}

pub(crate) fn format_module_field_tag<'a>(module_field_tag: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = module_field_tag.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, module_field_tag, &mut trivias);
    }
    if let Some(keyword) = module_field_tag.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("tag"));
        ctx.format_trivias_after_token(keyword, module_field_tag, &mut trivias);
    }
    if let Some(ident) = module_field_tag.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        ctx.format_trivias_after_token(ident, module_field_tag, &mut trivias);
    }
    module_field_tag.children_by_kind(EXPORT).for_each(|export| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_export(export, ctx));
        ctx.format_trivias_after_node(export, module_field_tag, &mut trivias);
    });
    if let Some(import) = module_field_tag.children_by_kind(IMPORT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_import(import, ctx));
        ctx.format_trivias_after_node(import, module_field_tag, &mut trivias);
    }
    if let Some(type_use) = module_field_tag.children_by_kind(TYPE_USE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_type_use(type_use, ctx));
        ctx.format_trivias_after_node(type_use, module_field_tag, &mut trivias);
    }
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren_on_same_line(module_field_tag),
    ]))
}

pub(crate) fn format_module_name<'a>(module_name: AmberNode<'a>) -> Doc<'a> {
    if let Some(string) = module_name.tokens_by_kind(STRING).next() {
        Doc::text(string.text())
    } else {
        Doc::nil()
    }
}

pub(crate) fn format_name<'a>(name: AmberNode<'a>) -> Doc<'a> {
    if let Some(string) = name.tokens_by_kind(STRING).next() {
        Doc::text(string.text())
    } else {
        Doc::nil()
    }
}

pub(crate) fn format_offset<'a>(offset: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = offset.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, offset, &mut trivias);
    }
    if let Some(keyword) = offset.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("offset"));
        ctx.format_trivias_after_token(keyword, offset, &mut trivias);
    }
    format_const_expr(offset, ctx, &mut docs, &mut trivias);
    if offset.tokens_by_kind(R_PAREN).next().is_some() {
        docs.append(&mut trivias);
        Doc::slice(ctx.bump.alloc_slice_fill_iter([
            Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
            ctx.format_right_paren(offset),
        ]))
        .group()
    } else {
        Doc::slice(docs.into_bump_slice())
    }
}

pub(crate) fn format_rec_type<'a>(rec_type: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = rec_type.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, rec_type, &mut trivias);
    }
    if let Some(keyword) = rec_type.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("rec"));
        ctx.format_trivias_after_token(keyword, rec_type, &mut trivias);
    }
    rec_type.children_by_kind(TYPE_DEF).for_each(|type_def| {
        if trivias.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_type_def(type_def, ctx));
        ctx.format_trivias_after_node(type_def, rec_type, &mut trivias);
    });
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren(rec_type),
    ]))
    .group()
}

pub(crate) fn format_table_use<'a>(table_use: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = table_use.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, table_use, &mut trivias);
    }
    if let Some(keyword) = table_use.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("table"));
        ctx.format_trivias_after_token(keyword, table_use, &mut trivias);
    }
    if let Some(index) = table_use.children_by_kind(INDEX).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_index(index));
        ctx.format_trivias_after_node(index, table_use, &mut trivias);
    }
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren_on_same_line(table_use),
    ]))
}

pub(crate) fn format_type_def<'a>(type_def: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = type_def.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, type_def, &mut trivias);
    }
    if let Some(keyword) = type_def.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("type"));
        ctx.format_trivias_after_token(keyword, type_def, &mut trivias);
    }
    if let Some(ident) = type_def.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        ctx.format_trivias_after_token(ident, type_def, &mut trivias);
    }
    if let Some(sub_type) = type_def.children_by_kind(SUB_TYPE).next() {
        if trivias.is_empty() {
            docs.push(Doc::line_or_space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_sub_type(sub_type, ctx));
        ctx.format_trivias_after_node(sub_type, type_def, &mut trivias);
    }
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren(type_def),
    ]))
    .group()
}

pub(crate) fn format_type_use<'a>(type_use: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = type_use.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, type_use, &mut trivias);
    }
    if let Some(keyword) = type_use.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("type"));
        ctx.format_trivias_after_token(keyword, type_use, &mut trivias);
    }
    if let Some(index) = type_use.children_by_kind(INDEX).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_index(index));
        ctx.format_trivias_after_node(index, type_use, &mut trivias);
    }
    if let Some(r_paren) = type_use.tokens_by_kind(R_PAREN).next() {
        docs.append(&mut trivias);
        docs.push(ctx.format_right_paren_on_same_line(type_use));
        ctx.format_trivias_after_token(r_paren, type_use, &mut trivias);
    }

    let mut params = type_use.children_by_kind(PARAM);
    if let Some(param) = params.next() {
        if trivias.is_empty() && !docs.is_empty() {
            docs.push(Doc::soft_line());
        } else if type_use.tokens_by_kind(L_PAREN).next().is_some() {
            docs.append(&mut trivias);
        }
        docs.push(format_param(param, ctx));
        ctx.format_trivias_after_node(param, type_use, &mut trivias);
    }
    params.for_each(|param| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_param(param, ctx));
        ctx.format_trivias_after_node(param, type_use, &mut trivias);
    });
    let mut results = type_use.children_by_kind(RESULT);
    if let Some(result) = results.next() {
        if trivias.is_empty() && !docs.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_result(result, ctx));
        ctx.format_trivias_after_node(result, type_use, &mut trivias);
    }
    results.for_each(|result| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_result(result, ctx));
        ctx.format_trivias_after_node(result, type_use, &mut trivias);
    });

    Doc::slice(docs.into_bump_slice())
}

pub(crate) fn format_extern_idx<'a>(extern_idx: AmberNode<'a>, ctx: &'a Ctx) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = extern_idx.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        ctx.format_trivias_after_token(l_paren, extern_idx, &mut trivias);
    }
    if let Some(keyword) = extern_idx.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text(keyword.text()));
        ctx.format_trivias_after_token(keyword, extern_idx, &mut trivias);
    }
    if let Some(index) = extern_idx.children_by_kind(INDEX).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_index(index));
        ctx.format_trivias_after_node(index, extern_idx, &mut trivias);
    }
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren_on_same_line(extern_idx),
    ]))
}

fn format_const_expr<'a>(
    parent: AmberNode<'a>,
    ctx: &'a Ctx,
    docs: &mut BumpVec<'a, Doc<'a>>,
    trivias: &mut BumpVec<'a, Doc<'a>>,
) {
    let mut instrs = parent.children_by_kind(Instr::can_cast).peekable();
    if let Some(instr) = instrs.next() {
        if trivias.is_empty() {
            if matches!(ctx.options.wrap_before_const_expr, crate::config::WrapBefore::MultiOnly)
                && instr.children_by_kind(Instr::can_cast).next().is_some()
            {
                docs.push(Doc::hard_line());
            } else {
                docs.push(wrap_before(&mut instrs, ctx.options.wrap_before_const_expr));
            }
        } else {
            docs.append(trivias);
        }
        docs.push(format_instr(instr, ctx));
        ctx.format_trivias_after_node(instr, parent, trivias);
    }
    instrs.for_each(|instr| {
        if trivias.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(trivias);
        }
        docs.push(format_instr(instr, ctx));
        ctx.format_trivias_after_node(instr, parent, trivias);
    });
}
