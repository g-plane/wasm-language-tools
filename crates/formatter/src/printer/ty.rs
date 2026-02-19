use super::*;
use tiny_pretty::Doc;
use wat_syntax::{NodeOrToken, SyntaxKind::*};

pub(crate) fn format_addr_type<'a>(addr_type: AmberNode<'a>) -> Doc<'a> {
    if let Some(token) = addr_type
        .children_with_tokens()
        .next()
        .and_then(NodeOrToken::into_token)
    {
        Doc::text(token.text())
    } else {
        Doc::nil()
    }
}

pub(crate) fn format_array_type<'a>(array_type: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = array_type.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, array_type, ctx);
    }
    if let Some(keyword) = array_type.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("array"));
        trivias = format_trivias_after_token(keyword, array_type, ctx);
    }
    if let Some(field_type) = array_type.children_by_kind(FIELD_TYPE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_field_type(field_type, ctx));
        trivias = format_trivias_after_node(field_type, array_type, ctx);
    }
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(array_type))
        .group()
}

pub(crate) fn format_comp_type<'a>(comp_type: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    match comp_type.kind() {
        FUNC_TYPE => format_func_type(comp_type, ctx),
        STRUCT_TYPE => format_struct_type(comp_type, ctx),
        ARRAY_TYPE => format_array_type(comp_type, ctx),
        _ => Doc::nil(),
    }
}

pub(crate) fn format_extern_type_func<'a>(func: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    format_extern_type(
        func,
        func.children_by_kind(TYPE_USE)
            .next()
            .map(|type_use| (type_use, format_type_use(type_use, ctx))),
        ctx,
    )
}

pub(crate) fn format_extern_type_global<'a>(global: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    format_extern_type(
        global,
        global
            .children_by_kind(GLOBAL_TYPE)
            .next()
            .map(|global_type| (global_type, format_global_type(global_type, ctx))),
        ctx,
    )
}

pub(crate) fn format_extern_type_memory<'a>(memory: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    format_extern_type(
        memory,
        memory
            .children_by_kind(MEM_TYPE)
            .next()
            .map(|mem_type| (mem_type, format_mem_type(mem_type, ctx))),
        ctx,
    )
}

pub(crate) fn format_extern_type_table<'a>(table: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    format_extern_type(
        table,
        table
            .children_by_kind(TABLE_TYPE)
            .next()
            .map(|table_type| (table_type, format_table_type(table_type, ctx))),
        ctx,
    )
}

pub(crate) fn format_extern_type_tag<'a>(tag: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    format_extern_type(
        tag,
        tag.children_by_kind(TYPE_USE)
            .next()
            .map(|type_use| (type_use, format_type_use(type_use, ctx))),
        ctx,
    )
}

pub(crate) fn format_field<'a>(field: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = field.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, field, ctx);
    }
    if let Some(keyword) = field.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("field"));
        trivias = format_trivias_after_token(keyword, field, ctx);
    }
    if let Some(ident) = field.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        trivias = format_trivias_after_token(ident, field, ctx);
    }
    field.children_by_kind(FIELD_TYPE).for_each(|field_type| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_field_type(field_type, ctx));
        trivias = format_trivias_after_node(field_type, field, ctx);
    });
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(field))
        .group()
}

pub(crate) fn format_field_type<'a>(field_type: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    if let Some(l_paren) = field_type.tokens_by_kind(L_PAREN).next() {
        let mut docs = Vec::with_capacity(2);
        docs.push(Doc::text("("));
        let mut trivias = format_trivias_after_token(l_paren, field_type, ctx);
        if let Some(keyword) = field_type.tokens_by_kind(KEYWORD).next() {
            docs.append(&mut trivias);
            docs.push(Doc::text("mut"));
            trivias = format_trivias_after_token(keyword, field_type, ctx);
        }
        if let Some(ty) = field_type.children_by_kind(StorageType::can_cast).next() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(format_storage_type(ty, ctx));
            trivias = format_trivias_after_node(ty, field_type, ctx);
        }
        docs.append(&mut trivias);
        Doc::list(docs)
            .nest(ctx.indent_width)
            .append(ctx.format_right_paren(field_type))
            .group()
    } else if let Some(ty) = field_type.children_by_kind(StorageType::can_cast).next() {
        format_storage_type(ty, ctx)
    } else {
        Doc::nil()
    }
}

pub(crate) fn format_func_type<'a>(func_type: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = func_type.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, func_type, ctx);
    }
    if let Some(keyword) = func_type.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("func"));
        trivias = format_trivias_after_token(keyword, func_type, ctx);
    }
    func_type.children_by_kind(PARAM).for_each(|param| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_param(param, ctx));
        trivias = format_trivias_after_node(param, func_type, ctx);
    });
    func_type.children_by_kind(RESULT).for_each(|result| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_result(result, ctx));
        trivias = format_trivias_after_node(result, func_type, ctx);
    });
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(func_type))
        .group()
}

pub(crate) fn format_global_type<'a>(global_type: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    if let Some(l_paren) = global_type.tokens_by_kind(L_PAREN).next() {
        let mut docs = Vec::with_capacity(2);
        docs.push(Doc::text("("));
        let mut trivias = format_trivias_after_token(l_paren, global_type, ctx);
        if let Some(keyword) = global_type.tokens_by_kind(KEYWORD).next() {
            docs.append(&mut trivias);
            docs.push(Doc::text("mut"));
            trivias = format_trivias_after_token(keyword, global_type, ctx);
        }
        if let Some(ty) = global_type.children_by_kind(ValType::can_cast).next() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(format_val_type(ty, ctx));
            trivias = format_trivias_after_node(ty, global_type, ctx);
        }
        docs.append(&mut trivias);
        Doc::list(docs)
            .nest(ctx.indent_width)
            .append(ctx.format_right_paren(global_type))
            .group()
    } else if let Some(ty) = global_type.children_by_kind(ValType::can_cast).next() {
        format_val_type(ty, ctx)
    } else {
        Doc::nil()
    }
}

pub(crate) fn format_heap_type<'a>(heap_type: AmberNode<'a>) -> Doc<'a> {
    if let Some(type_keyword) = heap_type.tokens_by_kind(TYPE_KEYWORD).next() {
        Doc::text(type_keyword.text())
    } else if let Some(index) = heap_type.children_by_kind(INDEX).next() {
        format_index(index)
    } else {
        Doc::nil()
    }
}

pub(crate) fn format_limits<'a>(limits: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    let mut uints = limits.tokens_by_kind(UNSIGNED_INT);
    if let Some(min) = uints.next() {
        docs.push(Doc::text(min.text()));
        trivias = format_trivias_after_token(min, limits, ctx);
    }
    if let Some(max) = uints.next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(max.text()));
    }
    Doc::list(docs)
}

pub(crate) fn format_mem_page_size<'a>(mem_page_size: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    let mut docs = Vec::with_capacity(5);
    let mut trivias = vec![];
    if let Some(l_paren) = mem_page_size.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, mem_page_size, ctx);
    }
    if let Some(keyword) = mem_page_size.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("pagesize"));
        trivias = format_trivias_after_token(keyword, mem_page_size, ctx);
    }
    if let Some(unsigned_int) = mem_page_size.tokens_by_kind(UNSIGNED_INT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(unsigned_int.text()));
        trivias = format_trivias_after_token(unsigned_int, mem_page_size, ctx);
    }
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(mem_page_size))
        .group()
}

pub(crate) fn format_mem_type<'a>(mem_type: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(addr_type) = mem_type.children_by_kind(ADDR_TYPE).next() {
        docs.push(format_addr_type(addr_type));
        trivias = format_trivias_after_node(addr_type, mem_type, ctx);
    }
    if let Some(limits) = mem_type.children_by_kind(LIMITS).next() {
        if trivias.is_empty() && !docs.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_limits(limits, ctx));
        trivias = format_trivias_after_node(limits, mem_type, ctx);
    }
    if let Some(share) = mem_type.tokens_by_kind(KEYWORD).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(share.text()));
        trivias = format_trivias_after_token(share, mem_type, ctx);
    }
    if let Some(mem_page_size) = mem_type.children_by_kind(MEM_PAGE_SIZE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_mem_page_size(mem_page_size, ctx));
    }
    Doc::list(docs)
}

pub(crate) fn format_num_type<'a>(num_type: AmberNode<'a>) -> Doc<'a> {
    if let Some(token) = num_type.tokens_by_kind(TYPE_KEYWORD).next() {
        Doc::text(token.text())
    } else {
        Doc::nil()
    }
}

pub(crate) fn format_packed_type<'a>(packed_type: AmberNode<'a>) -> Doc<'a> {
    if let Some(token) = packed_type.tokens_by_kind(TYPE_KEYWORD).next() {
        Doc::text(token.text())
    } else {
        Doc::nil()
    }
}

pub(crate) fn format_param<'a>(param: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = param.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, param, ctx);
    }
    if let Some(keyword) = param.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("param"));
        trivias = format_trivias_after_token(keyword, param, ctx);
    }
    if let Some(ident) = param.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        trivias = format_trivias_after_token(ident, param, ctx);
    }
    param.children_by_kind(ValType::can_cast).for_each(|val_type| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_val_type(val_type, ctx));
        trivias = format_trivias_after_node(val_type, param, ctx);
    });
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(param))
        .group()
}

pub(crate) fn format_ref_type<'a>(ref_type: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    if let Some(l_paren) = ref_type.tokens_by_kind(L_PAREN).next() {
        let mut docs = Vec::with_capacity(2);
        docs.push(Doc::text("("));
        let mut trivias = format_trivias_after_token(l_paren, ref_type, ctx);
        if let Some(keyword) = ref_type.tokens_by_kind(KEYWORD).next() {
            docs.append(&mut trivias);
            docs.push(Doc::text("ref"));
            trivias = format_trivias_after_token(keyword, ref_type, ctx);
        }
        if let Some(keyword) = ref_type.tokens_by_kind(MODIFIER_KEYWORD).next() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(Doc::text("null"));
            trivias = format_trivias_after_token(keyword, ref_type, ctx);
        }
        if let Some(heap_type) = ref_type.children_by_kind(HEAP_TYPE).next() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(format_heap_type(heap_type));
            trivias = format_trivias_after_node(heap_type, ref_type, ctx);
        }
        docs.append(&mut trivias);
        Doc::list(docs)
            .nest(ctx.indent_width)
            .append(ctx.format_right_paren(ref_type))
            .group()
    } else if let Some(type_keyword) = ref_type.tokens_by_kind(TYPE_KEYWORD).next() {
        Doc::text(type_keyword.text())
    } else {
        Doc::nil()
    }
}

pub(crate) fn format_result<'a>(result: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = result.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, result, ctx);
    }
    if let Some(keyword) = result.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("result"));
        trivias = format_trivias_after_token(keyword, result, ctx);
    }
    result.children_by_kind(ValType::can_cast).for_each(|val_type| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_val_type(val_type, ctx));
        trivias = format_trivias_after_node(val_type, result, ctx);
    });
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(result))
        .group()
}

pub(crate) fn format_storage_type<'a>(storage_type: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    if storage_type.kind() == PACKED_TYPE {
        format_packed_type(storage_type)
    } else {
        format_val_type(storage_type, ctx)
    }
}

pub(crate) fn format_struct_type<'a>(struct_type: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = struct_type.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, struct_type, ctx);
    }
    if let Some(keyword) = struct_type.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("struct"));
        trivias = format_trivias_after_token(keyword, struct_type, ctx);
    }
    let mut fields = struct_type.children_by_kind(FIELD).peekable();
    let mut fields_docs = Vec::with_capacity(1);
    let ws_of_multi_line = whitespace_of_multi_line(ctx.options.multi_line_fields, fields.peek().copied(), struct_type);
    if let Some(field) = fields.next() {
        if trivias.is_empty() {
            docs.push(wrap_before(&mut fields, ctx.options.wrap_before_fields));
        } else {
            docs.append(&mut trivias);
        }
        fields_docs.push(format_field(field, ctx));
        trivias = format_trivias_after_node(field, struct_type, ctx);
    }
    fields.for_each(|field| {
        if trivias.is_empty() {
            fields_docs.push(ws_of_multi_line.clone());
        } else {
            fields_docs.append(&mut trivias);
        }
        fields_docs.push(format_field(field, ctx));
        trivias = format_trivias_after_node(field, struct_type, ctx);
    });
    docs.push(Doc::list(fields_docs).group());
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(struct_type))
        .group()
}

pub(crate) fn format_sub_type<'a>(sub_type: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    if let Some(l_paren) = sub_type.tokens_by_kind(L_PAREN).next() {
        let mut docs = Vec::with_capacity(2);
        docs.push(Doc::text("("));
        let mut trivias = format_trivias_after_token(l_paren, sub_type, ctx);
        if let Some(keyword) = sub_type.tokens_by_kind(KEYWORD).next() {
            docs.append(&mut trivias);
            docs.push(Doc::text("sub"));
            trivias = format_trivias_after_token(keyword, sub_type, ctx);
        }
        if let Some(keyword) = sub_type.tokens_by_kind(KEYWORD).find(|token| token.text() == "final") {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(Doc::text("final"));
            trivias = format_trivias_after_token(keyword, sub_type, ctx);
        }
        sub_type.children_by_kind(INDEX).for_each(|index| {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(format_index(index));
            trivias = format_trivias_after_node(index, sub_type, ctx);
        });
        if let Some(ty) = sub_type.children_by_kind(CompType::can_cast).next() {
            if trivias.is_empty() {
                if ty.kind() == STRUCT_TYPE {
                    match ctx.options.wrap_before_fields {
                        WrapBefore::Never => docs.push(Doc::space()),
                        WrapBefore::Overflow => docs.push(Doc::line_or_space()),
                        WrapBefore::MultiOnly => {
                            if ty.children_by_kind(FIELD).count() > 1 {
                                docs.push(Doc::hard_line());
                            } else {
                                docs.push(Doc::space());
                            }
                        }
                        WrapBefore::Always => docs.push(Doc::hard_line()),
                    }
                } else {
                    docs.push(Doc::space());
                }
            } else {
                docs.append(&mut trivias);
            }
            docs.push(format_comp_type(ty, ctx));
            trivias = format_trivias_after_node(ty, sub_type, ctx);
        }
        docs.append(&mut trivias);
        Doc::list(docs)
            .nest(ctx.indent_width)
            .append(ctx.format_right_paren(sub_type))
            .group()
    } else if let Some(comp_type) = sub_type.children_by_kind(CompType::can_cast).next() {
        format_comp_type(comp_type, ctx)
    } else {
        Doc::nil()
    }
}

pub(crate) fn format_table_type<'a>(table_type: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(addr_type) = table_type.children_by_kind(ADDR_TYPE).next() {
        docs.push(format_addr_type(addr_type));
        trivias = format_trivias_after_node(addr_type, table_type, ctx);
    }
    if let Some(limits) = table_type.children_by_kind(LIMITS).next() {
        if trivias.is_empty() && !docs.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_limits(limits, ctx));
        trivias = format_trivias_after_node(limits, table_type, ctx);
    }
    if let Some(ref_type) = table_type.children_by_kind(REF_TYPE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_ref_type(ref_type, ctx));
    }
    Doc::list(docs)
}

pub(crate) fn format_val_type<'a>(val_type: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    match val_type.kind() {
        NUM_TYPE => format_num_type(val_type),
        VEC_TYPE => format_vec_type(val_type),
        REF_TYPE => format_ref_type(val_type, ctx),
        _ => Doc::nil(),
    }
}

pub(crate) fn format_vec_type<'a>(vec_type: AmberNode<'a>) -> Doc<'a> {
    if let Some(token) = vec_type.tokens_by_kind(TYPE_KEYWORD).next() {
        Doc::text(token.text())
    } else {
        Doc::nil()
    }
}

fn format_extern_type<'a>(node: AmberNode<'a>, ty: Option<(AmberNode<'a>, Doc<'a>)>, ctx: &Ctx) -> Doc<'a> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = node.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, node, ctx);
    }
    if let Some(keyword) = node.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text(keyword.text()));
        trivias = format_trivias_after_token(keyword, node, ctx);
    }
    if let Some(ident) = node.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        trivias = format_trivias_after_token(ident, node, ctx);
    }
    if let Some((ty, doc)) = ty {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(doc);
        trivias = format_trivias_after_node(ty, node, ctx);
    }
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(node))
        .group()
}
