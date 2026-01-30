use super::*;
use rowan::ast::AstNode;
use std::mem;
use tiny_pretty::Doc;

impl DocGen for BlockBlock {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("block"));
            trivias = format_trivias_after_token(keyword, ctx);
        }
        if let Some(ident) = self.ident_token() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(Doc::text(ident.to_string()));
            trivias = format_trivias_after_token(ident, ctx);
        }
        if let Some(type_use) = self.type_use() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(type_use.doc(ctx));
            trivias = format_trivias_after_node(type_use, ctx);
        }
        self.instrs().for_each(|instr| {
            if trivias.is_empty() {
                docs.push(Doc::hard_line());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(instr.doc(ctx));
            trivias = format_trivias_after_node(instr, ctx);
        });
        docs.append(&mut trivias);
        let mut docs = vec![Doc::list(docs).nest(ctx.indent_width)];
        if self.r_paren_token().is_some() {
            docs.push(ctx.format_right_paren(self));
        } else {
            if let Some(keyword) = self.end_keyword() {
                docs.push(Doc::hard_line());
                docs.push(Doc::text("end"));
                trivias = format_trivias_after_token(keyword, ctx);
            }
            if let Some(ident) = self.end_ident_token() {
                if trivias.is_empty() {
                    docs.push(Doc::space());
                } else {
                    docs.append(&mut trivias);
                }
                docs.push(Doc::text(ident.to_string()));
            }
        }
        Doc::list(docs).group()
    }
}

impl DocGen for BlockIf {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("if"));
            trivias = format_trivias_after_token(keyword, ctx);
        }
        if let Some(ident) = self.ident_token() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(Doc::text(ident.to_string()));
            trivias = format_trivias_after_token(ident, ctx);
        }
        if let Some(type_use) = self.type_use() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(type_use.doc(ctx));
            trivias = format_trivias_after_node(type_use, ctx);
        }
        self.instrs().for_each(|instr| {
            if trivias.is_empty() {
                docs.push(Doc::hard_line());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(instr.doc(ctx));
            trivias = format_trivias_after_node(instr, ctx);
        });
        if let Some(then_block) = self.then_block() {
            if trivias.is_empty() && then_block.l_paren_token().is_some() {
                docs.push(Doc::hard_line());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(then_block.doc(ctx));
            trivias = format_trivias_after_node(then_block, ctx);
        }
        if let Some(else_block) = self.else_block() {
            if trivias.is_empty() {
                docs.push(Doc::hard_line());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(else_block.doc(ctx));
            trivias = format_trivias_after_node(else_block, ctx);
        }
        docs.push(Doc::list(mem::take(&mut trivias)).nest(ctx.indent_width));
        if self.r_paren_token().is_some() {
            Doc::list(docs)
                .nest(ctx.indent_width)
                .append(ctx.format_right_paren(self))
                .group()
        } else {
            if let Some(keyword) = self.end_keyword() {
                docs.push(Doc::hard_line());
                docs.push(Doc::text("end"));
                trivias = format_trivias_after_token(keyword, ctx);
            }
            if let Some(ident) = self.end_ident_token() {
                if trivias.is_empty() {
                    docs.push(Doc::space());
                } else {
                    docs.append(&mut trivias);
                }
                docs.push(Doc::text(ident.to_string()));
            }
            Doc::list(docs)
        }
    }
}

impl DocGen for BlockIfElse {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("else"));
            trivias = format_trivias_after_token(keyword, ctx);
        }
        if let Some(ident) = self.ident_token() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(Doc::text(ident.to_string()));
            trivias = format_trivias_after_token(ident, ctx);
        }
        self.instrs().for_each(|instr| {
            if trivias.is_empty() {
                docs.push(Doc::hard_line());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(instr.doc(ctx));
            trivias = format_trivias_after_node(instr, ctx);
        });
        docs.append(&mut trivias);
        let doc = Doc::list(docs).nest(ctx.indent_width);
        if self.r_paren_token().is_some() {
            doc.append(ctx.format_right_paren(self)).group()
        } else {
            doc
        }
    }
}

impl DocGen for BlockIfThen {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("then"));
            trivias = format_trivias_after_token(keyword, ctx);
        }
        if let Some(ident) = self.ident_token() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(Doc::text(ident.to_string()));
            trivias = format_trivias_after_token(ident, ctx);
        }
        self.instrs().for_each(|instr| {
            if trivias.is_empty() {
                docs.push(Doc::hard_line());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(instr.doc(ctx));
            trivias = format_trivias_after_node(instr, ctx);
        });
        docs.append(&mut trivias);
        let doc = Doc::list(docs).nest(ctx.indent_width);
        if self.r_paren_token().is_some() {
            doc.append(ctx.format_right_paren(self)).group()
        } else {
            doc
        }
    }
}

impl DocGen for BlockInstr {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        match self {
            BlockInstr::Block(block_block) => block_block.doc(ctx),
            BlockInstr::Loop(block_loop) => block_loop.doc(ctx),
            BlockInstr::If(block_if) => block_if.doc(ctx),
            BlockInstr::TryTable(block_try_table) => block_try_table.doc(ctx),
        }
    }
}

impl DocGen for BlockLoop {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("loop"));
            trivias = format_trivias_after_token(keyword, ctx);
        }
        if let Some(ident) = self.ident_token() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(Doc::text(ident.to_string()));
            trivias = format_trivias_after_token(ident, ctx);
        }
        if let Some(type_use) = self.type_use() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(type_use.doc(ctx));
            trivias = format_trivias_after_node(type_use, ctx);
        }
        self.instrs().for_each(|instr| {
            if trivias.is_empty() {
                docs.push(Doc::hard_line());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(instr.doc(ctx));
            trivias = format_trivias_after_node(instr, ctx);
        });
        docs.append(&mut trivias);
        let mut docs = vec![Doc::list(docs).nest(ctx.indent_width)];
        if self.r_paren_token().is_some() {
            docs.push(ctx.format_right_paren(self));
        } else {
            if let Some(keyword) = self.end_keyword() {
                docs.push(Doc::hard_line());
                docs.push(Doc::text("end"));
                trivias = format_trivias_after_token(keyword, ctx);
            }
            if let Some(ident) = self.end_ident_token() {
                if trivias.is_empty() {
                    docs.push(Doc::space());
                } else {
                    docs.append(&mut trivias);
                }
                docs.push(Doc::text(ident.to_string()));
            }
        }
        Doc::list(docs).group()
    }
}

impl DocGen for BlockTryTable {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("try_table"));
            trivias = format_trivias_after_token(keyword, ctx);
        }
        if let Some(ident) = self.ident_token() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(Doc::text(ident.to_string()));
            trivias = format_trivias_after_token(ident, ctx);
        }
        if let Some(type_use) = self.type_use() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(type_use.doc(ctx));
            trivias = format_trivias_after_node(type_use, ctx);
        }
        self.catches().for_each(|cat| {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(cat.doc(ctx));
            trivias = format_trivias_after_node(cat, ctx);
        });
        self.instrs().for_each(|instr| {
            if trivias.is_empty() {
                docs.push(Doc::hard_line());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(instr.doc(ctx));
            trivias = format_trivias_after_node(instr, ctx);
        });
        docs.append(&mut trivias);
        let mut docs = vec![Doc::list(docs).nest(ctx.indent_width)];
        if self.r_paren_token().is_some() {
            docs.push(ctx.format_right_paren(self));
        } else {
            if let Some(keyword) = self.end_keyword() {
                docs.push(Doc::hard_line());
                docs.push(Doc::text("end"));
                trivias = format_trivias_after_token(keyword, ctx);
            }
            if let Some(ident) = self.end_ident_token() {
                if trivias.is_empty() {
                    docs.push(Doc::space());
                } else {
                    docs.append(&mut trivias);
                }
                docs.push(Doc::text(ident.to_string()));
            }
        }
        Doc::list(docs).group()
    }
}

impl DocGen for Cat {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        match self {
            Cat::Catch(catch) => catch.doc(ctx),
            Cat::CatchAll(catch_all) => catch_all.doc(ctx),
        }
    }
}

impl DocGen for Catch {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text(keyword.text().to_string()));
            trivias = format_trivias_after_token(keyword, ctx);
        }
        if let Some(tag_index) = self.tag_index() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(tag_index.doc(ctx));
            trivias = format_trivias_after_node(tag_index, ctx);
        }
        if let Some(label_index) = self.label_index() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(label_index.doc(ctx));
            trivias = format_trivias_after_node(label_index, ctx);
        }
        docs.append(&mut trivias);
        Doc::list(docs)
            .nest(ctx.indent_width)
            .append(ctx.format_right_paren(self))
            .group()
    }
}

impl DocGen for CatchAll {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text(keyword.text().to_string()));
            trivias = format_trivias_after_token(keyword, ctx);
        }
        if let Some(label_index) = self.label_index() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(label_index.doc(ctx));
            trivias = format_trivias_after_node(label_index, ctx);
        }
        docs.append(&mut trivias);
        Doc::list(docs)
            .nest(ctx.indent_width)
            .append(ctx.format_right_paren(self))
            .group()
    }
}

impl DocGen for Immediate {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if let Some(type_use) = self.type_use() {
            type_use.doc(ctx)
        } else if let Some(mem_arg) = self.mem_arg() {
            mem_arg.doc(ctx)
        } else if let Some(heap_type) = self.heap_type() {
            heap_type.doc(ctx)
        } else if let Some(ref_type) = self.ref_type() {
            ref_type.doc(ctx)
        } else if let Some(token) = self.syntax().first_token() {
            Doc::text(token.to_string())
        } else {
            Doc::nil()
        }
    }
}

impl DocGen for Instr {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        match self {
            Instr::Block(block_instr) => block_instr.doc(ctx),
            Instr::Plain(plain_instr) => plain_instr.doc(ctx),
        }
    }
}

impl DocGen for MemArg {
    fn doc(&self, _: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(3);
        if let Some(keyword) = self.mem_arg_keyword() {
            docs.push(Doc::text(keyword.to_string()));
        }
        docs.push(Doc::text("="));
        if let Some(unsigned_int) = self.unsigned_int() {
            docs.push(Doc::text(unsigned_int.to_string()));
        }
        Doc::list(docs)
    }
}

impl DocGen for PlainInstr {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(instr_name) = self.instr_name() {
            docs.append(&mut trivias);
            docs.push(Doc::text(instr_name.to_string()));
            trivias = format_trivias_after_token(instr_name, ctx);
        }
        self.immediates().for_each(|immediate| {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(immediate.doc(ctx));
            trivias = format_trivias_after_node(immediate, ctx);
        });
        self.instrs().for_each(|instr| {
            if trivias.is_empty() {
                docs.push(Doc::hard_line());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(instr.doc(ctx));
            trivias = format_trivias_after_node(instr, ctx);
        });
        docs.append(&mut trivias);
        let doc = Doc::list(docs).nest(ctx.indent_width);
        if self.r_paren_token().is_some() {
            doc.append(ctx.format_right_paren(self)).group()
        } else {
            doc
        }
    }
}
