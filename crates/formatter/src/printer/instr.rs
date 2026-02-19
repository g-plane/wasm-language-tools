use super::*;
use std::mem;
use tiny_pretty::Doc;
use wat_syntax::{NodeOrToken, SyntaxKind::*, ast::AstNode};

pub(crate) fn format_block_block<'a>(block_block: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = block_block.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, block_block, ctx);
    }
    if let Some(keyword) = block_block.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("block"));
        trivias = format_trivias_after_token(keyword, block_block, ctx);
    }
    if let Some(ident) = block_block.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        trivias = format_trivias_after_token(ident, block_block, ctx);
    }
    if let Some(type_use) = block_block.children_by_kind(TYPE_USE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_type_use(type_use, ctx));
        trivias = format_trivias_after_node(type_use, block_block, ctx);
    }
    block_block.children_by_kind(Instr::can_cast).for_each(|instr| {
        if trivias.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_instr(instr, ctx));
        trivias = format_trivias_after_node(instr, block_block, ctx);
    });
    docs.append(&mut trivias);
    let mut docs = vec![Doc::list(docs).nest(ctx.indent_width)];
    if block_block.tokens_by_kind(R_PAREN).next().is_some() {
        docs.push(ctx.format_right_paren(block_block));
    } else {
        if let Some(keyword) = block_block.tokens_by_kind(KEYWORD).find(|token| token.text() == "end") {
            docs.push(Doc::hard_line());
            docs.push(Doc::text("end"));
            trivias = format_trivias_after_token(keyword, block_block, ctx);
        }
        if let Some(ident) = block_block.tokens_by_kind(IDENT).nth(1) {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(Doc::text(ident.text()));
        }
    }
    Doc::list(docs).group()
}

pub(crate) fn format_block_if<'a>(block_if: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = block_if.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, block_if, ctx);
    }
    if let Some(keyword) = block_if.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("if"));
        trivias = format_trivias_after_token(keyword, block_if, ctx);
    }
    if let Some(ident) = block_if.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        trivias = format_trivias_after_token(ident, block_if, ctx);
    }
    if let Some(type_use) = block_if.children_by_kind(TYPE_USE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_type_use(type_use, ctx));
        trivias = format_trivias_after_node(type_use, block_if, ctx);
    }
    block_if.children_by_kind(Instr::can_cast).for_each(|instr| {
        if trivias.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_instr(instr, ctx));
        trivias = format_trivias_after_node(instr, block_if, ctx);
    });
    if let Some(then_block) = block_if.children_by_kind(BLOCK_IF_THEN).next() {
        if trivias.is_empty() && then_block.tokens_by_kind(L_PAREN).next().is_some() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_block_if_then(then_block, ctx));
        trivias = format_trivias_after_node(then_block, block_if, ctx);
    }
    if let Some(else_block) = block_if.children_by_kind(BLOCK_IF_ELSE).next() {
        if trivias.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_block_if_else(else_block, ctx));
        trivias = format_trivias_after_node(else_block, block_if, ctx);
    }
    docs.push(Doc::list(mem::take(&mut trivias)).nest(ctx.indent_width));
    if block_if.tokens_by_kind(R_PAREN).next().is_some() {
        Doc::list(docs)
            .nest(ctx.indent_width)
            .append(ctx.format_right_paren(block_if))
            .group()
    } else {
        if let Some(keyword) = block_if.tokens_by_kind(KEYWORD).find(|token| token.text() == "end") {
            docs.push(Doc::hard_line());
            docs.push(Doc::text("end"));
            trivias = format_trivias_after_token(keyword, block_if, ctx);
        }
        if let Some(ident) = block_if.tokens_by_kind(IDENT).nth(1) {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(Doc::text(ident.text()));
        }
        Doc::list(docs)
    }
}

pub(crate) fn format_block_if_else<'a>(block_if_else: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = block_if_else.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, block_if_else, ctx);
    }
    if let Some(keyword) = block_if_else.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("else"));
        trivias = format_trivias_after_token(keyword, block_if_else, ctx);
    }
    if let Some(ident) = block_if_else.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        trivias = format_trivias_after_token(ident, block_if_else, ctx);
    }
    block_if_else.children_by_kind(Instr::can_cast).for_each(|instr| {
        if trivias.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_instr(instr, ctx));
        trivias = format_trivias_after_node(instr, block_if_else, ctx);
    });
    docs.append(&mut trivias);
    let doc = Doc::list(docs).nest(ctx.indent_width);
    if block_if_else.tokens_by_kind(R_PAREN).next().is_some() {
        doc.append(ctx.format_right_paren(block_if_else)).group()
    } else {
        doc
    }
}

pub(crate) fn format_block_if_then<'a>(block_if_then: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = block_if_then.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, block_if_then, ctx);
    }
    if let Some(keyword) = block_if_then.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("then"));
        trivias = format_trivias_after_token(keyword, block_if_then, ctx);
    }
    if let Some(ident) = block_if_then.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        trivias = format_trivias_after_token(ident, block_if_then, ctx);
    }
    block_if_then.children_by_kind(Instr::can_cast).for_each(|instr| {
        if trivias.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_instr(instr, ctx));
        trivias = format_trivias_after_node(instr, block_if_then, ctx);
    });
    docs.append(&mut trivias);
    let doc = Doc::list(docs).nest(ctx.indent_width);
    if block_if_then.tokens_by_kind(R_PAREN).next().is_some() {
        doc.append(ctx.format_right_paren(block_if_then)).group()
    } else {
        doc
    }
}

pub(crate) fn format_block_loop<'a>(block_loop: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = block_loop.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, block_loop, ctx);
    }
    if let Some(keyword) = block_loop.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("loop"));
        trivias = format_trivias_after_token(keyword, block_loop, ctx);
    }
    if let Some(ident) = block_loop.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        trivias = format_trivias_after_token(ident, block_loop, ctx);
    }
    if let Some(type_use) = block_loop.children_by_kind(TYPE_USE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_type_use(type_use, ctx));
        trivias = format_trivias_after_node(type_use, block_loop, ctx);
    }
    block_loop.children_by_kind(Instr::can_cast).for_each(|instr| {
        if trivias.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_instr(instr, ctx));
        trivias = format_trivias_after_node(instr, block_loop, ctx);
    });
    docs.append(&mut trivias);
    let mut docs = vec![Doc::list(docs).nest(ctx.indent_width)];
    if block_loop.tokens_by_kind(R_PAREN).next().is_some() {
        docs.push(ctx.format_right_paren(block_loop));
    } else {
        if let Some(keyword) = block_loop.tokens_by_kind(KEYWORD).find(|token| token.text() == "end") {
            docs.push(Doc::hard_line());
            docs.push(Doc::text("end"));
            trivias = format_trivias_after_token(keyword, block_loop, ctx);
        }
        if let Some(ident) = block_loop.tokens_by_kind(IDENT).nth(1) {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(Doc::text(ident.text()));
        }
    }
    Doc::list(docs).group()
}

pub(crate) fn format_block_try_table<'a>(block_try_table: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = block_try_table.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, block_try_table, ctx);
    }
    if let Some(keyword) = block_try_table.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("try_table"));
        trivias = format_trivias_after_token(keyword, block_try_table, ctx);
    }
    if let Some(ident) = block_try_table.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        trivias = format_trivias_after_token(ident, block_try_table, ctx);
    }
    if let Some(type_use) = block_try_table.children_by_kind(TYPE_USE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_type_use(type_use, ctx));
        trivias = format_trivias_after_node(type_use, block_try_table, ctx);
    }
    block_try_table.children_by_kind(Cat::can_cast).for_each(|cat| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        match cat.kind() {
            CATCH => docs.push(format_catch(cat, ctx)),
            CATCH_ALL => docs.push(format_catch_all(cat, ctx)),
            _ => {}
        }
        trivias = format_trivias_after_node(cat, block_try_table, ctx);
    });
    block_try_table.children_by_kind(Instr::can_cast).for_each(|instr| {
        if trivias.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_instr(instr, ctx));
        trivias = format_trivias_after_node(instr, block_try_table, ctx);
    });
    docs.append(&mut trivias);
    let mut docs = vec![Doc::list(docs).nest(ctx.indent_width)];
    if block_try_table.tokens_by_kind(R_PAREN).next().is_some() {
        docs.push(ctx.format_right_paren(block_try_table));
    } else {
        if let Some(keyword) = block_try_table
            .tokens_by_kind(KEYWORD)
            .find(|token| token.text() == "end")
        {
            docs.push(Doc::hard_line());
            docs.push(Doc::text("end"));
            trivias = format_trivias_after_token(keyword, block_try_table, ctx);
        }
        if let Some(ident) = block_try_table.tokens_by_kind(IDENT).nth(1) {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(Doc::text(ident.text()));
        }
    }
    Doc::list(docs).group()
}

pub(crate) fn format_catch<'a>(catch: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = catch.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, catch, ctx);
    }
    if let Some(keyword) = catch.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text(keyword.text()));
        trivias = format_trivias_after_token(keyword, catch, ctx);
    }
    let mut indexes = catch.children_by_kind(INDEX);
    if let Some(tag_index) = indexes.next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_index(tag_index));
        trivias = format_trivias_after_node(tag_index, catch, ctx);
    }
    if let Some(label_index) = indexes.next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_index(label_index));
        trivias = format_trivias_after_node(label_index, catch, ctx);
    }
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(catch))
        .group()
}

pub(crate) fn format_catch_all<'a>(catch_all: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = catch_all.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, catch_all, ctx);
    }
    if let Some(keyword) = catch_all.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text(keyword.text()));
        trivias = format_trivias_after_token(keyword, catch_all, ctx);
    }
    if let Some(label_index) = catch_all.children_by_kind(INDEX).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_index(label_index));
        trivias = format_trivias_after_node(label_index, catch_all, ctx);
    }
    docs.append(&mut trivias);
    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(ctx.format_right_paren(catch_all))
        .group()
}

pub(crate) fn format_immediate<'a>(immediate: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    match immediate.children_with_tokens().next() {
        Some(NodeOrToken::Node(node)) => match node.kind() {
            TYPE_USE => format_type_use(node, ctx),
            MEM_ARG => format_mem_arg(node),
            HEAP_TYPE => format_heap_type(node),
            REF_TYPE => format_ref_type(node, ctx),
            _ => Doc::nil(),
        },
        Some(NodeOrToken::Token(token)) => Doc::text(token.text()),
        None => Doc::nil(),
    }
}

pub(crate) fn format_instr<'a>(instr: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    match instr.kind() {
        PLAIN_INSTR => format_plain_instr(instr, ctx),
        BLOCK_BLOCK => format_block_block(instr, ctx),
        BLOCK_LOOP => format_block_loop(instr, ctx),
        BLOCK_IF => format_block_if(instr, ctx),
        BLOCK_TRY_TABLE => format_block_try_table(instr, ctx),
        _ => Doc::nil(),
    }
}

pub(crate) fn format_mem_arg<'a>(mem_arg: AmberNode<'a>) -> Doc<'a> {
    let mut docs = Vec::with_capacity(3);
    if let Some(keyword) = mem_arg.tokens_by_kind(MEM_ARG_KEYWORD).next() {
        docs.push(Doc::text(keyword.text()));
    }
    docs.push(Doc::text("="));
    if let Some(unsigned_int) = mem_arg.tokens_by_kind(UNSIGNED_INT).next() {
        docs.push(Doc::text(unsigned_int.text()));
    }
    Doc::list(docs)
}

pub(crate) fn format_plain_instr<'a>(plain_instr: AmberNode<'a>, ctx: &Ctx) -> Doc<'a> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = plain_instr.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, plain_instr, ctx);
    }
    if let Some(instr_name) = plain_instr.tokens_by_kind(INSTR_NAME).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text(instr_name.text()));
        trivias = format_trivias_after_token(instr_name, plain_instr, ctx);
    }
    plain_instr.children_by_kind(IMMEDIATE).for_each(|immediate| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_immediate(immediate, ctx));
        trivias = format_trivias_after_node(immediate, plain_instr, ctx);
    });
    plain_instr.children_by_kind(Instr::can_cast).for_each(|instr| {
        if trivias.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_instr(instr, ctx));
        trivias = format_trivias_after_node(instr, plain_instr, ctx);
    });
    docs.append(&mut trivias);
    let doc = Doc::list(docs).nest(ctx.indent_width);
    if plain_instr.tokens_by_kind(R_PAREN).next().is_some() {
        doc.append(ctx.format_right_paren(plain_instr)).group()
    } else {
        doc
    }
}
