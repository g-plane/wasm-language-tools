use super::*;
use bumpalo::collections::Vec as BumpVec;
use tiny_pretty::Doc;
use wat_syntax::{NodeOrToken, SyntaxKind::*, ast::AstNode};

pub(crate) fn format_block_block<'a>(block_block: AmberNode<'a>, ctx: &'a Ctx<'a>) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = block_block.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::char('('));
        ctx.format_trivias_after_token(l_paren, block_block, &mut trivias);
    }
    if let Some(keyword) = block_block.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("block"));
        ctx.format_trivias_after_token(keyword, block_block, &mut trivias);
    }
    if let Some(ident) = block_block.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        ctx.format_trivias_after_token(ident, block_block, &mut trivias);
    }
    if let Some(type_use) = block_block.children_by_kind(TYPE_USE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_type_use(type_use, ctx));
        ctx.format_trivias_after_node(type_use, block_block, &mut trivias);
    }
    block_block.children_by_kind(Instr::can_cast).for_each(|instr| {
        if trivias.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_instr(instr, ctx));
        ctx.format_trivias_after_node(instr, block_block, &mut trivias);
    });
    docs.append(&mut trivias);
    let mut docs = BumpVec::from_iter_in([Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width)], &ctx.bump);
    if block_block.tokens_by_kind(R_PAREN).next().is_some() {
        docs.push(ctx.format_right_paren_after_instr(block_block));
    } else {
        if let Some(keyword) = block_block.tokens_by_kind(KEYWORD).find(|token| token.text() == "end") {
            docs.push(Doc::hard_line());
            docs.push(Doc::text("end"));
            ctx.format_trivias_after_token(keyword, block_block, &mut trivias);
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
    Doc::slice(docs.into_bump_slice())
}

pub(crate) fn format_block_if<'a>(block_if: AmberNode<'a>, ctx: &'a Ctx<'a>) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = block_if.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::char('('));
        ctx.format_trivias_after_token(l_paren, block_if, &mut trivias);
    }
    if let Some(keyword) = block_if.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("if"));
        ctx.format_trivias_after_token(keyword, block_if, &mut trivias);
    }
    if let Some(ident) = block_if.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        ctx.format_trivias_after_token(ident, block_if, &mut trivias);
    }
    if let Some(type_use) = block_if.children_by_kind(TYPE_USE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_type_use(type_use, ctx));
        ctx.format_trivias_after_node(type_use, block_if, &mut trivias);
    }
    block_if.children_by_kind(Instr::can_cast).for_each(|instr| {
        if trivias.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_instr(instr, ctx));
        ctx.format_trivias_after_node(instr, block_if, &mut trivias);
    });
    if let Some(then_block) = block_if.children_by_kind(BLOCK_IF_THEN).next() {
        if trivias.is_empty() && then_block.tokens_by_kind(L_PAREN).next().is_some() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_block_if_then(then_block, ctx));
        ctx.format_trivias_after_node(then_block, block_if, &mut trivias);
    }
    if let Some(else_block) = block_if.children_by_kind(BLOCK_IF_ELSE).next() {
        if trivias.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_block_if_else(else_block, ctx));
        ctx.format_trivias_after_node(else_block, block_if, &mut trivias);
    }
    docs.push(Doc::slice(trivias.into_bump_slice()).nest(ctx.indent_width));
    trivias = BumpVec::new_in(&ctx.bump);
    if block_if.tokens_by_kind(R_PAREN).next().is_some() {
        Doc::slice(ctx.bump.alloc_slice_fill_iter([
            Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
            ctx.format_right_paren(block_if),
        ]))
        .group()
    } else {
        if let Some(keyword) = block_if.tokens_by_kind(KEYWORD).find(|token| token.text() == "end") {
            docs.push(Doc::hard_line());
            docs.push(Doc::text("end"));
            ctx.format_trivias_after_token(keyword, block_if, &mut trivias);
        }
        if let Some(ident) = block_if.tokens_by_kind(IDENT).nth(1) {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(Doc::text(ident.text()));
        }
        Doc::slice(docs.into_bump_slice())
    }
}

pub(crate) fn format_block_if_else<'a>(block_if_else: AmberNode<'a>, ctx: &'a Ctx<'a>) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = block_if_else.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::char('('));
        ctx.format_trivias_after_token(l_paren, block_if_else, &mut trivias);
    }
    if let Some(keyword) = block_if_else.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("else"));
        ctx.format_trivias_after_token(keyword, block_if_else, &mut trivias);
    }
    if let Some(ident) = block_if_else.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        ctx.format_trivias_after_token(ident, block_if_else, &mut trivias);
    }
    block_if_else.children_by_kind(Instr::can_cast).for_each(|instr| {
        if trivias.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_instr(instr, ctx));
        ctx.format_trivias_after_node(instr, block_if_else, &mut trivias);
    });
    docs.append(&mut trivias);
    let doc = Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width);
    if block_if_else.tokens_by_kind(R_PAREN).next().is_some() {
        Doc::slice(
            ctx.bump
                .alloc_slice_fill_iter([doc, ctx.format_right_paren_after_instr(block_if_else)]),
        )
    } else {
        doc
    }
}

pub(crate) fn format_block_if_then<'a>(block_if_then: AmberNode<'a>, ctx: &'a Ctx<'a>) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = block_if_then.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::char('('));
        ctx.format_trivias_after_token(l_paren, block_if_then, &mut trivias);
    }
    if let Some(keyword) = block_if_then.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("then"));
        ctx.format_trivias_after_token(keyword, block_if_then, &mut trivias);
    }
    if let Some(ident) = block_if_then.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        ctx.format_trivias_after_token(ident, block_if_then, &mut trivias);
    }
    block_if_then.children_by_kind(Instr::can_cast).for_each(|instr| {
        if trivias.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_instr(instr, ctx));
        ctx.format_trivias_after_node(instr, block_if_then, &mut trivias);
    });
    docs.append(&mut trivias);
    let doc = Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width);
    if block_if_then.tokens_by_kind(R_PAREN).next().is_some() {
        Doc::slice(
            ctx.bump
                .alloc_slice_fill_iter([doc, ctx.format_right_paren_after_instr(block_if_then)]),
        )
    } else {
        doc
    }
}

pub(crate) fn format_block_loop<'a>(block_loop: AmberNode<'a>, ctx: &'a Ctx<'a>) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = block_loop.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::char('('));
        ctx.format_trivias_after_token(l_paren, block_loop, &mut trivias);
    }
    if let Some(keyword) = block_loop.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("loop"));
        ctx.format_trivias_after_token(keyword, block_loop, &mut trivias);
    }
    if let Some(ident) = block_loop.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        ctx.format_trivias_after_token(ident, block_loop, &mut trivias);
    }
    if let Some(type_use) = block_loop.children_by_kind(TYPE_USE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_type_use(type_use, ctx));
        ctx.format_trivias_after_node(type_use, block_loop, &mut trivias);
    }
    block_loop.children_by_kind(Instr::can_cast).for_each(|instr| {
        if trivias.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_instr(instr, ctx));
        ctx.format_trivias_after_node(instr, block_loop, &mut trivias);
    });
    docs.append(&mut trivias);
    let mut docs = BumpVec::from_iter_in([Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width)], &ctx.bump);
    if block_loop.tokens_by_kind(R_PAREN).next().is_some() {
        docs.push(ctx.format_right_paren_after_instr(block_loop));
    } else {
        if let Some(keyword) = block_loop.tokens_by_kind(KEYWORD).find(|token| token.text() == "end") {
            docs.push(Doc::hard_line());
            docs.push(Doc::text("end"));
            ctx.format_trivias_after_token(keyword, block_loop, &mut trivias);
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
    Doc::slice(docs.into_bump_slice())
}

pub(crate) fn format_block_try_table<'a>(block_try_table: AmberNode<'a>, ctx: &'a Ctx<'a>) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = block_try_table.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::char('('));
        ctx.format_trivias_after_token(l_paren, block_try_table, &mut trivias);
    }
    if let Some(keyword) = block_try_table.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("try_table"));
        ctx.format_trivias_after_token(keyword, block_try_table, &mut trivias);
    }
    if let Some(ident) = block_try_table.tokens_by_kind(IDENT).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.text()));
        ctx.format_trivias_after_token(ident, block_try_table, &mut trivias);
    }
    if let Some(type_use) = block_try_table.children_by_kind(TYPE_USE).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_type_use(type_use, ctx));
        ctx.format_trivias_after_node(type_use, block_try_table, &mut trivias);
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
        ctx.format_trivias_after_node(cat, block_try_table, &mut trivias);
    });
    block_try_table.children_by_kind(Instr::can_cast).for_each(|instr| {
        if trivias.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_instr(instr, ctx));
        ctx.format_trivias_after_node(instr, block_try_table, &mut trivias);
    });
    docs.append(&mut trivias);
    let mut docs = BumpVec::from_iter_in([Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width)], &ctx.bump);
    if block_try_table.tokens_by_kind(R_PAREN).next().is_some() {
        docs.push(ctx.format_right_paren_after_instr(block_try_table));
    } else {
        if let Some(keyword) = block_try_table
            .tokens_by_kind(KEYWORD)
            .find(|token| token.text() == "end")
        {
            docs.push(Doc::hard_line());
            docs.push(Doc::text("end"));
            ctx.format_trivias_after_token(keyword, block_try_table, &mut trivias);
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
    Doc::slice(docs.into_bump_slice())
}

pub(crate) fn format_catch<'a>(catch: AmberNode<'a>, ctx: &'a Ctx<'a>) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = catch.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::char('('));
        ctx.format_trivias_after_token(l_paren, catch, &mut trivias);
    }
    if let Some(keyword) = catch.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text(keyword.text()));
        ctx.format_trivias_after_token(keyword, catch, &mut trivias);
    }
    let mut indexes = catch.children_by_kind(INDEX);
    if let Some(tag_index) = indexes.next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_index(tag_index));
        ctx.format_trivias_after_node(tag_index, catch, &mut trivias);
    }
    if let Some(label_index) = indexes.next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_index(label_index));
        ctx.format_trivias_after_node(label_index, catch, &mut trivias);
    }
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren_on_same_line(catch),
    ]))
}

pub(crate) fn format_catch_all<'a>(catch_all: AmberNode<'a>, ctx: &'a Ctx<'a>) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = catch_all.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::char('('));
        ctx.format_trivias_after_token(l_paren, catch_all, &mut trivias);
    }
    if let Some(keyword) = catch_all.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text(keyword.text()));
        ctx.format_trivias_after_token(keyword, catch_all, &mut trivias);
    }
    if let Some(label_index) = catch_all.children_by_kind(INDEX).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_index(label_index));
        ctx.format_trivias_after_node(label_index, catch_all, &mut trivias);
    }
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren_on_same_line(catch_all),
    ]))
}

pub(crate) fn format_immediate<'a>(immediate: AmberNode<'a>, ctx: &'a Ctx<'a>) -> Doc<'a> {
    match immediate.children_with_tokens().next() {
        Some(NodeOrToken::Node(node)) => match node.kind() {
            TYPE_USE => format_type_use(node, ctx),
            MEM_ARG => format_mem_arg(node, ctx),
            HEAP_TYPE => format_heap_type(node),
            REF_TYPE => format_ref_type(node, ctx),
            ON_CLAUSE => format_on_clause(node, ctx),
            _ => Doc::nil(),
        },
        Some(NodeOrToken::Token(token)) => Doc::text(token.text()),
        None => Doc::nil(),
    }
}

pub(crate) fn format_instr<'a>(instr: AmberNode<'a>, ctx: &'a Ctx<'a>) -> Doc<'a> {
    match instr.kind() {
        PLAIN_INSTR => format_plain_instr(instr, ctx),
        BLOCK_BLOCK => format_block_block(instr, ctx),
        BLOCK_LOOP => format_block_loop(instr, ctx),
        BLOCK_IF => format_block_if(instr, ctx),
        BLOCK_TRY_TABLE => format_block_try_table(instr, ctx),
        _ => Doc::nil(),
    }
}

pub(crate) fn format_mem_arg<'a>(mem_arg: AmberNode<'a>, ctx: &'a Ctx<'a>) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(3, &ctx.bump);
    if let Some(keyword) = mem_arg.tokens_by_kind(MEM_ARG_KEYWORD).next() {
        docs.push(Doc::text(keyword.text()));
    }
    docs.push(Doc::char('='));
    if let Some(unsigned_int) = mem_arg.tokens_by_kind(UNSIGNED_INT).next() {
        docs.push(Doc::text(unsigned_int.text()));
    }
    Doc::slice(docs.into_bump_slice())
}

pub(crate) fn format_on_clause<'a>(on_clause: AmberNode<'a>, ctx: &'a Ctx<'a>) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(7, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = on_clause.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::char('('));
        ctx.format_trivias_after_token(l_paren, on_clause, &mut trivias);
    }
    if let Some(keyword) = on_clause.tokens_by_kind(KEYWORD).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text("on"));
        ctx.format_trivias_after_token(keyword, on_clause, &mut trivias);
    }
    on_clause.children_by_kind(INDEX).for_each(|index| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_index(index));
        ctx.format_trivias_after_node(index, on_clause, &mut trivias);
    });
    if let Some(modifier_keyword) = on_clause.tokens_by_kind(MODIFIER_KEYWORD).next() {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text("switch"));
        ctx.format_trivias_after_token(modifier_keyword, on_clause, &mut trivias);
    }
    docs.append(&mut trivias);
    Doc::slice(ctx.bump.alloc_slice_fill_iter([
        Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width),
        ctx.format_right_paren_on_same_line(on_clause),
    ]))
}

pub(crate) fn format_plain_instr<'a>(plain_instr: AmberNode<'a>, ctx: &'a Ctx<'a>) -> Doc<'a> {
    let mut docs = BumpVec::with_capacity_in(2, &ctx.bump);
    let mut trivias = BumpVec::new_in(&ctx.bump);
    if let Some(l_paren) = plain_instr.tokens_by_kind(L_PAREN).next() {
        docs.push(Doc::char('('));
        ctx.format_trivias_after_token(l_paren, plain_instr, &mut trivias);
    }
    if let Some(instr_name) = plain_instr.tokens_by_kind(INSTR_NAME).next() {
        docs.append(&mut trivias);
        docs.push(Doc::text(instr_name.text()));
        ctx.format_trivias_after_token(instr_name, plain_instr, &mut trivias);
    }
    plain_instr.children_by_kind(IMMEDIATE).for_each(|immediate| {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_immediate(immediate, ctx));
        ctx.format_trivias_after_node(immediate, plain_instr, &mut trivias);
    });
    plain_instr.children_by_kind(Instr::can_cast).for_each(|instr| {
        if trivias.is_empty() {
            docs.push(Doc::hard_line());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(format_instr(instr, ctx));
        ctx.format_trivias_after_node(instr, plain_instr, &mut trivias);
    });
    docs.append(&mut trivias);
    let doc = Doc::slice(docs.into_bump_slice()).nest(ctx.indent_width);
    if plain_instr.tokens_by_kind(R_PAREN).next().is_some() {
        Doc::slice(
            ctx.bump
                .alloc_slice_fill_iter([doc, ctx.format_right_paren_after_instr(plain_instr)]),
        )
    } else {
        doc
    }
}
