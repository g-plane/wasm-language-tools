use super::*;
use rowan::ast::AstNode;
use tiny_pretty::Doc;

impl DocGen for Data {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("data"));
            trivias = format_trivias_after_token(keyword, ctx);
        }
        self.string_tokens().for_each(|string| {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(Doc::text(string.to_string()));
            trivias = format_trivias_after_token(string, ctx);
        });
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for Elem {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("elem"));
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
        self.elem_exprs().for_each(|elem_expr| {
            let has_keyword = elem_expr.keyword().is_some();
            if trivias.is_empty() && has_keyword {
                docs.push(Doc::hard_line().nest(ctx.indent_width));
            } else {
                docs.append(&mut trivias);
            }
            if has_keyword {
                docs.push(elem_expr.doc(ctx).nest(ctx.indent_width));
            } else {
                docs.push(elem_expr.doc(ctx));
            }
            trivias = format_trivias_after_node(elem_expr, ctx);
        });
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for ElemExpr {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("item"));
            trivias = format_trivias_after_token(keyword, ctx);
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
        if self.r_paren_token().is_some() {
            docs.append(&mut trivias);
            docs.push(Doc::text(")"));
        }
        Doc::list(docs)
    }
}

impl DocGen for ElemList {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(keyword) = self.func_keyword() {
            docs.push(Doc::text("func"));
            trivias = format_trivias_after_token(keyword, ctx);
        }
        self.indexes().for_each(|index| {
            if trivias.is_empty() {
                if !docs.is_empty() {
                    docs.push(Doc::space());
                }
            } else {
                docs.append(&mut trivias);
            }
            docs.push(index.doc(ctx));
            trivias = format_trivias_after_node(index, ctx);
        });
        if let Some(ref_type) = self.ref_type() {
            docs.push(ref_type.doc(ctx));
            trivias = format_trivias_after_node(ref_type, ctx);
        }
        self.elem_exprs().for_each(|elem_expr| {
            let has_keyword = elem_expr.keyword().is_some();
            if trivias.is_empty() && has_keyword {
                docs.push(Doc::hard_line().nest(ctx.indent_width));
            } else {
                docs.append(&mut trivias);
            }
            if has_keyword {
                docs.push(elem_expr.doc(ctx).nest(ctx.indent_width));
            } else {
                docs.push(elem_expr.doc(ctx));
            }
            trivias = format_trivias_after_node(elem_expr, ctx);
        });
        docs.append(&mut trivias);
        Doc::list(docs)
    }
}

impl DocGen for Export {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("export"));
            trivias = format_trivias_after_token(keyword, ctx);
        }
        if let Some(name) = self.name() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(name, ctx);
        }
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for ExternIdx {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        match self {
            ExternIdx::Func(extern_idx_func) => extern_idx_func.doc(ctx),
            ExternIdx::Global(extern_idx_global) => extern_idx_global.doc(ctx),
            ExternIdx::Memory(extern_idx_memory) => extern_idx_memory.doc(ctx),
            ExternIdx::Table(extern_idx_table) => extern_idx_table.doc(ctx),
        }
    }
}

impl DocGen for ExternIdxFunc {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        format_extern_idx(self.l_paren_token(), self.keyword(), self.index(), ctx)
    }
}

impl DocGen for ExternIdxGlobal {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        format_extern_idx(self.l_paren_token(), self.keyword(), self.index(), ctx)
    }
}

impl DocGen for ExternIdxMemory {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        format_extern_idx(self.l_paren_token(), self.keyword(), self.index(), ctx)
    }
}

impl DocGen for ExternIdxTable {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        format_extern_idx(self.l_paren_token(), self.keyword(), self.index(), ctx)
    }
}

impl DocGen for Import {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("import"));
            trivias = format_trivias_after_token(keyword, ctx);
        }
        if let Some(module_name) = self.module_name() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(module_name.doc(ctx));
            trivias = format_trivias_after_node(module_name, ctx);
        }
        if let Some(name) = self.name() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(name, ctx);
        }
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for ImportDesc {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        match self {
            ImportDesc::GlobalType(import_desc_global_type) => import_desc_global_type.doc(ctx),
            ImportDesc::MemoryType(import_desc_memory_type) => import_desc_memory_type.doc(ctx),
            ImportDesc::TableType(import_desc_table_type) => import_desc_table_type.doc(ctx),
            ImportDesc::TypeUse(import_desc_type_use) => import_desc_type_use.doc(ctx),
        }
    }
}

impl DocGen for ImportDescGlobalType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("global"));
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
        if let Some(global_type) = self.global_type() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(global_type.doc(ctx));
            trivias = format_trivias_after_node(global_type, ctx);
        }
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for ImportDescMemoryType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("memory"));
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
        if let Some(memory_type) = self.memory_type() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(memory_type.doc(ctx));
            trivias = format_trivias_after_node(memory_type, ctx);
        }
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for ImportDescTableType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("table"));
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
        if let Some(table_type) = self.table_type() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(table_type.doc(ctx));
            trivias = format_trivias_after_node(table_type, ctx);
        }
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for ImportDescTypeUse {
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
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for Index {
    fn doc(&self, _: &Ctx) -> Doc<'static> {
        Doc::text(self.syntax().to_string())
    }
}

impl DocGen for Local {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("local"));
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

impl DocGen for MemUse {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("memory"));
            trivias = format_trivias_after_token(keyword, ctx);
        }
        if let Some(index) = self.index() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(index.doc(ctx));
            trivias = format_trivias_after_node(index, ctx);
        }
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for Module {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        let mut is_explicit_module = true;
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        } else {
            is_explicit_module = false;
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("module"));
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
        let module_fields = Doc::list(self.module_fields().fold(
            vec![],
            |mut fields_docs, module_field| {
                if trivias.is_empty() && (!docs.is_empty() || !fields_docs.is_empty()) {
                    fields_docs.push(Doc::hard_line());
                } else {
                    fields_docs.append(&mut trivias);
                }
                let node = module_field.syntax();
                if should_ignore(node, ctx) {
                    reflow(&node.to_string(), &mut fields_docs);
                } else {
                    fields_docs.push(module_field.doc(ctx));
                }
                trivias = format_trivias_after_node(module_field, ctx);
                fields_docs
            },
        ));
        if is_explicit_module {
            docs.push(module_fields.nest(ctx.indent_width));
        } else {
            docs.push(module_fields);
        }
        docs.append(&mut trivias);
        if is_explicit_module {
            docs.push(Doc::text(")"));
        }
        Doc::list(docs)
    }
}

impl DocGen for ModuleField {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        match self {
            ModuleField::Data(module_field_data) => module_field_data.doc(ctx),
            ModuleField::Elem(module_field_elem) => module_field_elem.doc(ctx),
            ModuleField::Export(module_field_export) => module_field_export.doc(ctx),
            ModuleField::Func(module_field_func) => module_field_func.doc(ctx),
            ModuleField::Global(module_field_global) => module_field_global.doc(ctx),
            ModuleField::Import(module_field_import) => module_field_import.doc(ctx),
            ModuleField::Memory(module_field_memory) => module_field_memory.doc(ctx),
            ModuleField::Start(module_field_start) => module_field_start.doc(ctx),
            ModuleField::Table(module_field_table) => module_field_table.doc(ctx),
            ModuleField::Type(type_def) => type_def.doc(ctx),
            ModuleField::RecType(rec_type) => rec_type.doc(ctx),
        }
    }
}

impl DocGen for ModuleFieldData {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("data"));
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
        if let Some(mem_use) = self.mem_use() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(mem_use.doc(ctx));
            trivias = format_trivias_after_node(mem_use, ctx);
        }
        if let Some(offset) = self.offset() {
            let has_keyword = offset.keyword().is_some();
            if trivias.is_empty() && has_keyword {
                docs.push(Doc::hard_line().nest(ctx.indent_width));
            } else {
                docs.append(&mut trivias);
            }
            if has_keyword {
                docs.push(offset.doc(ctx).nest(ctx.indent_width));
            } else {
                docs.push(offset.doc(ctx));
            }
            trivias = format_trivias_after_node(offset, ctx);
        }
        self.string_tokens().for_each(|string| {
            if trivias.is_empty() {
                docs.push(Doc::soft_line());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(Doc::text(string.to_string()));
            trivias = format_trivias_after_token(string, ctx);
        });
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for ModuleFieldElem {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("elem"));
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
        if let Some(keyword) = self.declare_keyword() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(Doc::text("declare"));
            trivias = format_trivias_after_token(keyword, ctx);
        }
        if let Some(table_use) = self.table_use() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(table_use.doc(ctx));
            trivias = format_trivias_after_node(table_use, ctx);
        }
        if let Some(offset) = self.offset() {
            let has_keyword = offset.keyword().is_some();
            if trivias.is_empty() && has_keyword {
                docs.push(Doc::hard_line().nest(ctx.indent_width));
            } else {
                docs.append(&mut trivias);
            }
            if has_keyword {
                docs.push(offset.doc(ctx).nest(ctx.indent_width));
            } else {
                docs.push(offset.doc(ctx));
            }
            trivias = format_trivias_after_node(offset, ctx);
        }
        if let Some(elem_list) = self.elem_list() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(elem_list.doc(ctx));
            trivias = format_trivias_after_node(elem_list, ctx);
        }
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for ModuleFieldExport {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("export"));
            trivias = format_trivias_after_token(keyword, ctx);
        }
        if let Some(name) = self.name() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(name, ctx);
        }
        if let Some(extern_idx) = self.extern_idx() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(extern_idx.doc(ctx));
            trivias = format_trivias_after_node(extern_idx, ctx);
        }
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for ModuleFieldFunc {
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
        if let Some(ident) = self.ident_token() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(Doc::text(ident.to_string()));
            trivias = format_trivias_after_token(ident, ctx);
        }
        if let Some(import) = self.import() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(import.doc(ctx));
            trivias = format_trivias_after_node(import, ctx);
        }
        self.exports().for_each(|export| {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(export.doc(ctx));
            trivias = format_trivias_after_node(export, ctx);
        });
        if let Some(type_use) = self.type_use() {
            if trivias.is_empty() {
                if type_use.keyword().is_some() {
                    docs.push(Doc::space());
                } else {
                    docs.push(Doc::soft_line().nest(ctx.indent_width));
                }
            } else {
                docs.append(&mut trivias);
            }
            docs.push(type_use.doc(ctx));
            trivias = format_trivias_after_node(type_use, ctx);
        }
        let mut locals = self.locals();
        if let Some(local) = locals.next() {
            if trivias.is_empty() {
                docs.push(Doc::soft_line().nest(ctx.indent_width));
            } else {
                docs.append(&mut trivias);
            }
            docs.push(local.doc(ctx));
            trivias = format_trivias_after_node(local, ctx);
        }
        locals.for_each(|local| {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(local.doc(ctx));
            trivias = format_trivias_after_node(local, ctx);
        });
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
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for ModuleFieldGlobal {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("global"));
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
        if let Some(import) = self.import() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(import.doc(ctx));
            trivias = format_trivias_after_node(import, ctx);
        }
        if let Some(export) = self.export() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(export.doc(ctx));
            trivias = format_trivias_after_node(export, ctx);
        }
        if let Some(global_type) = self.global_type() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(global_type.doc(ctx));
            trivias = format_trivias_after_node(global_type, ctx);
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
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for ModuleFieldImport {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("import"));
            trivias = format_trivias_after_token(keyword, ctx);
        }
        if let Some(module_name) = self.module_name() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(module_name.doc(ctx));
            trivias = format_trivias_after_node(module_name, ctx);
        }
        if let Some(name) = self.name() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(name, ctx);
        }
        if let Some(import_desc) = self.import_desc() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(import_desc.doc(ctx));
            trivias = format_trivias_after_node(import_desc, ctx);
        }
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for ModuleFieldMemory {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("memory"));
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
        if let Some(import) = self.import() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(import.doc(ctx));
            trivias = format_trivias_after_node(import, ctx);
        }
        if let Some(export) = self.export() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(export.doc(ctx));
            trivias = format_trivias_after_node(export, ctx);
        }
        if let Some(memory_type) = self.memory_type() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(memory_type.doc(ctx));
            trivias = format_trivias_after_node(memory_type, ctx);
        }
        if let Some(data) = self.data() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(data.doc(ctx));
            trivias = format_trivias_after_node(data, ctx);
        }
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for ModuleFieldStart {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("start"));
            trivias = format_trivias_after_token(keyword, ctx);
        }
        if let Some(index) = self.index() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(index.doc(ctx));
            trivias = format_trivias_after_node(index, ctx);
        }
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for ModuleFieldTable {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("table"));
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
        if let Some(import) = self.import() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(import.doc(ctx));
            trivias = format_trivias_after_node(import, ctx);
        }
        if let Some(export) = self.export() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(export.doc(ctx));
            trivias = format_trivias_after_node(export, ctx);
        }
        if let Some(table_type) = self.table_type() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(table_type.doc(ctx));
            trivias = format_trivias_after_node(table_type, ctx);
        }
        if let Some(ref_type) = self.ref_type() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(ref_type.doc(ctx));
            trivias = format_trivias_after_node(ref_type, ctx);
        }
        if let Some(elem) = self.elem() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(elem.doc(ctx));
            trivias = format_trivias_after_node(elem, ctx);
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
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for ModuleName {
    fn doc(&self, _: &Ctx) -> Doc<'static> {
        Doc::text(self.syntax().to_string())
    }
}

impl DocGen for Name {
    fn doc(&self, _: &Ctx) -> Doc<'static> {
        Doc::text(self.syntax().to_string())
    }
}

impl DocGen for Offset {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("offset"));
            trivias = format_trivias_after_token(keyword, ctx);
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
        if self.r_paren_token().is_some() {
            docs.append(&mut trivias);
            docs.push(Doc::text(")"));
        }
        Doc::list(docs)
    }
}

impl DocGen for RecType {
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
            docs.push(Doc::text("rec"));
            preferred_multi_line = has_line_break_after_token(&keyword);
            trivias = format_trivias_after_token(keyword, ctx);
        }
        self.type_defs().for_each(|type_def| {
            if trivias.is_empty() {
                if preferred_multi_line {
                    docs.push(Doc::hard_line());
                } else {
                    docs.push(Doc::line_or_space());
                }
            } else {
                docs.append(&mut trivias);
            }
            docs.push(type_def.doc(ctx));
            trivias = format_trivias_after_node(type_def, ctx);
        });
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs).nest(ctx.indent_width).group()
    }
}

impl DocGen for TableUse {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("table"));
            trivias = format_trivias_after_token(keyword, ctx);
        }
        if let Some(index) = self.index() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(index.doc(ctx));
            trivias = format_trivias_after_node(index, ctx);
        }
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl DocGen for TypeDef {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("type"));
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
        if let Some(sub_type) = self.sub_type() {
            if trivias.is_empty() {
                docs.push(Doc::line_or_space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(sub_type.doc(ctx));
            trivias = format_trivias_after_node(sub_type, ctx);
        }
        docs.append(&mut trivias);
        docs.push(Doc::text(")"));
        Doc::list(docs).nest(ctx.indent_width).group()
    }
}

impl DocGen for TypeUse {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(2);
        let mut trivias = vec![];
        if let Some(l_paren) = self.l_paren_token() {
            docs.push(Doc::text("("));
            trivias = format_trivias_after_token(l_paren, ctx);
        }
        if let Some(keyword) = self.keyword() {
            docs.append(&mut trivias);
            docs.push(Doc::text("type"));
            trivias = format_trivias_after_token(keyword, ctx);
        }
        if let Some(index) = self.index() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(index.doc(ctx));
            trivias = format_trivias_after_node(index, ctx);
        }
        if let Some(r_paren) = self.r_paren_token() {
            docs.append(&mut trivias);
            docs.push(Doc::text(")"));
            trivias = format_trivias_after_token(r_paren, ctx);
        }

        let mut params = self.params();
        if let Some(param) = params.next() {
            if trivias.is_empty() && !docs.is_empty() {
                docs.push(Doc::soft_line().nest(ctx.indent_width));
            } else if self.l_paren_token().is_some() {
                docs.append(&mut trivias);
            }
            docs.push(param.doc(ctx));
            trivias = format_trivias_after_node(param, ctx);
        }
        params.for_each(|param| {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(param.doc(ctx));
            trivias = format_trivias_after_node(param, ctx);
        });
        let mut results = self.results();
        if let Some(result) = results.next() {
            if trivias.is_empty() && !docs.is_empty() {
                docs.push(Doc::space());
            } else if self.l_paren_token().is_some() {
                docs.append(&mut trivias);
            }
            docs.push(result.doc(ctx));
            trivias = format_trivias_after_node(result, ctx);
        }
        results.for_each(|result| {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(result.doc(ctx));
            trivias = format_trivias_after_node(result, ctx);
        });

        Doc::list(docs)
    }
}

fn format_extern_idx(
    l_paren: Option<SyntaxToken>,
    keyword: Option<SyntaxToken>,
    index: Option<Index>,
    ctx: &Ctx,
) -> Doc<'static> {
    let mut docs = Vec::with_capacity(2);
    let mut trivias = vec![];
    if let Some(l_paren) = l_paren {
        docs.push(Doc::text("("));
        trivias = format_trivias_after_token(l_paren, ctx);
    }
    if let Some(keyword) = keyword {
        docs.append(&mut trivias);
        docs.push(Doc::text(keyword.to_string()));
        trivias = format_trivias_after_token(keyword, ctx);
    }
    if let Some(index) = index {
        if trivias.is_empty() {
            docs.push(Doc::space());
        } else {
            docs.append(&mut trivias);
        }
        docs.push(index.doc(ctx));
        trivias = format_trivias_after_node(index, ctx);
    }
    docs.append(&mut trivias);
    docs.push(Doc::text(")"));
    Doc::list(docs)
}
