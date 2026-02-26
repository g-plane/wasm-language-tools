use super::{GreenElement, Parser, builder::NodeMark, green, node};
use crate::error::Message;
use wat_syntax::{GreenNode, SyntaxKind::*};

impl Parser<'_> {
    fn parse_addr_type(&mut self) -> Option<GreenNode> {
        self.lexer.eat(TYPE_KEYWORD).map(|mut token| {
            let token = match token.text {
                "i32" => green::TYPE_KW_I32.clone(),
                "i64" => green::TYPE_KW_I64.clone(),
                _ => {
                    token.kind = ERROR;
                    self.report_error_token(&token, Message::Description("invalid address type"));
                    token.into()
                }
            };
            node(ADDR_TYPE, [token])
        })
    }

    fn parse_array_type(&mut self, mark: NodeMark) -> Option<GreenNode> {
        if !self.recover(Self::parse_field_type) {
            self.report_missing(Message::Name("field type"));
        }
        self.expect_right_paren();
        Some(self.finish_node(ARRAY_TYPE, mark))
    }

    fn parse_composite_type(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        self.lexer.next(L_PAREN)?;
        self.add_child(green::L_PAREN.clone());
        self.parse_trivias();
        match self.lexer.next(KEYWORD)?.text {
            "func" => {
                self.add_child(green::KW_FUNC.clone());
                self.parse_func_type(mark)
            }
            "struct" => {
                self.add_child(green::KW_STRUCT.clone());
                self.parse_struct_type(mark)
            }
            "array" => {
                self.add_child(green::KW_ARRAY.clone());
                self.parse_array_type(mark)
            }
            "cont" => {
                self.add_child(green::KW_CONT.clone());
                self.parse_cont_type(mark)
            }
            _ => None,
        }
    }

    fn parse_cont_type(&mut self, mark: NodeMark) -> Option<GreenNode> {
        if !self.recover(Self::parse_index) {
            self.report_missing(Message::Name("index"));
        }
        self.expect_right_paren();
        Some(self.finish_node(CONT_TYPE, mark))
    }

    pub(super) fn parse_extern_type(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        self.lexer.next(L_PAREN)?;
        self.add_child(green::L_PAREN.clone());
        self.parse_trivias();
        match self.lexer.next(KEYWORD)?.text {
            "func" => {
                self.add_child(green::KW_FUNC.clone());
                self.eat(IDENT);
                if let Some(type_use) = self.try_parse_with_trivias(Self::parse_type_use) {
                    self.add_child(type_use);
                }
                self.expect_right_paren();
                Some(self.finish_node(EXTERN_TYPE_FUNC, mark))
            }
            "global" => {
                self.add_child(green::KW_GLOBAL.clone());
                self.eat(IDENT);
                if !self.recover(Self::parse_global_type) {
                    self.report_missing(Message::Name("global type"));
                }
                self.expect_right_paren();
                Some(self.finish_node(EXTERN_TYPE_GLOBAL, mark))
            }
            "memory" => {
                self.add_child(green::KW_MEMORY.clone());
                self.eat(IDENT);
                if !self.recover(Self::parse_mem_type) {
                    self.report_missing(Message::Name("memory type"));
                }
                self.expect_right_paren();
                Some(self.finish_node(EXTERN_TYPE_MEMORY, mark))
            }
            "table" => {
                self.add_child(green::KW_TABLE.clone());
                self.eat(IDENT);
                if !self.recover(Self::parse_table_type) {
                    self.report_missing(Message::Name("table type"));
                }
                self.expect_right_paren();
                Some(self.finish_node(EXTERN_TYPE_TABLE, mark))
            }
            "tag" => {
                self.add_child(green::KW_TAG.clone());
                self.eat(IDENT);
                if let Some(type_use) = self.try_parse_with_trivias(Self::parse_type_use) {
                    self.add_child(type_use);
                }
                self.expect_right_paren();
                Some(self.finish_node(EXTERN_TYPE_TAG, mark))
            }
            _ => None,
        }
    }

    fn parse_field(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        self.lexer.next(L_PAREN)?;
        self.add_child(green::L_PAREN.clone());
        self.parse_trivias();
        self.lexer.keyword("field")?;
        self.add_child(green::KW_FIELD.clone());

        if self.eat(IDENT) {
            if !self.recover(Self::parse_field_type) {
                self.report_missing(Message::Name("field type"));
            }
        } else {
            while self.recover(Self::parse_field_type) {}
        }
        self.expect_right_paren();
        Some(self.finish_node(FIELD, mark))
    }

    fn parse_field_type(&mut self) -> Option<GreenNode> {
        if let Some(mark) = self.try_parse(|parser| {
            let mark = parser.start_node();
            parser.lexer.next(L_PAREN)?;
            parser.add_child(green::L_PAREN.clone());
            parser.parse_trivias();
            parser.lexer.keyword("mut")?;
            parser.add_child(green::KW_MUT.clone());
            Some(mark)
        }) {
            if !self.recover(Self::parse_storage_type) {
                self.report_missing(Message::Name("storage type"));
            }
            self.expect_right_paren();
            Some(self.finish_node(FIELD_TYPE, mark))
        } else {
            self.parse_storage_type()
                .map(|storage_type| node(FIELD_TYPE, [storage_type]))
        }
    }

    fn parse_func_type(&mut self, mark: NodeMark) -> Option<GreenNode> {
        while let Some(node) = self.try_parse_with_trivias(Self::parse_param) {
            self.add_child(node);
        }
        while let Some(node) = self.try_parse_with_trivias(Self::parse_result) {
            self.add_child(node);
        }
        self.expect_right_paren();
        Some(self.finish_node(FUNC_TYPE, mark))
    }

    pub(super) fn parse_global_type(&mut self) -> Option<GreenNode> {
        if let Some(mark) = self.try_parse(|parser| {
            let mark = parser.start_node();
            parser.lexer.next(L_PAREN)?;
            parser.add_child(green::L_PAREN.clone());
            parser.parse_trivias();
            parser.lexer.keyword("mut")?;
            parser.add_child(green::KW_MUT.clone());
            Some(mark)
        }) {
            if !self.recover(Self::parse_value_type) {
                self.report_missing(Message::Name("value type"));
            }
            self.expect_right_paren();
            Some(self.finish_node(GLOBAL_TYPE, mark))
        } else {
            self.parse_value_type()
                .map(|value_type| node(GLOBAL_TYPE, [value_type]))
        }
    }

    pub(super) fn parse_heap_type<const IMMEDIATE: bool>(&mut self) -> Option<GreenElement> {
        self.lexer
            .eat(TYPE_KEYWORD)
            .and_then(|mut token| match token.text {
                "any" | "eq" | "i31" | "struct" | "array" | "none" | "func" | "nofunc" | "exn" | "noexn" | "extern"
                | "noextern" | "cont" | "nocont" => Some(node(HEAP_TYPE, [token.into()]).into()),
                _ => {
                    if IMMEDIATE {
                        // for better error reporting
                        None
                    } else {
                        token.kind = ERROR;
                        self.report_error_token(&token, Message::Description("invalid heap type"));
                        Some(token.into())
                    }
                }
            })
            .or_else(|| self.parse_index().map(|index| node(HEAP_TYPE, [index.into()]).into()))
    }

    fn parse_limits(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        let min = self.expect(UNSIGNED_INT)?;
        self.add_child(min);
        self.eat(UNSIGNED_INT);
        Some(self.finish_node(LIMITS, mark))
    }

    fn parse_mem_page_size(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        self.lexer.next(L_PAREN)?;
        self.add_child(green::L_PAREN.clone());
        self.parse_trivias();
        let keyword = self.lexer.keyword("pagesize")?;
        self.add_child(keyword);
        if !self.retry(|parser| parser.expect(UNSIGNED_INT)) {
            self.report_missing(Message::Name("unsigned integer"));
        }
        self.expect_right_paren();
        Some(self.finish_node(MEM_PAGE_SIZE, mark))
    }

    pub(super) fn parse_mem_type(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        if let Some(addr_type) = self.try_parse(Self::parse_addr_type) {
            self.add_child(addr_type);
            self.parse_trivias();
        }
        let limits = self.parse_limits()?;
        self.add_child(limits);

        if let Some(share) = self.try_parse_with_trivias(|parser| {
            let token = parser.lexer.next(KEYWORD)?;
            if matches!(token.text, "shared" | "unshared") {
                Some(token)
            } else {
                parser.report_error_token(
                    &token,
                    Message::Description("expected share keyword to be `shared` or `unshared`"),
                );
                None
            }
        }) {
            self.add_child(share);
        }
        if let Some(mem_page_size) = self.try_parse_with_trivias(Self::parse_mem_page_size) {
            self.add_child(mem_page_size);
        }
        Some(self.finish_node(MEM_TYPE, mark))
    }

    fn parse_packed_type(&mut self) -> Option<GreenElement> {
        self.lexer.next(TYPE_KEYWORD).and_then(|token| match token.text {
            "i8" | "i16" => Some(node(PACKED_TYPE, [token.into()]).into()),
            _ => None,
        })
    }

    pub(super) fn parse_param(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        self.lexer.next(L_PAREN)?;
        self.add_child(green::L_PAREN.clone());
        self.parse_trivias();
        self.lexer.keyword("param")?;
        self.add_child(green::KW_PARAM.clone());

        if self.eat(IDENT) {
            if !self.recover(Self::parse_value_type) {
                self.report_missing(Message::Name("value type"));
            }
        } else {
            while self.recover(Self::parse_value_type) {}
        }
        self.expect_right_paren();
        Some(self.finish_node(PARAM, mark))
    }

    pub(super) fn parse_ref_type(&mut self) -> Option<GreenNode> {
        self.lexer
            .eat(TYPE_KEYWORD)
            .and_then(|token| match token.text {
                "anyref" | "eqref" | "i31ref" | "structref" | "arrayref" | "nullref" | "funcref" | "nullfuncref"
                | "exnref" | "nullexnref" | "externref" | "nullexternref" | "contref" | "nullcontref" => {
                    Some(node(REF_TYPE, [token.into()]))
                }
                _ => None,
            })
            .or_else(|| self.parse_ref_type_detailed())
    }

    fn parse_ref_type_detailed(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        self.lexer.next(L_PAREN)?;
        self.add_child(green::L_PAREN.clone());
        self.parse_trivias();
        self.lexer.keyword("ref")?;
        self.add_child(green::KW_REF.clone());

        if self
            .try_parse_with_trivias(|parser| parser.lexer.next(MODIFIER_KEYWORD).filter(|token| token.text == "null"))
            .is_some()
        {
            self.add_child(green::MODIFIER_KW_NULL.clone());
        }

        if !self.recover(Self::parse_heap_type::<false>) {
            self.report_missing(Message::Name("heap type"));
        }
        self.expect_right_paren();
        Some(self.finish_node(REF_TYPE, mark))
    }

    pub(super) fn parse_result(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        self.lexer.next(L_PAREN)?;
        self.add_child(green::L_PAREN.clone());
        self.parse_trivias();
        self.lexer.keyword("result")?;
        self.add_child(green::KW_RESULT.clone());

        while self.recover(Self::parse_value_type) {}
        self.expect_right_paren();
        Some(self.finish_node(RESULT, mark))
    }

    fn parse_storage_type(&mut self) -> Option<GreenElement> {
        self.try_parse(Self::parse_packed_type)
            .or_else(|| self.parse_value_type())
    }

    fn parse_struct_type(&mut self, mark: NodeMark) -> Option<GreenNode> {
        while self.recover(Self::parse_field) {}
        self.expect_right_paren();
        Some(self.finish_node(STRUCT_TYPE, mark))
    }

    pub(super) fn parse_sub_type(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        self.lexer.next(L_PAREN)?;
        self.add_child(green::L_PAREN.clone());
        self.parse_trivias();
        match self.lexer.next(KEYWORD)?.text {
            "func" => {
                self.add_child(green::KW_FUNC.clone());
                self.parse_func_type(mark).map(|ty| node(SUB_TYPE, [ty.into()]))
            }
            "struct" => {
                self.add_child(green::KW_STRUCT.clone());
                self.parse_struct_type(mark).map(|ty| node(SUB_TYPE, [ty.into()]))
            }
            "array" => {
                self.add_child(green::KW_ARRAY.clone());
                self.parse_array_type(mark).map(|ty| node(SUB_TYPE, [ty.into()]))
            }
            "cont" => {
                self.add_child(green::KW_CONT.clone());
                self.parse_cont_type(mark).map(|ty| node(SUB_TYPE, [ty.into()]))
            }
            "sub" => {
                self.add_child(green::KW_SUB.clone());
                if let Some(modifier_keyword) = self.try_parse_with_trivias(|parser| {
                    parser
                        .lexer
                        .next(MODIFIER_KEYWORD)
                        .filter(|token| token.text == "final")
                }) {
                    self.add_child(modifier_keyword);
                }

                while let Some(index) = self.try_parse_with_trivias(Self::parse_index) {
                    self.add_child(index);
                }

                if !self.retry(Self::parse_composite_type) {
                    self.report_missing(Message::Name("composite type"));
                }
                self.expect_right_paren();
                Some(self.finish_node(SUB_TYPE, mark))
            }
            _ => None,
        }
    }

    pub(super) fn parse_table_type(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        if let Some(addr_type) = self.try_parse(Self::parse_addr_type) {
            self.add_child(addr_type);
            self.parse_trivias();
        }
        let limits = self.parse_limits()?;
        self.add_child(limits);
        if !self.recover(Self::parse_ref_type) {
            self.report_missing(Message::Name("ref type"));
        }
        Some(self.finish_node(TABLE_TYPE, mark))
    }

    pub(super) fn parse_value_type(&mut self) -> Option<GreenElement> {
        if self.lexer.peek(L_PAREN).is_some() {
            self.parse_ref_type_detailed().map(GreenElement::from)
        } else {
            self.expect(TYPE_KEYWORD).map(|mut token| match token.text {
                "i32" => green::TYPE_I32.clone(),
                "i64" => green::TYPE_I64.clone(),
                "f32" => green::TYPE_F32.clone(),
                "f64" => green::TYPE_F64.clone(),
                "v128" => node(VEC_TYPE, [token.into()]).into(),
                "anyref" | "eqref" | "i31ref" | "structref" | "arrayref" | "nullref" | "funcref" | "nullfuncref"
                | "exnref" | "nullexnref" | "externref" | "nullexternref" | "contref" | "nullcontref" => {
                    node(REF_TYPE, [token.into()]).into()
                }
                _ => {
                    token.kind = ERROR;
                    self.report_error_token(&token, Message::Description("invalid value type"));
                    token.into()
                }
            })
        }
    }
}
