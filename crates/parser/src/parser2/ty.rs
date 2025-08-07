use super::{node, GreenElement, Parser};
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
        children.push(self.lexer.next(L_PAREN)?.into());
        self.parse_trivias(&mut children);
        let keyword = self.lexer.next(KEYWORD)?;
        let keyword_text = keyword.text;
        children.push(keyword.into());

        match keyword_text {
            "func" => self.parse_func_type(children),
            "struct" => self.parse_struct_type(children),
            "array" => self.parse_array_type(children),
            _ => None,
        }
    }

    fn parse_field(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(6);
        children.push(self.lexer.next(L_PAREN)?.into());
        self.parse_trivias(&mut children);
        children.push(self.parse_keyword("field")?);

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
        self.try_parse(|parser| {
            let mut children = Vec::with_capacity(5);
            children.push(parser.lexer.next(L_PAREN)?.into());
            parser.parse_trivias(&mut children);
            children.push(parser.parse_keyword("mut")?);
            if !parser.recover(Self::parse_storage_type, &mut children) {
                parser.report_missing(Message::Name("storage type"));
            }
            parser.expect_right_paren(&mut children);
            Some(node(FIELD_TYPE, children))
        })
        .or_else(|| {
            self.parse_storage_type()
                .map(|storage_type| node(FIELD_TYPE, [storage_type]))
        })
    }

    fn parse_func_type(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        while let Some(mut node_or_tokens) = self.try_parse(|parser| {
            let mut children = Vec::with_capacity(2);
            parser.parse_trivias(&mut children);
            children.push(parser.parse_param()?.into());
            Some(children)
        }) {
            children.append(&mut node_or_tokens);
        }

        while let Some(mut node_or_tokens) = self.try_parse(|parser| {
            let mut children = Vec::with_capacity(2);
            parser.parse_trivias(&mut children);
            children.push(parser.parse_result()?.into());
            Some(children)
        }) {
            children.append(&mut node_or_tokens);
        }

        self.expect_right_paren(&mut children);
        Some(node(FUNC_TYPE, children))
    }

    fn parse_heap_type(&mut self) -> Option<GreenElement> {
        self.lexer
            .eat(TYPE_KEYWORD)
            .map(|mut token| match token.text {
                "any" | "eq" | "i31" | "struct" | "array" | "none" | "func" | "nofunc"
                | "extern" | "noextern" => node(HEAP_TYPE, [token.into()]).into(),
                _ => {
                    token.kind = ERROR;
                    self.report_error_token(&token, Message::Description("invalid heap type"));
                    token.into()
                }
            })
            .or_else(|| {
                self.parse_index()
                    .map(|index| node(HEAP_TYPE, [index.into()]).into())
            })
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
        children.push(self.lexer.next(L_PAREN)?.into());
        self.parse_trivias(&mut children);
        children.push(self.parse_keyword("param")?);

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

    fn parse_ref_type_detailed(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(7);
        children.push(self.lexer.next(L_PAREN)?.into());
        self.parse_trivias(&mut children);
        children.push(self.parse_keyword("ref")?);

        if let Some(mut tokens) = self.try_parse(|parser| {
            let mut children = Vec::with_capacity(2);
            parser.parse_trivias(&mut children);
            children.push(parser.parse_keyword("null")?);
            Some(children)
        }) {
            children.append(&mut tokens);
        }

        if !self.recover(Self::parse_heap_type, &mut children) {
            self.report_missing(Message::Name("heap type"));
        }
        self.expect_right_paren(&mut children);
        Some(node(REF_TYPE, children))
    }

    pub(super) fn parse_result(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(6);
        children.push(self.lexer.next(L_PAREN)?.into());
        self.parse_trivias(&mut children);
        children.push(self.parse_keyword("result")?);

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
        children.push(self.lexer.next(L_PAREN)?.into());
        self.parse_trivias(&mut children);
        let mut keyword = self.lexer.next(KEYWORD)?;
        match keyword.text {
            "func" => {
                children.push(keyword.into());
                self.parse_func_type(children)
                    .map(|ty| node(SUB_TYPE, [ty.into()]))
            }
            "struct" => {
                children.push(keyword.into());
                self.parse_struct_type(children)
                    .map(|ty| node(SUB_TYPE, [ty.into()]))
            }
            "array" => {
                children.push(keyword.into());
                self.parse_array_type(children)
                    .map(|ty| node(SUB_TYPE, [ty.into()]))
            }
            "sub" => {
                children.push(keyword.into());
                if let Some(mut tokens) = self.try_parse(|parser| {
                    let mut children = Vec::with_capacity(2);
                    parser.parse_trivias(&mut children);
                    children.push(parser.parse_keyword("final")?);
                    Some(children)
                }) {
                    children.append(&mut tokens);
                }
                while self.recover(Self::parse_index, &mut children) {}
                if !self.recover(Self::parse_composite_type, &mut children) {
                    self.report_missing(Message::Name("composite type"));
                }
                self.expect_right_paren(&mut children);
                Some(node(SUB_TYPE, children))
            }
            _ => {
                keyword.kind = ERROR;
                self.report_error_token(&keyword, Message::Description("invalid sub type"));
                children.push(keyword.into());
                while self.eat(ERROR, &mut children) {}
                self.eat(R_PAREN, &mut children);
                Some(node(SUB_TYPE, children))
            }
        }
    }

    fn parse_value_type(&mut self) -> Option<GreenElement> {
        if self.lexer.peek(L_PAREN).is_some() {
            self.parse_ref_type_detailed().map(GreenElement::from)
        } else {
            self.expect(TYPE_KEYWORD).map(|mut token| match token.text {
                "i32" | "i64" | "f32" | "f64" => node(NUM_TYPE, [token.into()]).into(),
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
