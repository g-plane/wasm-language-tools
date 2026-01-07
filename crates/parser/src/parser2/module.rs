use super::{GreenElement, Parser, builder::NodeMark, green, node};
use crate::error::Message;
use rowan::GreenNode;
use wat_syntax::SyntaxKind::*;

impl Parser<'_> {
    fn parse_data(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        self.lexer.next(L_PAREN)?;
        self.add_child(green::L_PAREN.clone());
        self.parse_trivias();
        self.lexer.keyword("data")?;
        self.add_child(green::KW_DATA.clone());
        while self.eat(STRING) {}
        self.expect_right_paren();
        Some(self.finish_node(DATA, mark))
    }

    fn parse_elem(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        self.lexer.next(L_PAREN)?;
        self.add_child(green::L_PAREN.clone());
        self.parse_trivias();
        self.lexer.keyword("elem")?;
        self.add_child(green::KW_ELEM.clone());

        if self.lexer.peek(L_PAREN).is_some() {
            while self.recover(Self::parse_elem_expr) {}
        } else {
            while self.recover(Self::parse_index) {}
        }
        self.expect_right_paren();
        Some(self.finish_node(ELEM, mark))
    }

    fn parse_elem_expr(&mut self) -> Option<GreenNode> {
        if let Some(mark) = self.try_parse(|parser| {
            let mark = parser.start_node();
            parser.lexer.next(L_PAREN)?;
            parser.add_child(green::L_PAREN.clone());
            parser.parse_trivias();
            parser.lexer.keyword("item")?;
            parser.add_child(green::KW_ITEM.clone());
            Some(mark)
        }) {
            while self.recover(Self::parse_instr) {}
            self.expect_right_paren();
            Some(self.finish_node(ELEM_EXPR, mark))
        } else if self.lexer.peek(L_PAREN).is_some() {
            self.parse_instr()
                .map(|instr| node(ELEM_EXPR, [instr.into()]))
        } else {
            None
        }
    }

    fn parse_elem_list(&mut self) -> Option<GreenNode> {
        if let Some(node_or_token) = self
            .lexer
            .keyword("func")
            .map(GreenElement::from)
            .or_else(|| self.parse_index().map(GreenElement::from))
        {
            let mark = self.start_node();
            self.add_child(node_or_token);
            while self.recover(Self::parse_index) {}
            Some(self.finish_node(ELEM_LIST, mark))
        } else {
            let mark = self.start_node();
            let ref_type = self.parse_ref_type()?;
            self.add_child(ref_type);
            while self.recover(Self::parse_elem_expr) {}
            Some(self.finish_node(ELEM_LIST, mark))
        }
    }

    fn parse_export(&mut self, mark: NodeMark) -> GreenNode {
        if !self.retry(Self::parse_name) {
            self.report_missing(Message::Name("export name"));
        }
        self.expect_right_paren();
        self.finish_node(EXPORT, mark)
    }

    fn parse_extern_idx(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        self.lexer.next(L_PAREN)?;
        self.add_child(green::L_PAREN.clone());
        self.parse_trivias();
        let kind = match self.lexer.next(KEYWORD)?.text {
            "func" => {
                self.add_child(green::KW_FUNC.clone());
                EXTERN_IDX_FUNC
            }
            "table" => {
                self.add_child(green::KW_TABLE.clone());
                EXTERN_IDX_TABLE
            }
            "memory" => {
                self.add_child(green::KW_MEMORY.clone());
                EXTERN_IDX_MEMORY
            }
            "global" => {
                self.add_child(green::KW_GLOBAL.clone());
                EXTERN_IDX_GLOBAL
            }
            "tag" => {
                self.add_child(green::KW_TAG.clone());
                EXTERN_IDX_TAG
            }
            _ => return None,
        };

        if !self.recover(Self::parse_index) {
            self.report_missing(Message::Name("index"));
        }
        self.expect_right_paren();
        Some(self.finish_node(kind, mark))
    }

    fn parse_import(&mut self, mark: NodeMark) -> GreenNode {
        if !self.retry(Self::parse_module_name) {
            self.report_missing(Message::Name("import module name"));
        }
        if !self.retry(Self::parse_name) {
            self.report_missing(Message::Name("import name"));
        }
        self.expect_right_paren();
        self.finish_node(IMPORT, mark)
    }

    fn parse_exports_and_import(&mut self) {
        let mut has_import = false;
        while let Some((mark, is_import)) = self.try_parse_with_trivias(|parser| {
            let mark = parser.start_node();
            parser.lexer.next(L_PAREN)?;
            parser.add_child(green::L_PAREN.clone());
            parser.parse_trivias();
            let keyword = parser.lexer.next(KEYWORD)?;
            match keyword.text {
                "import" => {
                    if has_import {
                        parser.report_error_token(
                            &keyword,
                            Message::Description("only one import is allowed"),
                        );
                        None
                    } else {
                        has_import = true;
                        Some((mark, true))
                    }
                }
                "export" => {
                    if has_import {
                        parser.report_error_token(
                            &keyword,
                            Message::Description("export must come before import"),
                        );
                        None
                    } else {
                        Some((mark, false))
                    }
                }
                _ => None,
            }
        }) {
            if is_import {
                self.add_child(green::KW_IMPORT.clone());
                let import = self.parse_import(mark);
                self.add_child(import);
            } else {
                self.add_child(green::KW_EXPORT.clone());
                let export = self.parse_export(mark);
                self.add_child(export);
            }
        }
    }

    pub(super) fn parse_index(&mut self) -> Option<GreenNode> {
        self.lexer
            .eat(IDENT)
            .or_else(|| self.lexer.eat(UNSIGNED_INT))
            .map(|token| node(INDEX, [token.into()]))
    }

    fn parse_local(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        self.lexer.next(L_PAREN)?;
        self.add_child(green::L_PAREN.clone());
        self.parse_trivias();
        self.lexer.keyword("local")?;
        self.add_child(green::KW_LOCAL.clone());

        if self.eat(IDENT) {
            if !self.recover(Self::parse_value_type) {
                self.report_missing(Message::Name("value type"));
            }
        } else {
            while self.recover(Self::parse_value_type) {}
        }
        self.expect_right_paren();
        Some(self.finish_node(LOCAL, mark))
    }

    fn parse_mem_use(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        self.lexer.next(L_PAREN)?;
        self.add_child(green::L_PAREN.clone());
        self.parse_trivias();
        self.lexer.keyword("memory")?;
        self.add_child(green::KW_MEMORY.clone());
        if !self.recover(Self::parse_index) {
            self.report_missing(Message::Name("index"));
        }
        self.expect_right_paren();
        Some(self.finish_node(MEM_USE, mark))
    }

    pub(super) fn parse_module(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        self.lexer.next(L_PAREN)?;
        self.add_child(green::L_PAREN.clone());
        self.parse_trivias();
        let keyword = self.lexer.next(KEYWORD)?;

        self.lexer.top_level = false;
        let node = match keyword.text {
            "module" => {
                self.add_child(keyword);
                self.eat(IDENT);
                while self.recover(Self::parse_module_field) {}
                self.expect_right_paren();
                Some(self.finish_node(MODULE, mark))
            }
            // wabt allows top-level module fields
            "func" => {
                self.add_child(green::KW_FUNC.clone());
                let module_field = self.parse_module_field_func(mark);
                let mark = self.start_node();
                self.add_child(module_field);
                while self.recover(Self::parse_module_field) {}
                Some(self.finish_node(MODULE, mark))
            }
            "type" => {
                self.add_child(green::KW_TYPE.clone());
                let module_field = self.parse_type_def(mark);
                let mark = self.start_node();
                self.add_child(module_field);
                while self.recover(Self::parse_module_field) {}
                Some(self.finish_node(MODULE, mark))
            }
            "global" => {
                self.add_child(green::KW_GLOBAL.clone());
                let module_field = self.parse_module_field_global(mark);
                let mark = self.start_node();
                self.add_child(module_field);
                while self.recover(Self::parse_module_field) {}
                Some(self.finish_node(MODULE, mark))
            }
            "export" => {
                self.add_child(green::KW_EXPORT.clone());
                let module_field = self.parse_module_field_export(mark);
                let mark = self.start_node();
                self.add_child(module_field);
                while self.recover(Self::parse_module_field) {}
                Some(self.finish_node(MODULE, mark))
            }
            "import" => {
                self.add_child(green::KW_IMPORT.clone());
                let module_field = self.parse_module_field_import(mark);
                let mark = self.start_node();
                self.add_child(module_field);
                while self.recover(Self::parse_module_field) {}
                Some(self.finish_node(MODULE, mark))
            }
            "start" => {
                self.add_child(keyword);
                let module_field = self.parse_module_field_start(mark);
                let mark = self.start_node();
                self.add_child(module_field);
                while self.recover(Self::parse_module_field) {}
                Some(self.finish_node(MODULE, mark))
            }
            "data" => {
                self.add_child(green::KW_DATA.clone());
                let module_field = self.parse_module_field_data(mark);
                let mark = self.start_node();
                self.add_child(module_field);
                while self.recover(Self::parse_module_field) {}
                Some(self.finish_node(MODULE, mark))
            }
            "table" => {
                self.add_child(green::KW_TABLE.clone());
                let module_field = self.parse_module_field_table(mark);
                let mark = self.start_node();
                self.add_child(module_field);
                while self.recover(Self::parse_module_field) {}
                Some(self.finish_node(MODULE, mark))
            }
            "memory" => {
                self.add_child(green::KW_MEMORY.clone());
                let module_field = self.parse_module_field_memory(mark);
                let mark = self.start_node();
                self.add_child(module_field);
                while self.recover(Self::parse_module_field) {}
                Some(self.finish_node(MODULE, mark))
            }
            "elem" => {
                self.add_child(green::KW_ELEM.clone());
                let module_field = self.parse_module_field_elem(mark);
                let mark = self.start_node();
                self.add_child(module_field);
                while self.recover(Self::parse_module_field) {}
                Some(self.finish_node(MODULE, mark))
            }
            "rec" => {
                self.add_child(green::KW_REC.clone());
                let module_field = self.parse_rec_type(mark);
                let mark = self.start_node();
                self.add_child(module_field);
                while self.recover(Self::parse_module_field) {}
                Some(self.finish_node(MODULE, mark))
            }
            "tag" => {
                self.add_child(green::KW_TAG.clone());
                let module_field = self.parse_module_field_tag(mark);
                let mark = self.start_node();
                self.add_child(module_field);
                while self.recover(Self::parse_module_field) {}
                Some(self.finish_node(MODULE, mark))
            }
            _ => None,
        };
        self.lexer.top_level = true;
        node
    }

    pub(crate) fn parse_module_field(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        self.lexer.next(L_PAREN)?;
        self.add_child(green::L_PAREN.clone());
        self.parse_trivias();
        let keyword = self.lexer.next(KEYWORD)?;
        match keyword.text {
            "func" => {
                self.add_child(green::KW_FUNC.clone());
                Some(self.parse_module_field_func(mark))
            }
            "type" => {
                self.add_child(green::KW_TYPE.clone());
                Some(self.parse_type_def(mark))
            }
            "global" => {
                self.add_child(green::KW_GLOBAL.clone());
                Some(self.parse_module_field_global(mark))
            }
            "export" => {
                self.add_child(green::KW_EXPORT.clone());
                Some(self.parse_module_field_export(mark))
            }
            "import" => {
                self.add_child(green::KW_IMPORT.clone());
                Some(self.parse_module_field_import(mark))
            }
            "start" => {
                self.add_child(keyword);
                Some(self.parse_module_field_start(mark))
            }
            "data" => {
                self.add_child(green::KW_DATA.clone());
                Some(self.parse_module_field_data(mark))
            }
            "table" => {
                self.add_child(green::KW_TABLE.clone());
                Some(self.parse_module_field_table(mark))
            }
            "memory" => {
                self.add_child(green::KW_MEMORY.clone());
                Some(self.parse_module_field_memory(mark))
            }
            "elem" => {
                self.add_child(green::KW_ELEM.clone());
                Some(self.parse_module_field_elem(mark))
            }
            "rec" => {
                self.add_child(green::KW_REC.clone());
                Some(self.parse_rec_type(mark))
            }
            "tag" => {
                self.add_child(green::KW_TAG.clone());
                Some(self.parse_module_field_tag(mark))
            }
            _ => None,
        }
    }

    fn parse_module_field_data(&mut self, mark: NodeMark) -> GreenNode {
        self.eat(IDENT);
        if let Some(mem_use) = self.try_parse_with_trivias(Self::parse_mem_use) {
            self.add_child(mem_use);
        }
        if let Some(offset) = self.try_parse_with_trivias(Self::parse_offset) {
            self.add_child(offset);
        }
        while self.eat(STRING) {}
        self.expect_right_paren();
        self.finish_node(MODULE_FIELD_DATA, mark)
    }

    fn parse_module_field_elem(&mut self, mark: NodeMark) -> GreenNode {
        self.eat(IDENT);
        if let Some(keyword) = self.try_parse_with_trivias(|parser| parser.lexer.keyword("declare"))
        {
            self.add_child(keyword);
            if !self.recover(Self::parse_elem_list) {
                self.report_missing(Message::Name("elem list"));
            }
        } else if let Some(elem_list) = self.try_parse_with_trivias(Self::parse_elem_list) {
            self.add_child(elem_list);
        } else {
            if let Some(table_use) = self.try_parse_with_trivias(Self::parse_table_use) {
                self.add_child(table_use);
            }
            if !self.recover(Self::parse_offset) {
                self.report_missing(Message::Name("offset"));
            }
            if let Some(elem_list) = self.try_parse_with_trivias(Self::parse_elem_list) {
                self.add_child(elem_list);
            }
        }
        self.expect_right_paren();
        self.finish_node(MODULE_FIELD_ELEM, mark)
    }

    fn parse_module_field_export(&mut self, mark: NodeMark) -> GreenNode {
        if !self.retry(Self::parse_name) {
            self.report_missing(Message::Name("export name"));
        }
        if !self.retry(Self::parse_extern_idx) {
            self.report_missing(Message::Name("export idx"));
        }
        self.expect_right_paren();
        self.finish_node(MODULE_FIELD_EXPORT, mark)
    }

    fn parse_module_field_func(&mut self, mark: NodeMark) -> GreenNode {
        self.eat(IDENT);
        self.parse_exports_and_import();

        if let Some(type_use) = self.try_parse_with_trivias(Self::parse_type_use) {
            self.add_child(type_use);
        }
        while let Some(local) = self.try_parse_with_trivias(Self::parse_local) {
            self.add_child(local);
        }

        while self.recover(Self::parse_instr) {}
        self.expect_right_paren();
        self.finish_node(MODULE_FIELD_FUNC, mark)
    }

    fn parse_module_field_global(&mut self, mark: NodeMark) -> GreenNode {
        self.eat(IDENT);
        self.parse_exports_and_import();

        if !self.recover(Self::parse_global_type) {
            self.report_missing(Message::Name("global type"));
        }

        while self.recover(Self::parse_instr) {}
        self.expect_right_paren();
        self.finish_node(MODULE_FIELD_GLOBAL, mark)
    }

    fn parse_module_field_import(&mut self, mark: NodeMark) -> GreenNode {
        if !self.retry(Self::parse_module_name) {
            self.report_missing(Message::Name("import module name"));
        }
        if !self.retry(Self::parse_name) {
            self.report_missing(Message::Name("import name"));
        }
        if !self.retry(Self::parse_extern_type) {
            self.report_missing(Message::Name("extern type"));
        }
        self.expect_right_paren();
        self.finish_node(MODULE_FIELD_IMPORT, mark)
    }

    fn parse_module_field_memory(&mut self, mark: NodeMark) -> GreenNode {
        self.eat(IDENT);
        self.parse_exports_and_import();

        if self.lexer.peek(L_PAREN).is_some() {
            if !self.recover(Self::parse_data) {
                self.report_missing(Message::Name("data"));
            }
        } else if !self.recover(Self::parse_memory_type) {
            self.report_missing(Message::Name("memory type"));
        }

        self.expect_right_paren();
        self.finish_node(MODULE_FIELD_MEMORY, mark)
    }

    fn parse_module_field_start(&mut self, mark: NodeMark) -> GreenNode {
        if !self.recover(Self::parse_index) {
            self.report_missing(Message::Name("index"));
        }
        self.expect_right_paren();
        self.finish_node(MODULE_FIELD_START, mark)
    }

    fn parse_module_field_table(&mut self, mark: NodeMark) -> GreenNode {
        self.eat(IDENT);
        self.parse_exports_and_import();

        if self.lexer.peek(UNSIGNED_INT).is_some()
            || matches!(
                self.lexer.peek(TYPE_KEYWORD),
                Some(super::lexer::Token {
                    text: "i32" | "i64",
                    ..
                })
            )
        {
            if !self.recover(Self::parse_table_type) {
                self.report_missing(Message::Name("table type"));
            }
            while self.recover(Self::parse_instr) {}
        } else {
            if !self.recover(Self::parse_ref_type) {
                self.report_missing(Message::Name("ref type"));
            }
            if !self.recover(Self::parse_elem) {
                self.report_missing(Message::Name("elem"));
            }
        }

        self.expect_right_paren();
        self.finish_node(MODULE_FIELD_TABLE, mark)
    }

    fn parse_module_field_tag(&mut self, mark: NodeMark) -> GreenNode {
        self.eat(IDENT);
        self.parse_exports_and_import();
        if let Some(type_use) = self.try_parse_with_trivias(Self::parse_type_use) {
            self.add_child(type_use);
        }
        self.expect_right_paren();
        self.finish_node(MODULE_FIELD_TAG, mark)
    }

    fn parse_module_name(&mut self) -> Option<GreenNode> {
        self.expect(STRING)
            .map(|token| node(MODULE_NAME, [token.into()]))
    }

    fn parse_name(&mut self) -> Option<GreenNode> {
        self.expect(STRING).map(|token| node(NAME, [token.into()]))
    }

    fn parse_offset(&mut self) -> Option<GreenNode> {
        if let Some(mark) = self.try_parse(|parser| {
            let mark = parser.start_node();
            parser.lexer.next(L_PAREN)?;
            parser.add_child(green::L_PAREN.clone());
            parser.parse_trivias();
            parser.lexer.keyword("offset")?;
            parser.add_child(green::KW_OFFSET.clone());
            Some(mark)
        }) {
            while self.recover(Self::parse_instr) {}
            self.expect_right_paren();
            Some(self.finish_node(OFFSET, mark))
        } else if self.lexer.peek(L_PAREN).is_some() {
            self.parse_instr().map(|instr| node(OFFSET, [instr.into()]))
        } else {
            None
        }
    }

    fn parse_rec_type(&mut self, mark: NodeMark) -> GreenNode {
        while self.recover(Self::parse_type_def_in_rec_type) {}
        self.expect_right_paren();
        self.finish_node(REC_TYPE, mark)
    }

    fn parse_table_use(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        self.lexer.next(L_PAREN)?;
        self.add_child(green::L_PAREN.clone());
        self.parse_trivias();
        self.lexer.keyword("table")?;
        self.add_child(green::KW_TABLE.clone());

        if !self.recover(Self::parse_index) {
            self.report_missing(Message::Name("index"));
        }
        self.expect_right_paren();
        Some(self.finish_node(TABLE_USE, mark))
    }

    fn parse_type_def(&mut self, mark: NodeMark) -> GreenNode {
        self.eat(IDENT);
        if !self.recover(Self::parse_sub_type) {
            self.report_missing(Message::Name("sub type"));
        }
        self.expect_right_paren();
        self.finish_node(TYPE_DEF, mark)
    }

    fn parse_type_def_in_rec_type(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        self.lexer.next(L_PAREN)?;
        self.add_child(green::L_PAREN.clone());
        self.parse_trivias();
        self.lexer.keyword("type")?;
        self.add_child(green::KW_TYPE.clone());
        Some(self.parse_type_def(mark))
    }

    pub(super) fn parse_type_use(&mut self) -> Option<GreenNode> {
        const HAS_TYPE: u8 = 1 << 0;
        const HAS_PARAM: u8 = 1 << 1;
        const HAS_RESULT: u8 = 1 << 2;
        let mut state = 0u8;

        let mark = self.start_node();
        if self
            .try_parse(|parser| {
                parser.lexer.next(L_PAREN)?;
                parser.add_child(green::L_PAREN.clone());
                parser.parse_trivias();
                parser.lexer.keyword("type")?;
                parser.add_child(green::KW_TYPE.clone());
                Some(())
            })
            .is_some()
        {
            state |= HAS_TYPE;
            if !self.recover(Self::parse_index) {
                self.report_missing(Message::Name("index"));
            }
            self.expect_right_paren();
        }

        while let Some(param) = self.try_parse_with_trivias(Self::parse_param) {
            state |= HAS_PARAM;
            self.add_child(param);
        }
        while let Some(result) = self.try_parse_with_trivias(Self::parse_result) {
            state |= HAS_RESULT;
            self.add_child(result);
        }

        if state == 0 {
            None
        } else {
            Some(self.finish_node(TYPE_USE, mark))
        }
    }
}
