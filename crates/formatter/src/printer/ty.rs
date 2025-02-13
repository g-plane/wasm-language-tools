use super::*;
use tiny_pretty::Doc;

impl DocGen for CompType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        match self {
            CompType::Array(array_type) => Doc::text(array_type.syntax().to_string()),
            CompType::Struct(struct_type) => Doc::text(struct_type.syntax().to_string()),
            CompType::Func(func_type) => func_type.doc(ctx),
        }
    }
}

impl DocGen for FuncType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("func"));
            trivias = format_trivias_after_token(keyword, ctx);
        }
        self.params().for_each(|param| {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(param.doc(ctx));
            trivias = format_trivias_after_node(param, ctx);
        });
        self.results().for_each(|result| {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(result.doc(ctx));
            trivias = format_trivias_after_node(result, ctx);
        });
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for GlobalType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if let Some(l_paren) = self.l_paren_token() {
            let mut docs = Vec::with_capacity(2);
            docs.push(Doc::text("("));
            let mut trivias = format_trivias_after_token(l_paren, ctx);
            if let Some(keyword) = self.mut_keyword() {
                docs.append(&mut trivias);
                docs.push(Doc::text("mut"));
                trivias = format_trivias_after_token(keyword, ctx);
            }
            if let Some(ty) = self.val_type() {
                if trivias.is_empty() {
                    docs.push(Doc::space());
                } else {
                    docs.append(&mut trivias);
                }
                docs.push(ty.doc(ctx));
                trivias = format_trivias_after_node(ty, ctx);
            }
            docs.append(&mut trivias);
            docs.push(Doc::text(")"));
            Doc::list(docs)
        } else if let Some(ty) = self.val_type() {
            ty.doc(ctx)
        } else {
            Doc::nil()
        }
    }
}

impl DocGen for HeapType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if let Some(type_keyword) = self.type_keyword() {
            Doc::text(type_keyword.text().to_string())
        } else if let Some(index) = self.index() {
            index.doc(ctx)
        } else {
            Doc::nil()
        }
    }
}

impl DocGen for Limits {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(min) = self.min() {
            docs.push(Doc::text(min.to_string()));
            trivias = format_trivias_after_token(min, ctx);
        }
        if let Some(max) = self.max() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(Doc::text(max.to_string()));
        }
        Doc::list(docs)
    }
}

impl DocGen for MemoryType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if let Some(limits) = self.limits() {
            limits.doc(ctx)
        } else {
            Doc::nil()
        }
    }
}

impl DocGen for Param {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("param"));
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
        self.val_types().for_each(|val_type| {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(val_type.doc(ctx));
            trivias = format_trivias_after_node(val_type, ctx);
        });
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for RefType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if let Some(type_keyword) = self.type_keyword() {
            Doc::text(type_keyword.text().to_string())
        } else if let Some(heap_type) = self.heap_type() {
            heap_type.doc(ctx)
        } else {
            Doc::nil()
        }
    }
}

impl DocGen for Result {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("result"));
            trivias = format_trivias_after_token(keyword, ctx);
        }
        self.val_types().for_each(|val_type| {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(val_type.doc(ctx));
            trivias = format_trivias_after_node(val_type, ctx);
        });
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for SubType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if let Some(comp_type) = self.comp_type() {
            comp_type.doc(ctx)
        } else {
            Doc::nil()
        }
    }
}

impl DocGen for TableType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(limits) = self.limits() {
            docs.push(limits.doc(ctx));
            trivias = format_trivias_after_node(limits, ctx);
        }
        if let Some(ref_type) = self.ref_type() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(ref_type.doc(ctx));
        }
        Doc::list(docs)
    }
}

impl DocGen for ValType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if let Some(num_type) = self.num_type() {
            Doc::text(num_type.text().to_string())
        } else if let Some(vec_type) = self.vec_type() {
            Doc::text(vec_type.text().to_string())
        } else if let Some(ref_type) = self.ref_type() {
            ref_type.doc(ctx)
        } else {
            Doc::nil()
        }
    }
}
