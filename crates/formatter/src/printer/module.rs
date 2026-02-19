use super::*;
use tiny_pretty::Doc;
use wat_syntax::{SyntaxKind::*, ast::AstNode};

pub(crate) fn format_data(data: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = data.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, data, ctx);
    }
    if let Some(keyword) = data.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("data"));
        trivias = format_trivias_after_token(keyword, data, ctx);
    }
    data.tokens_by_kind(STRING).for_each(|string| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(string.text().to_string()));
        trivias = format_trivias_after_token(string, data, ctx);
    });
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(data))
        .group()
}

pub(crate) fn format_elem(elem: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = elem.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, elem, ctx);
    }
    if let Some(keyword) = elem.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("elem"));
        trivias = format_trivias_after_token(keyword, elem, ctx);
    }
    elem.children_by_kind(INDEX).for_each(|index| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_index(index));
        trivias = format_trivias_after_node(index, elem, ctx);
    });
    elem.children_by_kind(ELEM_EXPR).for_each(|elem_expr| {
        let has_keyword = elem_expr.tokens_by_kind(KEYWORD).next().is_some();
        if trivias.is_empty() && has_keyword {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_elem_expr(elem_expr, ctx));
        trivias = format_trivias_after_node(elem_expr, elem, ctx);
    });
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(elem))
        .group()
}

pub(crate) fn format_elem_expr(elem_expr: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = elem_expr.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, elem_expr, ctx);
    }
    if let Some(keyword) = elem_expr.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("item"));
        trivias = format_trivias_after_token(keyword, elem_expr, ctx);
    }
    format_const_expr(elem_expr, ctx, &mut docs, &mut trivias);
    if elem_expr.tokens_by_kind(R_PAREN).next().is_some() {
        docs.append(&mut trivias);
        Doc::list(docs)
            .nest(ctx.indent_width)
            .append(ctx.format_right_paren(elem_expr))
            .group()
    } else {
        Doc::list(docs)
    }
}

pub(crate) fn format_elem_list(elem_list: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(keyword) = elem_list.tokens_by_kind(KEYWORD).next() {
        docs.push(Doc::text("func"));
        trivias = format_trivias_after_token(keyword, elem_list, ctx);
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
        trivias = format_trivias_after_node(index, elem_list, ctx);
    });
    if let Some(ref_type) = elem_list.children_by_kind(REF_TYPE).next() {
        docs.push(format_ref_type(ref_type, ctx));
        trivias = format_trivias_after_node(ref_type, elem_list, ctx);
    }
    elem_list.children_by_kind(ELEM_EXPR).for_each(|elem_expr| {
        let has_keyword = elem_expr.tokens_by_kind(KEYWORD).next().is_some();
        if trivias.is_empty() && has_keyword {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_elem_expr(elem_expr, ctx));
        trivias = format_trivias_after_node(elem_expr, elem_list, ctx);
    });
    docs.append(&mut trivias);
    Doc::list(docs)
}

pub(crate) fn format_export(export: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = export.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, export, ctx);
    }
    if let Some(keyword) = export.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("export"));
        trivias = format_trivias_after_token(keyword, export, ctx);
    }
    if let Some(name) = export.children_by_kind(NAME).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_name(name));
        trivias = format_trivias_after_node(name, export, ctx);
    }
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(export))
        .group()
}

pub(crate) fn format_import(import: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = import.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, import, ctx);
    }
    if let Some(keyword) = import.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("import"));
        trivias = format_trivias_after_token(keyword, import, ctx);
    }
    if let Some(module_name) = import.children_by_kind(MODULE_NAME).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_module_name(module_name));
        trivias = format_trivias_after_node(module_name, import, ctx);
    }
    if let Some(name) = import.children_by_kind(NAME).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_name(name));
        trivias = format_trivias_after_node(name, import, ctx);
    }
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(import))
        .group()
}

pub(crate) fn format_index(index: AmberNode) -> Doc<'static> {
    if let Some(token) = index.children_with_tokens().next().and_then(NodeOrToken::into_token) {
        Doc::text(token.text().to_string())
    } else {
        Doc::nil()
    }
}

pub(crate) fn format_local(local: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = local.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, local, ctx);
    }
    if let Some(keyword) = local.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("local"));
        trivias = format_trivias_after_token(keyword, local, ctx);
    }
    if let Some(ident) = local.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text().to_string()));
        trivias = format_trivias_after_token(ident, local, ctx);
    }
    local.children_by_kind(ValType::can_cast).for_each(|val_type| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_val_type(val_type, ctx));
        trivias = format_trivias_after_node(val_type, local, ctx);
    });
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(local))
        .group()
}

pub(crate) fn format_mem_use(mem_use: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = mem_use.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, mem_use, ctx);
    }
    if let Some(keyword) = mem_use.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("memory"));
        trivias = format_trivias_after_token(keyword, mem_use, ctx);
    }
    if let Some(index) = mem_use.children_by_kind(INDEX).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_index(index));
        trivias = format_trivias_after_node(index, mem_use, ctx);
    }
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(mem_use))
        .group()
}

impl DocGen for Module {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        let mut is_explicit_module = true;
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren.amber(), self.syntax().amber(), ctx);
        } else {
            is_explicit_module = false;
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("module"));
            trivias = format_trivias_after_token(keyword.amber(), self.syntax().amber(), ctx);
        }
        if let Some(ident) = self.ident_token() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(Doc::text(ident.to_string()));
            trivias = format_trivias_after_token(ident.amber(), self.syntax().amber(), ctx);
        }
        self.module_fields().for_each(|module_field| {
            if trivias.is_empty() && (!docs.is_empty() || !docs.is_empty()) {
                docs.push(Doc::hard_line());
            } else {
                docs.append(&mut trivias);
            }
            let node = module_field.syntax();
            if should_ignore(node, ctx) {
                reflow(&node.to_string(), &mut docs);
            } else {
                docs.push(module_field.doc(ctx));
            }
            trivias = format_trivias_after_node(module_field.syntax().amber(), self.syntax().amber(), ctx);
        });
        docs.append(&mut trivias);
        if is_explicit_module {
            Doc::list(docs)
                .nest(ctx.indent_width)
                .append(ctx.format_right_paren(self.syntax().amber()))
                .group()
        } else {
            Doc::list(docs)
        }
    }
}

impl DocGen for ModuleField {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        match self {
            ModuleField::Data(module_field_data) => format_module_field_data(module_field_data.syntax().amber(), ctx),
            ModuleField::Elem(module_field_elem) => format_module_field_elem(module_field_elem.syntax().amber(), ctx),
            ModuleField::Export(module_field_export) => {
                format_module_field_export(module_field_export.syntax().amber(), ctx)
            }
            ModuleField::Func(module_field_func) => format_module_field_func(module_field_func.syntax().amber(), ctx),
            ModuleField::Global(module_field_global) => {
                format_module_field_global(module_field_global.syntax().amber(), ctx)
            }
            ModuleField::Import(module_field_import) => {
                format_module_field_import(module_field_import.syntax().amber(), ctx)
            }
            ModuleField::Memory(module_field_memory) => {
                format_module_field_memory(module_field_memory.syntax().amber(), ctx)
            }
            ModuleField::Start(module_field_start) => {
                format_module_field_start(module_field_start.syntax().amber(), ctx)
            }
            ModuleField::Table(module_field_table) => {
                format_module_field_table(module_field_table.syntax().amber(), ctx)
            }
            ModuleField::Tag(module_field_tag) => format_module_field_tag(module_field_tag.syntax().amber(), ctx),
            ModuleField::Type(type_def) => format_type_def(type_def.syntax().amber(), ctx),
            ModuleField::RecType(rec_type) => format_rec_type(rec_type.syntax().amber(), ctx),
        }
    }
}

pub(crate) fn format_module_field_data(module_field_data: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = module_field_data.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, module_field_data, ctx);
    }
    if let Some(keyword) = module_field_data.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("data"));
        trivias = format_trivias_after_token(keyword, module_field_data, ctx);
    }
    if let Some(ident) = module_field_data.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text().to_string()));
        trivias = format_trivias_after_token(ident, module_field_data, ctx);
    }
    if let Some(mem_use) = module_field_data.children_by_kind(MEM_USE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_mem_use(mem_use, ctx));
        trivias = format_trivias_after_node(mem_use, module_field_data, ctx);
    }
    if let Some(offset) = module_field_data.children_by_kind(OFFSET).next() {
        let has_keyword = offset.tokens_by_kind(KEYWORD).next().is_some();
        if trivias.is_empty() && has_keyword {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_offset(offset, ctx));
        trivias = format_trivias_after_node(offset, module_field_data, ctx);
    }
    module_field_data.tokens_by_kind(STRING).for_each(|string| {
        if trivias.is_empty() {
            docs.push(Doc::soft_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(string.text().to_string()));
        trivias = format_trivias_after_token(string, module_field_data, ctx);
    });
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(module_field_data))
        .group()
}

pub(crate) fn format_module_field_elem(module_field_elem: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = module_field_elem.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, module_field_elem, ctx);
    }
    if let Some(keyword) = module_field_elem.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("elem"));
        trivias = format_trivias_after_token(keyword, module_field_elem, ctx);
    }
    if let Some(ident) = module_field_elem.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text().to_string()));
        trivias = format_trivias_after_token(ident, module_field_elem, ctx);
    }
    if let Some(keyword) = module_field_elem
        .tokens_by_kind(KEYWORD)
        .find(|token| token.text() == "declare")
    {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text("declare"));
        trivias = format_trivias_after_token(keyword, module_field_elem, ctx);
    }
    if let Some(table_use) = module_field_elem.children_by_kind(TABLE_USE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_table_use(table_use, ctx));
        trivias = format_trivias_after_node(table_use, module_field_elem, ctx);
    }
    if let Some(offset) = module_field_elem.children_by_kind(OFFSET).next() {
        let has_keyword = offset.tokens_by_kind(KEYWORD).next().is_some();
        if trivias.is_empty() && has_keyword {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_offset(offset, ctx));
        trivias = format_trivias_after_node(offset, module_field_elem, ctx);
    }
    if let Some(elem_list) = module_field_elem.children_by_kind(ELEM_LIST).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_elem_list(elem_list, ctx));
        trivias = format_trivias_after_node(elem_list, module_field_elem, ctx);
    }
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(module_field_elem))
        .group()
}

pub(crate) fn format_module_field_export(module_field_export: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = module_field_export.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, module_field_export, ctx);
    }
    if let Some(keyword) = module_field_export.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("export"));
        trivias = format_trivias_after_token(keyword, module_field_export, ctx);
    }
    if let Some(name) = module_field_export.children_by_kind(NAME).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_name(name));
        trivias = format_trivias_after_node(name, module_field_export, ctx);
    }
    if let Some(extern_idx) = module_field_export.children_by_kind(ExternIdx::can_cast).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_extern_idx(extern_idx, ctx));
        trivias = format_trivias_after_node(extern_idx, module_field_export, ctx);
    }
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(module_field_export))
        .group()
}

pub(crate) fn format_module_field_func(module_field_func: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = module_field_func.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, module_field_func, ctx);
    }
    if let Some(keyword) = module_field_func.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("func"));
        trivias = format_trivias_after_token(keyword, module_field_func, ctx);
    }
    if let Some(ident) = module_field_func.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text().to_string()));
        trivias = format_trivias_after_token(ident, module_field_func, ctx);
    }
    module_field_func.children_by_kind(EXPORT).for_each(|export| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_export(export, ctx));
        trivias = format_trivias_after_node(export, module_field_func, ctx);
    });
    if let Some(import) = module_field_func.children_by_kind(IMPORT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_import(import, ctx));
        trivias = format_trivias_after_node(import, module_field_func, ctx);
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
        trivias = format_trivias_after_node(type_use, module_field_func, ctx);
    }
    let mut locals = module_field_func.children_by_kind(LOCAL).peekable();
    let mut locals_docs = Vec::with_capacity(1);
    let ws_of_multi_line =
        whitespace_of_multi_line(ctx.options.multi_line_locals, locals.peek().copied(), module_field_func);
    if let Some(local) = locals.next() {
        if trivias.is_empty() {
            docs.push(wrap_before(&mut locals, ctx.options.wrap_before_locals));
        } else {
            docs.append(&mut trivias);
        }
        locals_docs.push(format_local(local, ctx));
        trivias = format_trivias_after_node(local, module_field_func, ctx);
    }
    locals.for_each(|local| {
        if trivias.is_empty() {
            locals_docs.push(ws_of_multi_line.clone());
        } else {
            locals_docs.append(&mut trivias);
        }
        locals_docs.push(format_local(local, ctx));
        trivias = format_trivias_after_node(local, module_field_func, ctx);
    });
    docs.push(Doc::list(locals_docs).group());
    module_field_func.children_by_kind(Instr::can_cast).for_each(|instr| {
        if trivias.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_instr(instr, ctx));
        trivias = format_trivias_after_node(instr, module_field_func, ctx);
    });
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(module_field_func))
        .group()
}

pub(crate) fn format_module_field_global(module_field_global: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = module_field_global.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, module_field_global, ctx);
    }
    if let Some(keyword) = module_field_global.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("global"));
        trivias = format_trivias_after_token(keyword, module_field_global, ctx);
    }
    if let Some(ident) = module_field_global.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text().to_string()));
        trivias = format_trivias_after_token(ident, module_field_global, ctx);
    }
    module_field_global.children_by_kind(EXPORT).for_each(|export| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_export(export, ctx));
        trivias = format_trivias_after_node(export, module_field_global, ctx);
    });
    if let Some(import) = module_field_global.children_by_kind(IMPORT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_import(import, ctx));
        trivias = format_trivias_after_node(import, module_field_global, ctx);
    }
    if let Some(global_type) = module_field_global.children_by_kind(GLOBAL_TYPE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_global_type(global_type, ctx));
        trivias = format_trivias_after_node(global_type, module_field_global, ctx);
    }
    format_const_expr(module_field_global, ctx, &mut docs, &mut trivias);
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(module_field_global))
        .group()
}

pub(crate) fn format_module_field_import(module_field_import: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = module_field_import.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, module_field_import, ctx);
    }
    if let Some(keyword) = module_field_import.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("import"));
        trivias = format_trivias_after_token(keyword, module_field_import, ctx);
    }
    if let Some(module_name) = module_field_import.children_by_kind(MODULE_NAME).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_module_name(module_name));
        trivias = format_trivias_after_node(module_name, module_field_import, ctx);
    }
    if let Some(name) = module_field_import.children_by_kind(NAME).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_name(name));
        trivias = format_trivias_after_node(name, module_field_import, ctx);
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
        trivias = format_trivias_after_node(extern_type, module_field_import, ctx);
    }
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(module_field_import))
        .group()
}

pub(crate) fn format_module_field_memory(module_field_memory: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = module_field_memory.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, module_field_memory, ctx);
    }
    if let Some(keyword) = module_field_memory.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("memory"));
        trivias = format_trivias_after_token(keyword, module_field_memory, ctx);
    }
    if let Some(ident) = module_field_memory.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text().to_string()));
        trivias = format_trivias_after_token(ident, module_field_memory, ctx);
    }
    module_field_memory.children_by_kind(EXPORT).for_each(|export| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_export(export, ctx));
        trivias = format_trivias_after_node(export, module_field_memory, ctx);
    });
    if let Some(import) = module_field_memory.children_by_kind(IMPORT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_import(import, ctx));
        trivias = format_trivias_after_node(import, module_field_memory, ctx);
    }
    if let Some(mem_type) = module_field_memory.children_by_kind(MEM_TYPE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_mem_type(mem_type, ctx));
        trivias = format_trivias_after_node(mem_type, module_field_memory, ctx);
    }
    if let Some(data) = module_field_memory.children_by_kind(DATA).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_data(data, ctx));
        trivias = format_trivias_after_node(data, module_field_memory, ctx);
    }
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(module_field_memory))
        .group()
}

pub(crate) fn format_module_field_start(module_field_start: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = module_field_start.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, module_field_start, ctx);
    }
    if let Some(keyword) = module_field_start.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("start"));
        trivias = format_trivias_after_token(keyword, module_field_start, ctx);
    }
    if let Some(index) = module_field_start.children_by_kind(INDEX).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_index(index));
        trivias = format_trivias_after_node(index, module_field_start, ctx);
    }
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(module_field_start))
        .group()
}

pub(crate) fn format_module_field_table(module_field_table: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = module_field_table.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, module_field_table, ctx);
    }
    if let Some(keyword) = module_field_table.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("table"));
        trivias = format_trivias_after_token(keyword, module_field_table, ctx);
    }
    if let Some(ident) = module_field_table.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text().to_string()));
        trivias = format_trivias_after_token(ident, module_field_table, ctx);
    }
    module_field_table.children_by_kind(EXPORT).for_each(|export| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_export(export, ctx));
        trivias = format_trivias_after_node(export, module_field_table, ctx);
    });
    if let Some(import) = module_field_table.children_by_kind(IMPORT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_import(import, ctx));
        trivias = format_trivias_after_node(import, module_field_table, ctx);
    }
    if let Some(table_type) = module_field_table.children_by_kind(TABLE_TYPE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_table_type(table_type, ctx));
        trivias = format_trivias_after_node(table_type, module_field_table, ctx);
    }
    if let Some(ref_type) = module_field_table.children_by_kind(REF_TYPE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_ref_type(ref_type, ctx));
        trivias = format_trivias_after_node(ref_type, module_field_table, ctx);
    }
    if let Some(elem) = module_field_table.children_by_kind(ELEM).next() {
        if trivias.is_empty() {
            docs.push(Doc::line_or_space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_elem(elem, ctx));
        trivias = format_trivias_after_node(elem, module_field_table, ctx);
    }
    format_const_expr(module_field_table, ctx, &mut docs, &mut trivias);
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(module_field_table))
        .group()
}

pub(crate) fn format_module_field_tag(module_field_tag: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = module_field_tag.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, module_field_tag, ctx);
    }
    if let Some(keyword) = module_field_tag.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("tag"));
        trivias = format_trivias_after_token(keyword, module_field_tag, ctx);
    }
    if let Some(ident) = module_field_tag.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text().to_string()));
        trivias = format_trivias_after_token(ident, module_field_tag, ctx);
    }
    module_field_tag.children_by_kind(EXPORT).for_each(|export| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_export(export, ctx));
        trivias = format_trivias_after_node(export, module_field_tag, ctx);
    });
    if let Some(import) = module_field_tag.children_by_kind(IMPORT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_import(import, ctx));
        trivias = format_trivias_after_node(import, module_field_tag, ctx);
    }
    if let Some(type_use) = module_field_tag.children_by_kind(TYPE_USE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_type_use(type_use, ctx));
        trivias = format_trivias_after_node(type_use, module_field_tag, ctx);
    }
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(module_field_tag))
        .group()
}

pub(crate) fn format_module_name(module_name: AmberNode) -> Doc<'static> {
    if let Some(string) = module_name.tokens_by_kind(STRING).next() {
        Doc::text(string.text().to_string())
    } else {
        Doc::nil()
    }
}

pub(crate) fn format_name(name: AmberNode) -> Doc<'static> {
    if let Some(string) = name.tokens_by_kind(STRING).next() {
        Doc::text(string.text().to_string())
    } else {
        Doc::nil()
    }
}

pub(crate) fn format_offset(offset: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = offset.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, offset, ctx);
    }
    if let Some(keyword) = offset.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("offset"));
        trivias = format_trivias_after_token(keyword, offset, ctx);
    }
    format_const_expr(offset, ctx, &mut docs, &mut trivias);
    if offset.tokens_by_kind(R_PAREN).next().is_some() {
        docs.append(&mut trivias);
        Doc::list(docs)
            .nest(ctx.indent_width)
            .append(ctx.format_right_paren(offset))
            .group()
    } else {
        Doc::list(docs)
    }
}

pub(crate) fn format_rec_type(rec_type: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = rec_type.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, rec_type, ctx);
    }
    if let Some(keyword) = rec_type.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("rec"));
        trivias = format_trivias_after_token(keyword, rec_type, ctx);
    }
    rec_type.children_by_kind(TYPE_DEF).for_each(|type_def| {
        if trivias.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_type_def(type_def, ctx));
        trivias = format_trivias_after_node(type_def, rec_type, ctx);
    });
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(rec_type))
        .group()
}

pub(crate) fn format_table_use(table_use: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = table_use.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, table_use, ctx);
    }
    if let Some(keyword) = table_use.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("table"));
        trivias = format_trivias_after_token(keyword, table_use, ctx);
    }
    if let Some(index) = table_use.children_by_kind(INDEX).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_index(index));
        trivias = format_trivias_after_node(index, table_use, ctx);
    }
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(table_use))
        .group()
}

pub(crate) fn format_type_def(type_def: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = type_def.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, type_def, ctx);
    }
    if let Some(keyword) = type_def.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("type"));
        trivias = format_trivias_after_token(keyword, type_def, ctx);
    }
    if let Some(ident) = type_def.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text().to_string()));
        trivias = format_trivias_after_token(ident, type_def, ctx);
    }
    if let Some(sub_type) = type_def.children_by_kind(SUB_TYPE).next() {
        if trivias.is_empty() {
            docs.push(Doc::line_or_space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_sub_type(sub_type, ctx));
        trivias = format_trivias_after_node(sub_type, type_def, ctx);
    }
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(type_def))
        .group()
}

pub(crate) fn format_type_use(type_use: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = type_use.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, type_use, ctx);
    }
    if let Some(keyword) = type_use.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("type"));
        trivias = format_trivias_after_token(keyword, type_use, ctx);
    }
    if let Some(index) = type_use.children_by_kind(INDEX).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_index(index));
        trivias = format_trivias_after_node(index, type_use, ctx);
    }
    if let Some(r_paren) = type_use.tokens_by_kind(R_PAREN).next() {
        docs.append(&mut trivias);
        docs.push(ctx.format_right_paren(type_use).group());
        trivias = format_trivias_after_token(r_paren, type_use, ctx);
    }

    let mut params = type_use.children_by_kind(PARAM);
    if let Some(param) = params.next() {
        if trivias.is_empty() && !docs.is_empty() {
            docs.push(Doc::soft_line());
        } else if type_use.tokens_by_kind(L_PAREN).next().is_some() {
            docs.append(&mut trivias);
        }
        docs.push(format_param(param, ctx));
        trivias = format_trivias_after_node(param, type_use, ctx);
    }
    params.for_each(|param| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_param(param, ctx));
        trivias = format_trivias_after_node(param, type_use, ctx);
    });
    let mut results = type_use.children_by_kind(RESULT);
    if let Some(result) = results.next() {
        if trivias.is_empty() && !docs.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_result(result, ctx));
        trivias = format_trivias_after_node(result, type_use, ctx);
    }
    results.for_each(|result| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_result(result, ctx));
        trivias = format_trivias_after_node(result, type_use, ctx);
    });

    Doc::list(docs)
}

pub(crate) fn format_extern_idx(extern_idx: AmberNode, ctx: &Ctx) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = extern_idx.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, extern_idx, ctx);
    }
    if let Some(keyword) = extern_idx.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text(keyword.text().to_string()));
        trivias = format_trivias_after_token(keyword, extern_idx, ctx);
    }
    if let Some(index) = extern_idx.children_by_kind(INDEX).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_index(index));
        trivias = format_trivias_after_node(index, extern_idx, ctx);
    }
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(extern_idx))
        .group()
}

fn format_const_expr(parent: AmberNode, ctx: &Ctx, docs: &mut Vec<Doc<'static>>, trivias: &mut Vec<Doc<'static>>) {
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
        *trivias = format_trivias_after_node(instr, parent, ctx);
    }
    instrs.for_each(|instr| {
        if trivias.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(trivias);
        }
        docs.push(format_instr(instr, ctx));
        *trivias = format_trivias_after_node(instr, parent, ctx);
    });
}
