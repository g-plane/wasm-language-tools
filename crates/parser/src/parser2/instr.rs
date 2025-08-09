use super::{lexer::Token, node, GreenElement, Parser};
use crate::error::Message;
use rowan::GreenNode;
use wat_syntax::SyntaxKind::{self, *};

impl Parser<'_> {
    fn parse_block_if_folded(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        self.eat(IDENT, &mut children);
        if let Some((mut trivias, node)) = self.try_parse_with_trivias(Self::parse_block_type) {
            children.append(&mut trivias);
            children.push(node.into());
        }

        while !self.should_exit_block_if_cond() && self.recover(Self::parse_instr, &mut children) {}

        if !self.recover(Self::parse_then_block, &mut children) {
            self.report_missing(Message::Name("then block"));
        }

        if let Some((mut trivias, mut else_children)) = self.try_parse_with_trivias(|parser| {
            let mut children = Vec::with_capacity(2);
            children.push(parser.lexer.next(L_PAREN)?.into());
            parser.parse_trivias(&mut children);
            children.push(parser.parse_keyword("else")?);
            Some(children)
        }) {
            children.append(&mut trivias);
            self.eat(IDENT, &mut else_children);
            while self.recover(Self::parse_instr, &mut else_children) {}
            self.expect_right_paren(&mut else_children);
            children.push(node(BLOCK_IF_ELSE, else_children).into());
        }

        self.expect_right_paren(&mut children);
        Some(node(BLOCK_IF, children))
    }

    fn parse_block_if_sequence(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        self.eat(IDENT, &mut children);
        if let Some((mut trivias, node)) = self.try_parse_with_trivias(Self::parse_block_type) {
            children.append(&mut trivias);
            children.push(node.into());
        }

        let mut then_children = Vec::with_capacity(2);
        while self
            .lexer
            .peek(KEYWORD)
            .filter(|token| matches!(token.text, "end" | "else"))
            .is_none()
            && self.recover(Self::parse_instr, &mut then_children)
        {}
        children.push(node(BLOCK_IF_THEN, then_children).into());

        if let Some((mut trivias, else_keyword)) =
            self.try_parse_with_trivias(|parser| parser.parse_keyword("else"))
        {
            children.append(&mut trivias);
            let mut else_children = vec![else_keyword];
            self.eat(IDENT, &mut else_children);
            while self
                .lexer
                .peek(KEYWORD)
                .filter(|token| token.text == "end")
                .is_none()
                && self.recover(Self::parse_instr, &mut else_children)
            {}
            children.push(node(BLOCK_IF_ELSE, else_children).into());
        }

        if !self.recover(Self::parse_end_keyword, &mut children) {
            self.report_missing(Message::Str("end"));
        }
        self.eat(IDENT, &mut children);
        Some(node(BLOCK_IF, children))
    }

    fn parse_block_like_folded(
        &mut self,
        kind: SyntaxKind,
        mut children: Vec<GreenElement>,
    ) -> Option<GreenNode> {
        self.eat(IDENT, &mut children);
        if let Some((mut trivias, node)) = self.try_parse_with_trivias(Self::parse_block_type) {
            children.append(&mut trivias);
            children.push(node.into());
        }

        while self.recover(Self::parse_instr, &mut children) {}

        self.expect_right_paren(&mut children);
        Some(node(kind, children))
    }

    fn parse_block_like_sequence(
        &mut self,
        kind: SyntaxKind,
        mut children: Vec<GreenElement>,
    ) -> Option<GreenNode> {
        self.eat(IDENT, &mut children);
        if let Some((mut trivias, node)) = self.try_parse_with_trivias(Self::parse_block_type) {
            children.append(&mut trivias);
            children.push(node.into());
        }

        while self
            .lexer
            .peek(KEYWORD)
            .filter(|token| token.text == "end")
            .is_none()
            && self.recover(Self::parse_instr, &mut children)
        {}

        if !self.recover(Self::parse_end_keyword, &mut children) {
            self.report_missing(Message::Str("end"));
        }
        self.eat(IDENT, &mut children);
        Some(node(kind, children))
    }

    fn parse_block_type(&mut self) -> Option<GreenNode> {
        self.parse_type_use()
            .map(|type_use| node(BLOCK_TYPE, [type_use.into()]))
    }

    fn parse_end_keyword(&mut self) -> Option<GreenElement> {
        self.parse_keyword("end")
    }

    fn parse_immediate(&mut self) -> Option<GreenNode> {
        self.lexer
            .eat(INT)
            .or_else(|| self.lexer.eat(FLOAT))
            .or_else(|| self.lexer.eat(IDENT))
            .or_else(|| self.lexer.eat(STRING))
            .or_else(|| self.lexer.eat(UNSIGNED_INT))
            .or_else(|| self.lexer.eat(MEM_ARG))
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
                self.try_parse(Self::parse_heap_type::<true>)
                    .map(|child| node(IMMEDIATE, [child.into()]))
            })
    }

    pub(super) fn parse_instr(&mut self) -> Option<GreenNode> {
        if let Some(token) = self.lexer.eat(L_PAREN) {
            let mut children = Vec::with_capacity(4);
            children.push(token.into());
            let mut token = self.expect(INSTR_NAME)?;
            match token.text {
                "if" => {
                    token.kind = KEYWORD;
                    children.push(token.into());
                    self.parse_block_if_folded(children)
                }
                "loop" => {
                    token.kind = KEYWORD;
                    children.push(token.into());
                    self.parse_block_like_folded(BLOCK_LOOP, children)
                }
                "block" => {
                    token.kind = KEYWORD;
                    children.push(token.into());
                    self.parse_block_like_folded(BLOCK_BLOCK, children)
                }
                _ => {
                    children.push(token.into());
                    self.parse_plain_instr_folded(children)
                }
            }
        } else {
            let mut children = Vec::with_capacity(2);
            let mut token = self.expect(INSTR_NAME)?;
            match token.text {
                "if" => {
                    token.kind = KEYWORD;
                    children.push(token.into());
                    self.parse_block_if_sequence(children)
                }
                "loop" => {
                    token.kind = KEYWORD;
                    children.push(token.into());
                    self.parse_block_like_sequence(BLOCK_LOOP, children)
                }
                "block" => {
                    token.kind = KEYWORD;
                    children.push(token.into());
                    self.parse_block_like_sequence(BLOCK_BLOCK, children)
                }
                _ => {
                    children.push(token.into());
                    self.parse_plain_instr_sequence(children)
                }
            }
        }
    }

    fn parse_plain_instr_folded(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        while let Some((mut trivias, node)) = self.try_parse_with_trivias(|parser| {
            parser.parse_immediate().or_else(|| {
                parser.lexer.eat(KEYWORD).map(|mut token| {
                    token.kind = ERROR;
                    parser.report_error_token(&token, Message::Description("invalid immediate"));
                    node(IMMEDIATE, [token.into()])
                })
            })
        }) {
            children.append(&mut trivias);
            children.push(node.into());
        }
        while self.lexer.peek(L_PAREN).is_some() && self.recover(Self::parse_instr, &mut children) {
        }
        self.expect_right_paren(&mut children);
        Some(node(PLAIN_INSTR, children))
    }

    fn parse_plain_instr_sequence(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        while let Some((mut trivias, node)) = self.try_parse_with_trivias(|parser| {
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
            children.append(&mut trivias);
            children.push(node.into());
        }
        Some(node(PLAIN_INSTR, children))
    }

    fn parse_then_block(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(2);
        children.push(self.lexer.next(L_PAREN)?.into());
        self.parse_trivias(&mut children);
        children.push(self.parse_keyword("then")?);
        while self.recover(Self::parse_instr, &mut children) {}
        self.expect_right_paren(&mut children);
        Some(node(BLOCK_IF_THEN, children))
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
