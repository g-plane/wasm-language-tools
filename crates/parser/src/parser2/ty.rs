use super::{green, node, GreenElement, Parser};
use crate::error::Message;
use rowan::GreenNode;
use wat_syntax::SyntaxKind::*;

impl Parser<'_> {
    fn parse_array_type(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        if !self.recover(Self::parse_field_type, &mut children) {
            self.report_missing(Message::Name("field type"));
        }
        self.expect_right_paren(&mut children);
        Some(node(ARRAY_TYPE, children))
    }

    fn parse_composite_type(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(3);
        self.lexer.next(L_PAREN)?;
        children.push(green::L_PAREN.clone());
        self.parse_trivias(&mut children);
        match self.lexer.next(KEYWORD)?.text {
            "func" => {
                children.push(green::KW_FUNC.clone());
                self.parse_func_type(children)
            }
            "struct" => {
                children.push(green::KW_STRUCT.clone());
                self.parse_struct_type(children)
            }
            "array" => {
                children.push(green::KW_ARRAY.clone());
                self.parse_array_type(children)
            }
            _ => None,
        }
    }

    fn parse_field(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(6);
        self.lexer.next(L_PAREN)?;
        children.push(green::L_PAREN.clone());
        self.parse_trivias(&mut children);
        children.push(self.lexer.keyword("field")?.into());

        if self.eat(IDENT, &mut children) {
            if !self.recover(Self::parse_field_type, &mut children) {
                self.report_missing(Message::Name("field type"));
            }
        } else {
            while self.recover(Self::parse_field_type, &mut children) {}
        }
        self.expect_right_paren(&mut children);
        Some(node(FIELD, children))
    }

    fn parse_field_type(&mut self) -> Option<GreenNode> {
        if let Some(mut children) = self.try_parse(|parser| {
            let mut children = Vec::with_capacity(5);
            parser.lexer.next(L_PAREN)?;
            children.push(green::L_PAREN.clone());
            parser.parse_trivias(&mut children);
            parser.lexer.keyword("mut")?;
            children.push(green::KW_MUT.clone());
            Some(children)
        }) {
            if !self.recover(Self::parse_storage_type, &mut children) {
                self.report_missing(Message::Name("storage type"));
            }
            self.expect_right_paren(&mut children);
            Some(node(FIELD_TYPE, children))
        } else {
            self.parse_storage_type()
                .map(|storage_type| node(FIELD_TYPE, [storage_type]))
        }
    }

    fn parse_func_type(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        while let Some((trivias, node)) = self.try_parse_with_trivias(Self::parse_param) {
            children.extend(trivias);
            children.push(node.into());
        }
        while let Some((trivias, node)) = self.try_parse_with_trivias(Self::parse_result) {
            children.extend(trivias);
            children.push(node.into());
        }
        self.expect_right_paren(&mut children);
        Some(node(FUNC_TYPE, children))
    }

    pub(super) fn parse_global_type(&mut self) -> Option<GreenNode> {
        if let Some(mut children) = self.try_parse(|parser| {
            let mut children = Vec::with_capacity(2);
            parser.lexer.next(L_PAREN)?;
            children.push(green::L_PAREN.clone());
            parser.parse_trivias(&mut children);
            parser.lexer.keyword("mut")?;
            children.push(green::KW_MUT.clone());
            Some(children)
        }) {
            if !self.recover(Self::parse_value_type, &mut children) {
                self.report_missing(Message::Name("value type"));
            }
            self.expect_right_paren(&mut children);
            Some(node(GLOBAL_TYPE, children))
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
        let mut children = vec![self.expect(UNSIGNED_INT)?.into()];
        self.eat(UNSIGNED_INT, &mut children);
        Some(node(LIMITS, children))
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
        let mut children = Vec::with_capacity(6);
        self.lexer.next(L_PAREN)?;
        children.push(green::L_PAREN.clone());
        self.parse_trivias(&mut children);
        self.lexer.keyword("param")?;
        children.push(green::KW_PARAM.clone());

        if self.eat(IDENT, &mut children) {
            if !self.recover(Self::parse_value_type, &mut children) {
                self.report_missing(Message::Name("value type"));
            }
        } else {
            while self.recover(Self::parse_value_type, &mut children) {}
        }
        self.expect_right_paren(&mut children);
        Some(node(PARAM, children))
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
        let mut children = Vec::with_capacity(7);
        self.lexer.next(L_PAREN)?;
        children.push(green::L_PAREN.clone());
        self.parse_trivias(&mut children);
        self.lexer.keyword("ref")?;
        children.push(green::KW_REF.clone());

        if let Some((trivias, keyword)) =
            self.try_parse_with_trivias(|parser| parser.lexer.keyword("null"))
        {
            children.extend(trivias);
            children.push(keyword.into());
        }

        if !self.recover(Self::parse_heap_type::<false>, &mut children) {
            self.report_missing(Message::Name("heap type"));
        }
        self.expect_right_paren(&mut children);
        Some(node(REF_TYPE, children))
    }

    pub(super) fn parse_result(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(6);
        self.lexer.next(L_PAREN)?;
        children.push(green::L_PAREN.clone());
        self.parse_trivias(&mut children);
        self.lexer.keyword("result")?;
        children.push(green::KW_RESULT.clone());

        while self.recover(Self::parse_value_type, &mut children) {}
        self.expect_right_paren(&mut children);
        Some(node(RESULT, children))
    }

    fn parse_storage_type(&mut self) -> Option<GreenElement> {
        self.try_parse(Self::parse_packed_type)
            .or_else(|| self.parse_value_type())
    }

    fn parse_struct_type(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        while self.recover(Self::parse_field, &mut children) {}
        self.expect_right_paren(&mut children);
        Some(node(STRUCT_TYPE, children))
    }

    pub(super) fn parse_sub_type(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(3);
        self.lexer.next(L_PAREN)?;
        children.push(green::L_PAREN.clone());
        self.parse_trivias(&mut children);
        match self.lexer.next(KEYWORD)?.text {
            "func" => {
                children.push(green::KW_FUNC.clone());
                self.parse_func_type(children)
                    .map(|ty| node(SUB_TYPE, [ty.into()]))
            }
            "struct" => {
                children.push(green::KW_STRUCT.clone());
                self.parse_struct_type(children)
                    .map(|ty| node(SUB_TYPE, [ty.into()]))
            }
            "array" => {
                children.push(green::KW_ARRAY.clone());
                self.parse_array_type(children)
                    .map(|ty| node(SUB_TYPE, [ty.into()]))
            }
            "sub" => {
                children.push(green::KW_SUB.clone());
                if let Some((trivias, keyword)) =
                    self.try_parse_with_trivias(|parser| parser.lexer.keyword("final"))
                {
                    children.extend(trivias);
                    children.push(keyword.into());
                }

                while let Some((trivias, index)) = self.try_parse_with_trivias(Self::parse_index) {
                    children.extend(trivias);
                    children.push(index.into());
                }

                if !self.retry(Self::parse_composite_type, &mut children) {
                    self.report_missing(Message::Name("composite type"));
                }
                self.expect_right_paren(&mut children);
                Some(node(SUB_TYPE, children))
            }
            _ => None,
        }
    }

    pub(super) fn parse_table_type(&mut self) -> Option<GreenNode> {
        let mut children = vec![self.parse_limits()?.into()];
        if !self.recover(Self::parse_ref_type, &mut children) {
            self.report_missing(Message::Name("ref type"));
        }
        Some(node(TABLE_TYPE, children))
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
