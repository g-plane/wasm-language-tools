use super::*;
use tiny_pretty::Doc;

impl DocGen for AddrType {
    fn doc(&self, _: &Ctx) -> Doc<'static> {
        if let Some(token) = self.syntax().first_token() {
            Doc::text(token.text().to_string())
        } else {
            Doc::nil()
        }
    }
}

impl DocGen for ArrayType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("array"));
            trivias = format_trivias_after_token(keyword, ctx);
        }
        if let Some(field_type) = self.field_type() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(field_type.doc(ctx));
            trivias = format_trivias_after_node(field_type, ctx);
        }
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for CompType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        match self {
            CompType::Array(array_type) => array_type.doc(ctx),
            CompType::Struct(struct_type) => struct_type.doc(ctx),
            CompType::Func(func_type) => func_type.doc(ctx),
        }
    }
}

impl DocGen for ExternType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        match self {
            ExternType::Func(func) => func.doc(ctx),
            ExternType::Global(global) => global.doc(ctx),
            ExternType::Memory(memory) => memory.doc(ctx),
            ExternType::Table(table) => table.doc(ctx),
            ExternType::Tag(tag) => tag.doc(ctx),
        }
    }
}

impl DocGen for ExternTypeFunc {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        format_extern_type(
            self.l_paren_token(),
            self.keyword(),
            self.ident_token(),
            self.type_use(),
            ctx,
        )
    }
}

impl DocGen for ExternTypeGlobal {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        format_extern_type(
            self.l_paren_token(),
            self.keyword(),
            self.ident_token(),
            self.global_type(),
            ctx,
        )
    }
}

impl DocGen for ExternTypeMemory {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        format_extern_type(
            self.l_paren_token(),
            self.keyword(),
            self.ident_token(),
            self.memory_type(),
            ctx,
        )
    }
}

impl DocGen for ExternTypeTable {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        format_extern_type(
            self.l_paren_token(),
            self.keyword(),
            self.ident_token(),
            self.table_type(),
            ctx,
        )
    }
}

impl DocGen for ExternTypeTag {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        format_extern_type(
            self.l_paren_token(),
            self.keyword(),
            self.ident_token(),
            self.type_use(),
            ctx,
        )
    }
}

impl DocGen for Field {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("field"));
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
        self.field_types().for_each(|field_type| {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(field_type.doc(ctx));
            trivias = format_trivias_after_node(field_type, ctx);
        });
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for FieldType {
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
            if let Some(ty) = self.storage_type() {
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
        } else if let Some(ty) = self.storage_type() {
            ty.doc(ctx)
        } else {
            Doc::nil()
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
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(addr_type) = self.addr_type() {
            docs.push(addr_type.doc(ctx));
            trivias = format_trivias_after_node(addr_type, ctx);
        }
        if let Some(limits) = self.limits() {
            if trivias.is_empty() && !docs.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(limits.doc(ctx));
        }
        Doc::list(docs)
    }
}

impl DocGen for NumType {
    fn doc(&self, _: &Ctx) -> Doc<'static> {
        if let Some(type_keyword) = self.type_keyword() {
            Doc::text(type_keyword.text().to_string())
        } else {
            Doc::nil()
        }
    }
}

impl DocGen for PackedType {
    fn doc(&self, _: &Ctx) -> Doc<'static> {
        if let Some(type_keyword) = self.type_keyword() {
            Doc::text(type_keyword.text().to_string())
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
        if let Some(l_paren) = self.l_paren_token() {
            let mut docs = Vec::with_capacity(2);
            docs.push(Doc::text("("));
            let mut trivias = format_trivias_after_token(l_paren, ctx);
            if let Some(keyword) = self.keyword() {
                docs.append(&mut trivias);
                docs.push(Doc::text("ref"));
                trivias = format_trivias_after_token(keyword, ctx);
            }
            if let Some(keyword) = self.null_keyword() {
                if trivias.is_empty() {
                    docs.push(Doc::space());
                } else {
                    docs.append(&mut trivias);
                }
                docs.push(Doc::text("null"));
                trivias = format_trivias_after_token(keyword, ctx);
            }
            if let Some(heap_type) = self.heap_type() {
                if trivias.is_empty() {
                    docs.push(Doc::space());
                } else {
                    docs.append(&mut trivias);
                }
                docs.push(heap_type.doc(ctx));
                trivias = format_trivias_after_node(heap_type, ctx);
            }
            docs.append(&mut trivias);
            docs.push(Doc::text(")"));
            Doc::list(docs)
        } else if let Some(type_keyword) = self.type_keyword() {
            Doc::text(type_keyword.text().to_string())
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

impl DocGen for StorageType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        match self {
            StorageType::Val(val_type) => val_type.doc(ctx),
            StorageType::Packed(packed_type) => packed_type.doc(ctx),
        }
    }
}

impl DocGen for StructType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut preferred_multi_line = false;
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("struct"));
            preferred_multi_line = has_line_break_after_token(&keyword);
            trivias = format_trivias_after_token(keyword, ctx);
        }
        self.fields().for_each(|field| {
            if trivias.is_empty() {
                if preferred_multi_line {
                    docs.push(Doc::hard_line());
                } else {
                    docs.push(Doc::line_or_space());
                }
            } else {
                docs.append(&mut trivias);
            }
            docs.push(field.doc(ctx));
            trivias = format_trivias_after_node(field, ctx);
        });
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs).nest(ctx.indent_width).group()
    }
}

impl DocGen for SubType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if let Some(l_paren) = self.l_paren_token() {
            let mut docs = Vec::with_capacity(2);
            docs.push(Doc::text("("));
            let mut trivias = format_trivias_after_token(l_paren, ctx);
            if let Some(keyword) = self.keyword() {
                docs.append(&mut trivias);
                docs.push(Doc::text("sub"));
                trivias = format_trivias_after_token(keyword, ctx);
            }
            if let Some(keyword) = self.final_keyword() {
                if trivias.is_empty() {
                    docs.push(Doc::space());
                } else {
                    docs.append(&mut trivias);
                }
                docs.push(Doc::text("final"));
                trivias = format_trivias_after_token(keyword, ctx);
            }
            self.indexes().for_each(|index| {
                if trivias.is_empty() {
                    docs.push(Doc::space());
                } else {
                    docs.append(&mut trivias);
                }
                docs.push(index.doc(ctx));
                trivias = format_trivias_after_node(index, ctx);
            });
            if let Some(ty) = self.comp_type() {
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
        } else if let Some(comp_type) = self.comp_type() {
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
        if let Some(addr_type) = self.addr_type() {
            docs.push(addr_type.doc(ctx));
            trivias = format_trivias_after_node(addr_type, ctx);
        }
        if let Some(limits) = self.limits() {
            if trivias.is_empty() && !docs.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
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
        match self {
            ValType::Num(num_type) => num_type.doc(ctx),
            ValType::Vec(vec_type) => vec_type.doc(ctx),
            ValType::Ref(ref_type) => ref_type.doc(ctx),
        }
    }
}

impl DocGen for VecType {
    fn doc(&self, _: &Ctx) -> Doc<'static> {
        if let Some(type_keyword) = self.type_keyword() {
            Doc::text(type_keyword.text().to_string())
        } else {
            Doc::nil()
        }
    }
}

fn format_extern_type<N>(
    l_paren: Option<SyntaxToken>,
    keyword: Option<SyntaxToken>,
    ident: Option<SyntaxToken>,
    ty: Option<N>,
    ctx: &Ctx,
) -> Doc<'static>
where
    N: AstNode<Language = WatLanguage> + DocGen,
{
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = l_paren {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, ctx);
    }
    if let Some(keyword) = keyword {
        docs.append(&mut trivias);
        docs.push(Doc::text(keyword.text().to_string()));
        trivias = format_trivias_after_token(keyword, ctx);
    }
    if let Some(ident) = ident {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(Doc::text(ident.to_string()));
        trivias = format_trivias_after_token(ident, ctx);
    }
    if let Some(ty) = ty {
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
}
