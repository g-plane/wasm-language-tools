use super::{node, GreenElement, Parser};
use crate::error::Message;
use rowan::{GreenNode, Language, NodeOrToken};
use wat_syntax::SyntaxKind::*;

impl Parser<'_> {
    fn parse_elem(&mut self) -> Option<GreenNode> {
        let mut children = vec![self.lexer.next(L_PAREN)?.into()];
        self.parse_trivias(&mut children);
        children.push(self.parse_keyword("elem")?);

        if self.lexer.peek(L_PAREN).is_some() {
            while self.recover(Self::parse_elem_expr, &mut children) {}
        } else {
            while self.recover(Self::parse_index, &mut children) {}
        }
        self.expect_right_paren(&mut children);
        Some(node(ELEM, children))
    }

    fn parse_elem_expr(&mut self) -> Option<GreenNode> {
        if let Some(mut children) = self.try_parse(|parser| {
            let mut children = Vec::with_capacity(5);
            children.push(parser.lexer.next(L_PAREN)?.into());
            parser.parse_trivias(&mut children);
            children.push(parser.parse_keyword("item")?);
            Some(children)
        }) {
            while self.recover(Self::parse_instr, &mut children) {}
            self.expect_right_paren(&mut children);
            Some(node(ELEM_EXPR, children))
        } else if self.lexer.peek(L_PAREN).is_some() {
            self.parse_instr()
                .map(|instr| node(ELEM_EXPR, [instr.into()]))
        } else {
            None
        }
    }

    fn parse_export(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(5);
        children.push(self.lexer.next(L_PAREN)?.into());
        self.parse_trivias(&mut children);
        children.push(self.parse_keyword("export")?);
        if !self.recover(Self::parse_name, &mut children) {
            self.report_missing(Message::Name("name"));
        }
        self.expect_right_paren(&mut children);
        Some(node(EXPORT, children))
    }

    fn parse_export_desc(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(3);
        children.push(self.lexer.next(L_PAREN)?.into());
        self.parse_trivias(&mut children);
        let keyword = self.lexer.next(KEYWORD)?;
        let kind = match keyword.text {
            "func" => EXPORT_DESC_FUNC,
            "table" => EXPORT_DESC_TABLE,
            "memory" => EXPORT_DESC_MEMORY,
            "global" => EXPORT_DESC_GLOBAL,
            _ => return None,
        };
        children.push(keyword.into());

        if !self.recover(Self::parse_index, &mut children) {
            self.report_missing(Message::Name("index"));
        }
        self.expect_right_paren(&mut children);
        Some(node(kind, children))
    }

    fn parse_import(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(5);
        children.push(self.lexer.next(L_PAREN)?.into());
        self.parse_trivias(&mut children);
        children.push(self.parse_keyword("import")?);
        if !self.recover(Self::parse_module_name, &mut children) {
            self.report_missing(Message::Name("module name"));
        }
        if !self.recover(Self::parse_name, &mut children) {
            self.report_missing(Message::Name("name"));
        }
        self.expect_right_paren(&mut children);
        Some(node(IMPORT, children))
    }

    fn parse_import_desc(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(5);
        children.push(self.lexer.next(L_PAREN)?.into());
        self.parse_trivias(&mut children);
        let keyword = self.lexer.next(KEYWORD)?;
        match keyword.text {
            "func" => {
                children.push(keyword.into());
                self.eat(IDENT, &mut children);
                if !self.recover(Self::parse_type_use, &mut children) {
                    self.report_missing(Message::Name("type use"));
                }
                self.expect_right_paren(&mut children);
                Some(node(IMPORT_DESC_TYPE_USE, children))
            }
            "global" => {
                children.push(keyword.into());
                self.eat(IDENT, &mut children);
                if !self.recover(Self::parse_global_type, &mut children) {
                    self.report_missing(Message::Name("global type"));
                }
                self.expect_right_paren(&mut children);
                Some(node(IMPORT_DESC_GLOBAL_TYPE, children))
            }
            "memory" => {
                children.push(keyword.into());
                self.eat(IDENT, &mut children);
                if !self.recover(Self::parse_memory_type, &mut children) {
                    self.report_missing(Message::Name("memory type"));
                }
                self.expect_right_paren(&mut children);
                Some(node(IMPORT_DESC_MEMORY_TYPE, children))
            }
            "table" => {
                children.push(keyword.into());
                self.eat(IDENT, &mut children);
                if !self.recover(Self::parse_table_type, &mut children) {
                    self.report_missing(Message::Name("table type"));
                }
                self.expect_right_paren(&mut children);
                Some(node(IMPORT_DESC_TABLE_TYPE, children))
            }
            _ => None,
        }
    }

    pub(super) fn parse_index(&mut self) -> Option<GreenNode> {
        self.lexer
            .eat(IDENT)
            .or_else(|| self.lexer.eat(UNSIGNED_INT))
            .map(|token| node(INDEX, [token.into()]))
    }

    fn parse_local(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(6);
        children.push(self.lexer.next(L_PAREN)?.into());
        self.parse_trivias(&mut children);
        children.push(self.parse_keyword("local")?);

        if self.eat(IDENT, &mut children) {
            if !self.recover(Self::parse_value_type, &mut children) {
                self.report_missing(Message::Name("value type"));
            }
        } else {
            while self.recover(Self::parse_value_type, &mut children) {}
        }
        self.expect_right_paren(&mut children);
        Some(node(LOCAL, children))
    }

    fn parse_mem_use(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(5);
        children.push(self.lexer.next(L_PAREN)?.into());
        self.parse_trivias(&mut children);
        children.push(self.parse_keyword("memory")?);
        if !self.recover(Self::parse_index, &mut children) {
            self.report_missing(Message::Name("index"));
        }
        self.expect_right_paren(&mut children);
        Some(node(MEM_USE, children))
    }

    pub(super) fn parse_module(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(5);
        children.push(self.lexer.next(L_PAREN)?.into());
        self.parse_trivias(&mut children);
        let keyword = self.lexer.next(KEYWORD)?;
        let keyword_text = keyword.text;
        children.push(keyword.into());

        self.lexer.top_level = false;
        let node = match keyword_text {
            "module" => {
                self.eat(IDENT, &mut children);
                while self.recover(Self::parse_module_field, &mut children) {}
                self.expect_right_paren(&mut children);
                Some(node(MODULE, children))
            }
            // wabt allows top-level module fields
            "func" => {
                let mut children = vec![self.parse_module_field_func(children)?.into()];
                while self.recover(Self::parse_module_field, &mut children) {}
                Some(node(MODULE, children))
            }
            "type" => {
                let mut children = vec![self.parse_type_def(children)?.into()];
                while self.recover(Self::parse_module_field, &mut children) {}
                Some(node(MODULE, children))
            }
            "export" => {
                let mut children = vec![self.parse_module_field_export(children)?.into()];
                while self.recover(Self::parse_module_field, &mut children) {}
                Some(node(MODULE, children))
            }
            "import" => {
                let mut children = vec![self.parse_module_field_import(children)?.into()];
                while self.recover(Self::parse_module_field, &mut children) {}
                Some(node(MODULE, children))
            }
            "start" => {
                let mut children = vec![self.parse_module_field_start(children)?.into()];
                while self.recover(Self::parse_module_field, &mut children) {}
                Some(node(MODULE, children))
            }
            "data" => {
                let mut children = vec![self.parse_module_field_data(children)?.into()];
                while self.recover(Self::parse_module_field, &mut children) {}
                Some(node(MODULE, children))
            }
            "table" => {
                let mut children = vec![self.parse_module_field_table(children)?.into()];
                while self.recover(Self::parse_module_field, &mut children) {}
                Some(node(MODULE, children))
            }
            "global" => {
                let mut children = vec![self.parse_module_field_global(children)?.into()];
                while self.recover(Self::parse_module_field, &mut children) {}
                Some(node(MODULE, children))
            }
            "rec" => {
                let mut children = vec![self.parse_rec_type(children)?.into()];
                while self.recover(Self::parse_module_field, &mut children) {}
                Some(node(MODULE, children))
            }
            _ => None,
        };
        self.lexer.top_level = true;
        node
    }

    fn parse_module_field(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(3);
        children.push(self.lexer.next(L_PAREN)?.into());
        self.parse_trivias(&mut children);
        let keyword = self.lexer.next(KEYWORD)?;
        let keyword_text = keyword.text;
        children.push(keyword.into());

        match keyword_text {
            "func" => self.parse_module_field_func(children),
            "type" => self.parse_type_def(children),
            "export" => self.parse_module_field_export(children),
            "import" => self.parse_module_field_import(children),
            "start" => self.parse_module_field_start(children),
            "data" => self.parse_module_field_data(children),
            "table" => self.parse_module_field_table(children),
            "global" => self.parse_module_field_global(children),
            "rec" => self.parse_rec_type(children),
            _ => None,
        }
    }

    fn parse_module_field_data(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        self.eat(IDENT, &mut children);
        if let Some((trivias, mem_use)) = self.try_parse_with_trivias(Self::parse_mem_use) {
            children.extend(trivias);
            children.push(mem_use.into());
        }
        if let Some((trivias, offset)) = self.try_parse_with_trivias(Self::parse_offset) {
            children.extend(trivias);
            children.push(offset.into());
        }
        while self.eat(STRING, &mut children) {}
        self.expect_right_paren(&mut children);
        Some(node(MODULE_FIELD_DATA, children))
    }

    fn parse_module_field_export(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        if let Some(name) = self.parse_name() {
            children.push(name.into());
        }
        if !self.recover(Self::parse_export_desc, &mut children) {
            self.report_missing(Message::Name("export descriptor"));
        }
        self.expect_right_paren(&mut children);
        Some(node(MODULE_FIELD_EXPORT, children))
    }

    fn parse_module_field_func(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        self.eat(IDENT, &mut children);

        if let Some((trivias, import)) = self.try_parse_with_trivias(Self::parse_import) {
            children.extend(trivias);
            children.push(import.into());
        } else if let Some((trivias, export)) = self.try_parse_with_trivias(Self::parse_export) {
            children.extend(trivias);
            children.push(export.into());
        }

        if let Some((trivias, type_use)) = self.try_parse_with_trivias(Self::parse_type_use) {
            children.extend(trivias);
            children.push(type_use.into());
        }
        while let Some((trivias, node)) = self.try_parse_with_trivias(Self::parse_local) {
            children.extend(trivias);
            children.push(node.into());
        }

        while self.recover(Self::parse_instr, &mut children) {}
        self.expect_right_paren(&mut children);
        Some(node(MODULE_FIELD_FUNC, children))
    }

    fn parse_module_field_global(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        self.eat(IDENT, &mut children);

        if let Some((trivias, import)) = self.try_parse_with_trivias(Self::parse_import) {
            children.extend(trivias);
            children.push(import.into());
        } else if let Some((trivias, export)) = self.try_parse_with_trivias(Self::parse_export) {
            children.extend(trivias);
            children.push(export.into());
        }

        if !self.recover(Self::parse_global_type, &mut children) {
            self.report_missing(Message::Name("global type"));
        }

        while self.recover(Self::parse_instr, &mut children) {}
        self.expect_right_paren(&mut children);
        Some(node(MODULE_FIELD_GLOBAL, children))
    }

    fn parse_module_field_import(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        if let Some(module_name) = self.parse_module_name() {
            children.push(module_name.into());
        }
        if let Some(name) = self.parse_name() {
            children.push(name.into());
        }
        if !self.recover(Self::parse_import_desc, &mut children) {
            self.report_missing(Message::Name("import descriptor"));
        }
        self.expect_right_paren(&mut children);
        Some(node(MODULE_FIELD_IMPORT, children))
    }

    fn parse_module_field_start(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        if !self.recover(Self::parse_index, &mut children) {
            self.report_missing(Message::Name("index"));
        }
        self.expect_right_paren(&mut children);
        Some(node(MODULE_FIELD_START, children))
    }

    fn parse_module_field_table(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        self.eat(IDENT, &mut children);

        if let Some((trivias, import)) = self.try_parse_with_trivias(Self::parse_import) {
            children.extend(trivias);
            children.push(import.into());
        } else if let Some((trivias, export)) = self.try_parse_with_trivias(Self::parse_export) {
            children.extend(trivias);
            children.push(export.into());
        }

        if self.lexer.peek(UNSIGNED_INT).is_some() {
            if !self.recover(Self::parse_table_type, &mut children) {
                self.report_missing(Message::Name("table type"));
            }
            while self.recover(Self::parse_instr, &mut children) {}
        } else {
            if !self.recover(Self::parse_ref_type, &mut children) {
                self.report_missing(Message::Name("ref type"));
            }
            if !self.recover(Self::parse_elem, &mut children) {
                self.report_missing(Message::Name("elem"));
            }
        }

        self.expect_right_paren(&mut children);
        Some(node(MODULE_FIELD_TABLE, children))
    }

    fn parse_module_name(&mut self) -> Option<GreenNode> {
        self.expect(STRING)
            .map(|token| node(MODULE_NAME, [token.into()]))
    }

    fn parse_name(&mut self) -> Option<GreenNode> {
        self.expect(STRING).map(|token| node(NAME, [token.into()]))
    }

    fn parse_offset(&mut self) -> Option<GreenNode> {
        if let Some(mut children) = self.try_parse(|parser| {
            let mut children = Vec::with_capacity(5);
            children.push(parser.lexer.next(L_PAREN)?.into());
            parser.parse_trivias(&mut children);
            children.push(parser.parse_keyword("offset")?);
            Some(children)
        }) {
            while self.recover(Self::parse_instr, &mut children) {}
            self.expect_right_paren(&mut children);
            Some(node(OFFSET, children))
        } else if self.lexer.peek(L_PAREN).is_some() {
            self.parse_instr().map(|instr| node(OFFSET, [instr.into()]))
        } else {
            None
        }
    }

    fn parse_rec_type(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        while self.recover(Self::parse_type_def_in_rec_type, &mut children) {}
        self.expect_right_paren(&mut children);
        Some(node(REF_TYPE, children))
    }

    fn parse_type_def(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        self.eat(IDENT, &mut children);
        if !self.recover(Self::parse_sub_type, &mut children) {
            self.report_missing(Message::Name("sub type"));
        }
        self.expect_right_paren(&mut children);
        Some(node(TYPE_DEF, children))
    }

    fn parse_type_def_in_rec_type(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(5);
        children.push(self.lexer.next(L_PAREN)?.into());
        self.parse_trivias(&mut children);
        children.push(self.parse_keyword("type")?);
        self.parse_type_def(children)
    }

    pub(super) fn parse_type_use(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(1);
        if let Some(mut node_or_tokens) = self.try_parse(|parser| {
            let mut children = Vec::with_capacity(2);
            children.push(parser.lexer.next(L_PAREN)?.into());
            parser.parse_trivias(&mut children);
            children.push(parser.parse_keyword("type")?);

            if !parser.recover(Self::parse_index, &mut children) {
                parser.report_missing(Message::Name("index"));
            }
            parser.expect_right_paren(&mut children);
            Some(children)
        }) {
            children.append(&mut node_or_tokens);
        }

        while let Some((trivias, node)) = self.try_parse_with_trivias(Self::parse_param) {
            children.extend(trivias);
            children.push(node.into());
        }
        while let Some((trivias, node)) = self.try_parse_with_trivias(Self::parse_result) {
            children.extend(trivias);
            children.push(node.into());
        }

        if children.iter().any(|node_or_token| match node_or_token {
            NodeOrToken::Node(..) => true,
            NodeOrToken::Token(token) => !matches!(
                wat_syntax::WatLanguage::kind_from_raw(token.kind()),
                WHITESPACE | LINE_COMMENT | BLOCK_COMMENT | ERROR
            ),
        }) {
            Some(node(TYPE_USE, children))
        } else {
            None
        }
    }
}
