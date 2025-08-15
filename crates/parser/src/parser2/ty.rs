use super::{builder::NodeMark, green, node, GreenElement, Parser};
use crate::error::Message;
use rowan::GreenNode;
use wat_syntax::SyntaxKind::*;

impl Parser<'_> {
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
                "any" | "eq" | "i31" | "struct" | "array" | "none" | "func" | "nofunc"
                | "extern" | "noextern" => Some(node(HEAP_TYPE, [token.into()]).into()),
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
            .or_else(|| {
                self.parse_index()
                    .map(|index| node(HEAP_TYPE, [index.into()]).into())
            })
    }

    fn parse_limits(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        let min = self.expect(UNSIGNED_INT)?;
        self.add_child(min);
        self.eat(UNSIGNED_INT);
        Some(self.finish_node(LIMITS, mark))
    }

    pub(super) fn parse_memory_type(&mut self) -> Option<GreenNode> {
        self.parse_limits()
            .map(|limits| node(MEMORY_TYPE, [limits.into()]))
    }

    fn parse_packed_type(&mut self) -> Option<GreenElement> {
        self.lexer
            .next(TYPE_KEYWORD)
            .and_then(|token| match token.text {
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
                "anyref" | "eqref" | "i31ref" | "structref" | "arrayref" | "nullref"
                | "funcref" | "nullfuncref" | "externref" | "nullexternref" => {
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

        if let Some(keyword) = self.try_parse_with_trivias(|parser| parser.lexer.keyword("null")) {
            self.add_child(keyword);
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
                self.parse_func_type(mark)
                    .map(|ty| node(SUB_TYPE, [ty.into()]))
            }
            "struct" => {
                self.add_child(green::KW_STRUCT.clone());
                self.parse_struct_type(mark)
                    .map(|ty| node(SUB_TYPE, [ty.into()]))
            }
            "array" => {
                self.add_child(green::KW_ARRAY.clone());
                self.parse_array_type(mark)
                    .map(|ty| node(SUB_TYPE, [ty.into()]))
            }
            "sub" => {
                self.add_child(green::KW_SUB.clone());
                if let Some(keyword) =
                    self.try_parse_with_trivias(|parser| parser.lexer.keyword("final"))
                {
                    self.add_child(keyword);
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
                "anyref" | "eqref" | "i31ref" | "structref" | "arrayref" | "nullref"
                | "funcref" | "nullfuncref" | "externref" | "nullexternref" => {
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
