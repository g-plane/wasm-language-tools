use super::*;
use rowan::ast::AstNode;
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
        if let Some(block_type) = self.block_type() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(block_type.doc(ctx));
            trivias = format_trivias_after_node(block_type, ctx);
        }
        docs.push(
            Doc::list(self.instrs().fold(vec![], |mut docs, instr| {
                if trivias.is_empty() {
                    docs.push(Doc::hard_line());
                } else {
                    docs.append(&mut trivias);
                }
                docs.push(instr.doc(ctx));
                trivias = format_trivias_after_node(instr, ctx);
                docs
            }))
            .nest(ctx.indent_width),
        );
        if let Some(r_paren) = self.r_paren_token() {
            docs.append(&mut trivias);
            docs.push(Doc::text(")"));
            trivias = format_trivias_after_token(r_paren, ctx);
        }
        if let Some(keyword) = self.end_keyword() {
            if trivias.is_empty() {
                docs.push(Doc::hard_line());
            } else {
                docs.append(&mut trivias);
            }
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

impl DocGen for BlockIf {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        let mut is_folded = false;
        if let Some(l_paren) = self.l_paren_token() {
            is_folded = true;
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
        if let Some(block_type) = self.block_type() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(block_type.doc(ctx));
            trivias = format_trivias_after_node(block_type, ctx);
        }
        let mut block_docs = Vec::with_capacity(3);
        self.instrs().for_each(|instr| {
            if trivias.is_empty() {
                block_docs.push(Doc::hard_line());
            } else {
                block_docs.append(&mut trivias);
            }
            block_docs.push(instr.doc(ctx));
            trivias = format_trivias_after_node(instr, ctx);
        });
        if let Some(then_block) = self.then_block() {
            if trivias.is_empty() && then_block.l_paren_token().is_some() {
                block_docs.push(Doc::hard_line());
            } else {
                block_docs.append(&mut trivias);
            }
            block_docs.push(then_block.doc(ctx));
            trivias = format_trivias_after_node(then_block, ctx);
        }
        if let Some(else_block) = self.else_block() {
            if trivias.is_empty() {
                block_docs.push(Doc::hard_line());
            } else {
                block_docs.append(&mut trivias);
            }
            block_docs.push(else_block.doc(ctx));
            trivias = format_trivias_after_node(else_block, ctx);
        }
        if is_folded {
            docs.push(Doc::list(block_docs).nest(ctx.indent_width));
        } else {
            docs.append(&mut block_docs);
        }
        if let Some(r_paren) = self.r_paren_token() {
            docs.append(&mut trivias);
            docs.push(Doc::text(")"));
            trivias = format_trivias_after_token(r_paren, ctx);
        }
        if let Some(keyword) = self.end_keyword() {
            if trivias.is_empty() {
                docs.push(Doc::hard_line());
            } else {
                docs.append(&mut trivias);
            }
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
        if self.r_paren_token().is_some() {
            docs.append(&mut trivias);
            docs.push(Doc::text(")"));
        }
        Doc::list(docs).nest(ctx.indent_width)
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
        if self.r_paren_token().is_some() {
            docs.append(&mut trivias);
            docs.push(Doc::text(")"));
        }
        Doc::list(docs).nest(ctx.indent_width)
    }
}

impl DocGen for BlockInstr {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        match self {
            BlockInstr::Block(block_block) => block_block.doc(ctx),
            BlockInstr::Loop(block_loop) => block_loop.doc(ctx),
            BlockInstr::If(block_if) => block_if.doc(ctx),
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
        if let Some(block_type) = self.block_type() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(block_type.doc(ctx));
            trivias = format_trivias_after_node(block_type, ctx);
        }
        docs.push(
            Doc::list(self.instrs().fold(vec![], |mut docs, instr| {
                if trivias.is_empty() {
                    docs.push(Doc::hard_line());
                } else {
                    docs.append(&mut trivias);
                }
                docs.push(instr.doc(ctx));
                trivias = format_trivias_after_node(instr, ctx);
                docs
            }))
            .nest(ctx.indent_width),
        );
        if let Some(r_paren) = self.r_paren_token() {
            docs.append(&mut trivias);
            docs.push(Doc::text(")"));
            trivias = format_trivias_after_token(r_paren, ctx);
        }
        if let Some(keyword) = self.end_keyword() {
            if trivias.is_empty() {
                docs.push(Doc::hard_line());
            } else {
                docs.append(&mut trivias);
            }
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

impl DocGen for BlockType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if let Some(type_use) = self.type_use() {
            type_use.doc(ctx)
        } else {
            Doc::nil()
        }
    }
}

impl DocGen for Immediate {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if let Some(type_use) = self.type_use() {
            type_use.doc(ctx)
        } else if let Some(heap_type) = self.heap_type() {
            heap_type.doc(ctx)
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
        if self.r_paren_token().is_some() {
            docs.append(&mut trivias);
            docs.push(Doc::text(")"));
        }
        Doc::list(docs).nest(ctx.indent_width)
    }
}
