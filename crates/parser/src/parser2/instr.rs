use super::{GreenElement, Parser, builder::NodeMark, green, lexer::Token, node};
use crate::error::{Message, SyntaxError};
use rowan::{GreenNode, TextRange};
use wat_syntax::SyntaxKind::{self, *};

impl Parser<'_> {
    fn parse_block_if_folded(&mut self, mark: NodeMark) -> Option<GreenNode> {
        self.eat(IDENT);
        if let Some(node) = self.try_parse_with_trivias(Self::parse_block_type) {
            self.add_child(node);
        }

        while !self.should_exit_block_if_cond() && self.recover(Self::parse_instr) {}

        if !self.recover(Self::parse_then_block) {
            self.report_missing(Message::Name("then block"));
        }

        if let Some(mark) = self.try_parse_with_trivias(|parser| {
            let mark = parser.start_node();
            parser.lexer.next(L_PAREN)?;
            parser.add_child(green::L_PAREN.clone());
            parser.parse_trivias();
            parser.lexer.keyword("else")?;
            parser.add_child(green::KW_ELSE.clone());
            Some(mark)
        }) {
            self.eat(IDENT);
            while self.recover(Self::parse_instr) {}
            self.expect_right_paren();
            let node = self.finish_node(BLOCK_IF_ELSE, mark);
            self.add_child(node);
        }

        self.expect_right_paren();
        Some(self.finish_node(BLOCK_IF, mark))
    }

    fn parse_block_if_sequence(&mut self, mark: NodeMark) -> Option<GreenNode> {
        self.eat(IDENT);
        if let Some(node) = self.try_parse_with_trivias(Self::parse_block_type) {
            self.add_child(node);
        }

        let then_mark = self.start_node();
        while self
            .lexer
            .peek(KEYWORD)
            .filter(|token| matches!(token.text, "end" | "else"))
            .is_none()
            && self.recover(Self::parse_instr)
        {}
        let node = self.finish_node(BLOCK_IF_THEN, then_mark);
        self.add_child(node);

        if self
            .try_parse_with_trivias(|parser| parser.lexer.keyword("else"))
            .is_some()
        {
            let else_mark = self.start_node();
            self.add_child(green::KW_ELSE.clone());
            self.eat(IDENT);
            while self
                .lexer
                .peek(KEYWORD)
                .filter(|token| token.text == "end")
                .is_none()
                && self.recover(Self::parse_instr)
            {}
            let node = self.finish_node(BLOCK_IF_ELSE, else_mark);
            self.add_child(node);
        }

        if !self.recover(Self::parse_end_keyword) {
            self.report_missing(Message::Str("end"));
        }
        self.eat(IDENT);
        Some(self.finish_node(BLOCK_IF, mark))
    }

    fn parse_block_like_folded(&mut self, kind: SyntaxKind, mark: NodeMark) -> Option<GreenNode> {
        self.eat(IDENT);
        if let Some(node) = self.try_parse_with_trivias(Self::parse_block_type) {
            self.add_child(node);
        }

        while self.recover(Self::parse_instr) {}

        self.expect_right_paren();
        Some(self.finish_node(kind, mark))
    }

    fn parse_block_like_sequence(&mut self, kind: SyntaxKind, mark: NodeMark) -> Option<GreenNode> {
        self.eat(IDENT);
        if let Some(node) = self.try_parse_with_trivias(Self::parse_block_type) {
            self.add_child(node);
        }

        while self
            .lexer
            .peek(KEYWORD)
            .filter(|token| token.text == "end")
            .is_none()
            && self.recover(Self::parse_instr)
        {}

        if !self.recover(Self::parse_end_keyword) {
            self.report_missing(Message::Str("end"));
        }
        self.eat(IDENT);
        Some(self.finish_node(kind, mark))
    }

    fn parse_block_try_table_folded(&mut self, mark: NodeMark) -> Option<GreenNode> {
        self.eat(IDENT);
        if let Some(node) = self.try_parse_with_trivias(Self::parse_block_type) {
            self.add_child(node);
        }
        while let Some(node) = self.try_parse_with_trivias(Self::parse_catch) {
            self.add_child(node);
        }

        while self.recover(Self::parse_instr) {}

        self.expect_right_paren();
        Some(self.finish_node(BLOCK_TRY_TABLE, mark))
    }

    fn parse_block_try_table_sequence(&mut self, mark: NodeMark) -> Option<GreenNode> {
        self.eat(IDENT);
        if let Some(node) = self.try_parse_with_trivias(Self::parse_block_type) {
            self.add_child(node);
        }
        while let Some(node) = self.try_parse_with_trivias(Self::parse_catch) {
            self.add_child(node);
        }

        while self
            .lexer
            .peek(KEYWORD)
            .filter(|token| token.text == "end")
            .is_none()
            && self.recover(Self::parse_instr)
        {}

        if !self.recover(Self::parse_end_keyword) {
            self.report_missing(Message::Str("end"));
        }
        self.eat(IDENT);
        Some(self.finish_node(BLOCK_TRY_TABLE, mark))
    }

    fn parse_block_type(&mut self) -> Option<GreenNode> {
        self.parse_type_use()
            .map(|type_use| node(BLOCK_TYPE, [type_use.into()]))
    }

    fn parse_catch(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        self.lexer.next(L_PAREN)?;
        self.add_child(green::L_PAREN.clone());
        self.parse_trivias();
        let keyword = self.lexer.next(KEYWORD)?;
        match keyword.text {
            "catch" | "catch_ref" => {
                self.add_child(keyword);
                if !self.recover(Self::parse_index) {
                    self.report_missing(Message::Name("tag index"));
                }
                if !self.recover(Self::parse_index) {
                    self.report_missing(Message::Name("label index"));
                }
                self.expect_right_paren();
                Some(self.finish_node(CATCH, mark))
            }
            "catch_all" | "catch_all_ref" => {
                self.add_child(keyword);
                if !self.recover(Self::parse_index) {
                    self.report_missing(Message::Name("label index"));
                }
                self.expect_right_paren();
                Some(self.finish_node(CATCH_ALL, mark))
            }
            _ => None,
        }
    }

    fn parse_end_keyword(&mut self) -> Option<GreenElement> {
        self.lexer.keyword("end").map(|_| green::KW_END.clone())
    }

    fn parse_immediate(&mut self) -> Option<GreenNode> {
        self.lexer
            .eat(INT)
            .or_else(|| {
                self.lexer.eat(FLOAT).inspect(|token| {
                    if token.kind == ERROR {
                        self.report_error_token(
                            token,
                            Message::Description("invalid float literal"),
                        );
                    }
                })
            })
            .or_else(|| self.lexer.eat(IDENT))
            .or_else(|| self.lexer.eat(STRING))
            .or_else(|| self.lexer.eat(SHAPE_DESCRIPTOR))
            .map(|token| node(IMMEDIATE, [token.into()]))
            .or_else(|| {
                self.try_parse(Self::parse_ref_type)
                    .map(|child| node(IMMEDIATE, [child.into()]))
            })
            .or_else(|| {
                self.try_parse(Self::parse_type_use)
                    .map(|child| node(IMMEDIATE, [child.into()]))
            })
            .or_else(|| {
                self.try_parse(Self::parse_mem_arg)
                    .map(|child| node(IMMEDIATE, [child.into()]))
            })
            .or_else(|| {
                self.try_parse(Self::parse_heap_type::<true>)
                    .map(|child| node(IMMEDIATE, [child]))
            })
    }

    pub(super) fn parse_instr(&mut self) -> Option<GreenNode> {
        if self.lexer.eat(L_PAREN).is_some() {
            let mark = self.start_node();
            self.add_child(green::L_PAREN.clone());
            self.parse_trivias();
            let token = self.expect(INSTR_NAME)?;
            match token.text {
                "if" => {
                    self.add_child(green::KW_IF.clone());
                    self.parse_block_if_folded(mark)
                }
                "loop" => {
                    self.add_child(green::KW_LOOP.clone());
                    self.parse_block_like_folded(BLOCK_LOOP, mark)
                }
                "block" => {
                    self.add_child(green::KW_BLOCK.clone());
                    self.parse_block_like_folded(BLOCK_BLOCK, mark)
                }
                "try_table" => {
                    self.add_child(green::KW_TRY_TABLE.clone());
                    self.parse_block_try_table_folded(mark)
                }
                _ => {
                    self.add_child(token);
                    self.parse_plain_instr_folded(mark)
                }
            }
        } else {
            let mark = self.start_node();
            let token = self.expect(INSTR_NAME)?;
            match token.text {
                "if" => {
                    self.add_child(green::KW_IF.clone());
                    self.parse_block_if_sequence(mark)
                }
                "loop" => {
                    self.add_child(green::KW_LOOP.clone());
                    self.parse_block_like_sequence(BLOCK_LOOP, mark)
                }
                "block" => {
                    self.add_child(green::KW_BLOCK.clone());
                    self.parse_block_like_sequence(BLOCK_BLOCK, mark)
                }
                "try_table" => {
                    self.add_child(green::KW_TRY_TABLE.clone());
                    self.parse_block_try_table_sequence(mark)
                }
                _ => {
                    self.add_child(token);
                    self.parse_plain_instr_sequence(mark)
                }
            }
        }
    }

    fn parse_mem_arg(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        let keyword = self.lexer.next(MEM_ARG_KEYWORD)?;
        self.add_child(keyword);

        const MSG: &str = "whitespaces or comments are not allowed inside memory argument";

        let before_trivias = self.lexer.checkpoint().at(self.source);
        if let Some((after_trivias, eq)) = self.try_parse_with_trivias(|parser| {
            let after_trivias = parser.lexer.checkpoint().at(parser.source);
            parser.lexer.next(EQ).map(|eq| (after_trivias, eq))
        }) {
            self.add_child(eq);
            if after_trivias > before_trivias {
                self.errors.push(SyntaxError {
                    range: TextRange::new(before_trivias, after_trivias),
                    message: Message::Description(MSG),
                });
            }
        } else {
            self.report_missing(Message::Char('='));
        }

        let before_trivias = self.lexer.checkpoint().at(self.source);
        if let Some((after_trivias, unsigned_int)) = self.try_parse_with_trivias(|parser| {
            let after_trivias = parser.lexer.checkpoint().at(parser.source);
            parser
                .lexer
                .next(UNSIGNED_INT)
                .map(|unsigned_int| (after_trivias, unsigned_int))
        }) {
            self.add_child(unsigned_int);
            if after_trivias > before_trivias {
                self.errors.push(SyntaxError {
                    range: TextRange::new(before_trivias, after_trivias),
                    message: Message::Description(MSG),
                });
            }
        } else {
            self.report_missing(Message::Name("unsigned int"));
        }

        Some(self.finish_node(MEM_ARG, mark))
    }

    fn parse_plain_instr_folded(&mut self, mark: NodeMark) -> Option<GreenNode> {
        while let Some(node_or_token) = self.try_parse_with_trivias(|parser| {
            parser
                .parse_immediate()
                .map(GreenElement::from)
                .or_else(|| {
                    parser.lexer.eat(ERROR).map(|token| {
                        parser
                            .report_error_token(&token, Message::Description("invalid immediate"));
                        token.into()
                    })
                })
        }) {
            self.add_child(node_or_token);
        }
        while self.lexer.peek(L_PAREN).is_some() && self.recover(Self::parse_instr) {}
        self.expect_right_paren();
        Some(self.finish_node(PLAIN_INSTR, mark))
    }

    fn parse_plain_instr_sequence(&mut self, mark: NodeMark) -> Option<GreenNode> {
        while let Some(node) = self.try_parse_with_trivias(|parser| {
            if parser
                .lexer
                .peek(KEYWORD)
                .is_some_and(|token| matches!(token.text, "end" | "else"))
            {
                None
            } else {
                parser.parse_immediate()
            }
        }) {
            self.add_child(node);
        }
        Some(self.finish_node(PLAIN_INSTR, mark))
    }

    fn parse_then_block(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        self.lexer.next(L_PAREN)?;
        self.add_child(green::L_PAREN.clone());
        self.parse_trivias();
        self.lexer.keyword("then")?;
        self.add_child(green::KW_THEN.clone());
        while self.recover(Self::parse_instr) {}
        self.expect_right_paren();
        Some(self.finish_node(BLOCK_IF_THEN, mark))
    }

    fn should_exit_block_if_cond(&mut self) -> bool {
        let checkpoint = self.lexer.checkpoint();
        while self.lexer.trivia().is_some() {}
        if self.lexer.next(L_PAREN).is_none() {
            return true;
        }
        while self.lexer.trivia().is_some() {}
        let result = matches!(
            self.lexer.next(KEYWORD),
            Some(Token {
                text: "then" | "else",
                ..
            })
        );
        self.lexer.reset(checkpoint);
        result
    }
}
