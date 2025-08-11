use super::{green, node, GreenElement, Parser};
use crate::error::Message;
use rowan::{GreenNode, Language, NodeOrToken};
use wat_syntax::SyntaxKind::*;

impl Parser<'_> {
    fn parse_data(&mut self) -> Option<GreenNode> {
        self.lexer.next(L_PAREN)?;
        let mut children = vec![green::L_PAREN.clone()];
        self.parse_trivias(&mut children);
        self.lexer.keyword("data")?;
        children.push(green::KW_DATA.clone());
        while self.eat(STRING, &mut children) {}
        self.expect_right_paren(&mut children);
        Some(node(DATA, children))
    }

    fn parse_elem(&mut self) -> Option<GreenNode> {
        self.lexer.next(L_PAREN)?;
        let mut children = vec![green::L_PAREN.clone()];
        self.parse_trivias(&mut children);
        self.lexer.keyword("elem")?;
        children.push(green::KW_ELEM.clone());

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
            let mut children = Vec::with_capacity(2);
            parser.lexer.next(L_PAREN)?;
            children.push(green::L_PAREN.clone());
            parser.parse_trivias(&mut children);
            children.push(parser.lexer.keyword("item")?.into());
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

    fn parse_elem_list(&mut self) -> Option<GreenNode> {
        if let Some(node_or_token) = self
            .lexer
            .keyword("func")
            .map(GreenElement::from)
            .or_else(|| self.parse_index().map(GreenElement::from))
        {
            let mut children = vec![node_or_token];
            while self.recover(Self::parse_index, &mut children) {}
            Some(node(ELEM_LIST, children))
        } else {
            let mut children = vec![self.parse_ref_type()?.into()];
            while self.recover(Self::parse_elem_expr, &mut children) {}
            Some(node(ELEM_LIST, children))
        }
    }

    fn parse_export(&mut self, mut children: Vec<GreenElement>) -> GreenNode {
        if !self.retry(Self::parse_name, &mut children) {
            self.report_missing(Message::Name("export name"));
        }
        self.expect_right_paren(&mut children);
        node(EXPORT, children)
    }

    fn parse_export_desc(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(3);
        self.lexer.next(L_PAREN)?;
        children.push(green::L_PAREN.clone());
        self.parse_trivias(&mut children);
        let kind = match self.lexer.next(KEYWORD)?.text {
            "func" => {
                children.push(green::KW_FUNC.clone());
                EXPORT_DESC_FUNC
            }
            "table" => {
                children.push(green::KW_TABLE.clone());
                EXPORT_DESC_TABLE
            }
            "memory" => {
                children.push(green::KW_MEMORY.clone());
                EXPORT_DESC_MEMORY
            }
            "global" => {
                children.push(green::KW_GLOBAL.clone());
                EXPORT_DESC_GLOBAL
            }
            _ => return None,
        };

        if !self.recover(Self::parse_index, &mut children) {
            self.report_missing(Message::Name("index"));
        }
        self.expect_right_paren(&mut children);
        Some(node(kind, children))
    }

    fn parse_import(&mut self, mut children: Vec<GreenElement>) -> GreenNode {
        if !self.retry(Self::parse_module_name, &mut children) {
            self.report_missing(Message::Name("import module name"));
        }
        if !self.retry(Self::parse_name, &mut children) {
            self.report_missing(Message::Name("import name"));
        }
        self.expect_right_paren(&mut children);
        node(IMPORT, children)
    }

    fn parse_imports_and_exports(&mut self, children: &mut Vec<GreenElement>) {
        loop {
            if let Some((trivias, (node_or_tokens, is_import))) =
                self.try_parse_with_trivias(|parser| {
                    let mut children = Vec::with_capacity(2);
                    parser.lexer.next(L_PAREN)?;
                    children.push(green::L_PAREN.clone());
                    parser.parse_trivias(&mut children);
                    let keyword = parser.lexer.next(KEYWORD)?;
                    let is_import = keyword.text == "import";
                    if is_import || keyword.text == "export" {
                        children.push(keyword.into());
                        Some((children, is_import))
                    } else {
                        None
                    }
                })
            {
                children.extend(trivias);
                if is_import {
                    children.push(self.parse_import(node_or_tokens).into());
                } else {
                    children.push(self.parse_export(node_or_tokens).into());
                }
            } else {
                break;
            }
        }
    }

    fn parse_import_desc(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(5);
        self.lexer.next(L_PAREN)?;
        children.push(green::L_PAREN.clone());
        self.parse_trivias(&mut children);
        match self.lexer.next(KEYWORD)?.text {
            "func" => {
                children.push(green::KW_FUNC.clone());
                self.eat(IDENT, &mut children);
                if let Some((trivias, type_use)) = self.try_parse_with_trivias(Self::parse_type_use)
                {
                    children.extend(trivias);
                    children.push(type_use.into());
                }
                self.expect_right_paren(&mut children);
                Some(node(IMPORT_DESC_TYPE_USE, children))
            }
            "global" => {
                children.push(green::KW_GLOBAL.clone());
                self.eat(IDENT, &mut children);
                if !self.recover(Self::parse_global_type, &mut children) {
                    self.report_missing(Message::Name("global type"));
                }
                self.expect_right_paren(&mut children);
                Some(node(IMPORT_DESC_GLOBAL_TYPE, children))
            }
            "memory" => {
                children.push(green::KW_MEMORY.clone());
                self.eat(IDENT, &mut children);
                if !self.recover(Self::parse_memory_type, &mut children) {
                    self.report_missing(Message::Name("memory type"));
                }
                self.expect_right_paren(&mut children);
                Some(node(IMPORT_DESC_MEMORY_TYPE, children))
            }
            "table" => {
                children.push(green::KW_TABLE.clone());
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
        self.lexer.next(L_PAREN)?;
        children.push(green::L_PAREN.clone());
        self.parse_trivias(&mut children);
        self.lexer.keyword("local")?;
        children.push(green::KW_LOCAL.clone());

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
        self.lexer.next(L_PAREN)?;
        children.push(green::L_PAREN.clone());
        self.parse_trivias(&mut children);
        self.lexer.keyword("memory")?;
        children.push(green::KW_MEMORY.clone());
        if !self.recover(Self::parse_index, &mut children) {
            self.report_missing(Message::Name("index"));
        }
        self.expect_right_paren(&mut children);
        Some(node(MEM_USE, children))
    }

    pub(super) fn parse_module(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(5);
        self.lexer.next(L_PAREN)?;
        children.push(green::L_PAREN.clone());
        self.parse_trivias(&mut children);
        let keyword = self.lexer.next(KEYWORD)?;

        self.lexer.top_level = false;
        let node = match keyword.text {
            "module" => {
                children.push(keyword.into());
                self.eat(IDENT, &mut children);
                while self.recover(Self::parse_module_field, &mut children) {}
                self.expect_right_paren(&mut children);
                Some(node(MODULE, children))
            }
            // wabt allows top-level module fields
            "func" => {
                children.push(green::KW_FUNC.clone());
                let mut children = vec![self.parse_module_field_func(children)?.into()];
                while self.recover(Self::parse_module_field, &mut children) {}
                Some(node(MODULE, children))
            }
            "type" => {
                children.push(green::KW_TYPE.clone());
                let mut children = vec![self.parse_type_def(children)?.into()];
                while self.recover(Self::parse_module_field, &mut children) {}
                Some(node(MODULE, children))
            }
            "global" => {
                children.push(green::KW_GLOBAL.clone());
                let mut children = vec![self.parse_module_field_global(children)?.into()];
                while self.recover(Self::parse_module_field, &mut children) {}
                Some(node(MODULE, children))
            }
            "export" => {
                children.push(green::KW_EXPORT.clone());
                let mut children = vec![self.parse_module_field_export(children)?.into()];
                while self.recover(Self::parse_module_field, &mut children) {}
                Some(node(MODULE, children))
            }
            "import" => {
                children.push(green::KW_IMPORT.clone());
                let mut children = vec![self.parse_module_field_import(children)?.into()];
                while self.recover(Self::parse_module_field, &mut children) {}
                Some(node(MODULE, children))
            }
            "start" => {
                children.push(keyword.into());
                let mut children = vec![self.parse_module_field_start(children)?.into()];
                while self.recover(Self::parse_module_field, &mut children) {}
                Some(node(MODULE, children))
            }
            "data" => {
                children.push(green::KW_DATA.clone());
                let mut children = vec![self.parse_module_field_data(children)?.into()];
                while self.recover(Self::parse_module_field, &mut children) {}
                Some(node(MODULE, children))
            }
            "table" => {
                children.push(green::KW_TABLE.clone());
                let mut children = vec![self.parse_module_field_table(children)?.into()];
                while self.recover(Self::parse_module_field, &mut children) {}
                Some(node(MODULE, children))
            }
            "memory" => {
                children.push(green::KW_MEMORY.clone());
                let mut children = vec![self.parse_module_field_memory(children)?.into()];
                while self.recover(Self::parse_module_field, &mut children) {}
                Some(node(MODULE, children))
            }
            "elem" => {
                children.push(green::KW_ELEM.clone());
                let mut children = vec![self.parse_module_field_elem(children)?.into()];
                while self.recover(Self::parse_module_field, &mut children) {}
                Some(node(MODULE, children))
            }
            "rec" => {
                children.push(green::KW_REC.clone());
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
        self.lexer.next(L_PAREN)?;
        children.push(green::L_PAREN.clone());
        self.parse_trivias(&mut children);
        let keyword = self.lexer.next(KEYWORD)?;
        match keyword.text {
            "func" => {
                children.push(green::KW_FUNC.clone());
                self.parse_module_field_func(children)
            }
            "type" => {
                children.push(green::KW_TYPE.clone());
                self.parse_type_def(children)
            }
            "global" => {
                children.push(green::KW_GLOBAL.clone());
                self.parse_module_field_global(children)
            }
            "export" => {
                children.push(green::KW_EXPORT.clone());
                self.parse_module_field_export(children)
            }
            "import" => {
                children.push(green::KW_IMPORT.clone());
                self.parse_module_field_import(children)
            }
            "start" => {
                children.push(keyword.into());
                self.parse_module_field_start(children)
            }
            "data" => {
                children.push(green::KW_DATA.clone());
                self.parse_module_field_data(children)
            }
            "table" => {
                children.push(green::KW_TABLE.clone());
                self.parse_module_field_table(children)
            }
            "memory" => {
                children.push(green::KW_MEMORY.clone());
                self.parse_module_field_memory(children)
            }
            "elem" => {
                children.push(green::KW_ELEM.clone());
                self.parse_module_field_elem(children)
            }
            "rec" => {
                children.push(green::KW_REC.clone());
                self.parse_rec_type(children)
            }
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

    fn parse_module_field_elem(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        self.eat(IDENT, &mut children);
        if let Some((trivias, keyword)) =
            self.try_parse_with_trivias(|parser| parser.lexer.keyword("declare"))
        {
            children.extend(trivias);
            children.push(keyword.into());
            if !self.recover(Self::parse_elem_list, &mut children) {
                self.report_missing(Message::Name("elem list"));
            }
        } else if let Some((trivias, elem_list)) =
            self.try_parse_with_trivias(Self::parse_elem_list)
        {
            children.extend(trivias);
            children.push(elem_list.into());
        } else {
            if let Some((trivias, table_use)) = self.try_parse_with_trivias(Self::parse_table_use) {
                children.extend(trivias);
                children.push(table_use.into());
            }
            if !self.recover(Self::parse_offset, &mut children) {
                self.report_missing(Message::Name("offset"));
            }
            if let Some((trivias, elem_list)) = self.try_parse_with_trivias(Self::parse_elem_list) {
                children.extend(trivias);
                children.push(elem_list.into());
            }
        }
        self.expect_right_paren(&mut children);
        Some(node(MODULE_FIELD_ELEM, children))
    }

    fn parse_module_field_export(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        if !self.retry(Self::parse_name, &mut children) {
            self.report_missing(Message::Name("export name"));
        }
        if !self.retry(Self::parse_export_desc, &mut children) {
            self.report_missing(Message::Name("export descriptor"));
        }
        self.expect_right_paren(&mut children);
        Some(node(MODULE_FIELD_EXPORT, children))
    }

    fn parse_module_field_func(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        self.eat(IDENT, &mut children);
        self.parse_imports_and_exports(&mut children);

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
        self.parse_imports_and_exports(&mut children);

        if !self.recover(Self::parse_global_type, &mut children) {
            self.report_missing(Message::Name("global type"));
        }

        while self.recover(Self::parse_instr, &mut children) {}
        self.expect_right_paren(&mut children);
        Some(node(MODULE_FIELD_GLOBAL, children))
    }

    fn parse_module_field_import(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        if !self.retry(Self::parse_module_name, &mut children) {
            self.report_missing(Message::Name("import module name"));
        }
        if !self.retry(Self::parse_name, &mut children) {
            self.report_missing(Message::Name("import name"));
        }
        if !self.retry(Self::parse_import_desc, &mut children) {
            self.report_missing(Message::Name("import descriptor"));
        }
        self.expect_right_paren(&mut children);
        Some(node(MODULE_FIELD_IMPORT, children))
    }

    fn parse_module_field_memory(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        self.eat(IDENT, &mut children);
        self.parse_imports_and_exports(&mut children);

        if self.lexer.peek(L_PAREN).is_some() {
            if !self.recover(Self::parse_data, &mut children) {
                self.report_missing(Message::Name("data"));
            }
        } else if !self.recover(Self::parse_memory_type, &mut children) {
            self.report_missing(Message::Name("memory type"));
        }

        self.expect_right_paren(&mut children);
        Some(node(MODULE_FIELD_MEMORY, children))
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
        self.parse_imports_and_exports(&mut children);

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
            parser.lexer.next(L_PAREN)?;
            children.push(green::L_PAREN.clone());
            parser.parse_trivias(&mut children);
            children.push(parser.lexer.keyword("offset")?.into());
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
        Some(node(REC_TYPE, children))
    }

    fn parse_table_use(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(5);
        self.lexer.next(L_PAREN)?;
        children.push(green::L_PAREN.clone());
        self.parse_trivias(&mut children);
        self.lexer.keyword("table")?;
        children.push(green::KW_TABLE.clone());

        if !self.recover(Self::parse_index, &mut children) {
            self.report_missing(Message::Name("index"));
        }
        self.expect_right_paren(&mut children);
        Some(node(TABLE_USE, children))
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
        self.lexer.next(L_PAREN)?;
        children.push(green::L_PAREN.clone());
        self.parse_trivias(&mut children);
        self.lexer.keyword("type")?;
        children.push(green::KW_TYPE.clone());
        self.parse_type_def(children)
    }

    pub(super) fn parse_type_use(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(1);
        if let Some(mut node_or_tokens) = self.try_parse(|parser| {
            let mut children = Vec::with_capacity(2);
            parser.lexer.next(L_PAREN)?;
            children.push(green::L_PAREN.clone());
            parser.parse_trivias(&mut children);
            parser.lexer.keyword("type")?;
            children.push(green::KW_TYPE.clone());

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
