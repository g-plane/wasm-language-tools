use super::{
    GreenElement, Parser,
    builder::{Checkpoint, NodeMark},
    green, helpers,
    lexer::Token,
};
use crate::error::{Message, SyntaxError};
use wat_syntax::{
    GreenNode,
    SyntaxKind::{self, *},
    TextRange,
};

impl<'s> Parser<'s, '_> {
    fn parse_block_if_folded(&mut self, mark: NodeMark) -> Option<GreenNode> {
        self.eat(IDENT);
        if let Some(node) = self.try_parse_with_trivias(Self::parse_type_use) {
            self.add_child(node);
        }

        while let Some(instr) = self.try_parse_with_trivias(Self::parse_instr) {
            self.add_child(instr);
        }

        if !self.recover(Self::parse_then_block) {
            self.report_missing(Message::Name("then block"));
        }

        if let Some(mark) = self.try_parse_with_trivias(|parser| {
            let mark = parser.start_node();
            parser.lexer.next(L_PAREN)?;
            parser.add_child(green::L_PAREN.clone());
            if !parser.lexer.peek_byte()?.is_ascii_alphabetic() {
                parser.parse_trivias();
            }
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
        if let Some(node) = self.try_parse_with_trivias(Self::parse_type_use) {
            self.add_child(node);
        }

        let then_mark = self.start_node();
        let mut has_then_body = false;
        while let Some(instr) = self.try_parse_with_trivias(Self::parse_instr) {
            self.add_child(instr);
            has_then_body = true;
        }
        if has_then_body {
            let node = self.finish_node(BLOCK_IF_THEN, then_mark);
            self.add_child(node);
        }

        if self
            .try_parse_with_trivias(|parser| parser.lexer.keyword("else"))
            .is_some()
        {
            let else_mark = self.start_node();
            self.add_child(green::KW_ELSE.clone());
            self.eat(IDENT);
            while let Some(instr) = self.try_parse_with_trivias(Self::parse_instr) {
                self.add_child(instr);
            }
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
        if let Some(node) = self.try_parse_with_trivias(Self::parse_type_use) {
            self.add_child(node);
        }

        while self.recover(Self::parse_instr) {}

        self.expect_right_paren();
        Some(self.finish_node(kind, mark))
    }

    fn parse_block_like_sequence(&mut self, kind: SyntaxKind, mark: NodeMark) -> Option<GreenNode> {
        self.eat(IDENT);
        if let Some(node) = self.try_parse_with_trivias(Self::parse_type_use) {
            self.add_child(node);
        }

        while let Some(instr) = self.try_parse_with_trivias(Self::parse_instr) {
            self.add_child(instr);
        }

        if !self.recover(Self::parse_end_keyword) {
            self.report_missing(Message::Str("end"));
        }
        self.eat(IDENT);
        Some(self.finish_node(kind, mark))
    }

    fn parse_block_try_table_folded(&mut self, mark: NodeMark) -> Option<GreenNode> {
        self.eat(IDENT);
        if let Some(node) = self.try_parse_with_trivias(Self::parse_type_use) {
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
        if let Some(node) = self.try_parse_with_trivias(Self::parse_type_use) {
            self.add_child(node);
        }
        while let Some(node) = self.try_parse_with_trivias(Self::parse_catch) {
            self.add_child(node);
        }

        while let Some(instr) = self.try_parse_with_trivias(Self::parse_instr) {
            self.add_child(instr);
        }

        if !self.recover(Self::parse_end_keyword) {
            self.report_missing(Message::Str("end"));
        }
        self.eat(IDENT);
        Some(self.finish_node(BLOCK_TRY_TABLE, mark))
    }

    pub(super) fn parse_catch(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        self.lexer.next(L_PAREN)?;
        self.add_child(green::L_PAREN.clone());
        if !self.lexer.peek_byte()?.is_ascii_alphabetic() {
            self.parse_trivias();
        }
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

    pub(super) fn parse_immediate(&mut self) -> Option<GreenNode> {
        match self.lexer.peek_byte()? {
            b'0'..=b'9' | b'-' | b'+' => self
                .lexer
                .eat(INT)
                .map(|token| {
                    if let Some(i) = helpers::parse_small_int(token.text)
                        && let Some(node) = green::IMMEDIATE_INT.get(i)
                    {
                        node.clone()
                    } else if token.text == "-1" {
                        green::IMMEDIATE_INT_NEG_ONE.clone()
                    } else {
                        let token = self.intern_token(token);
                        GreenNode::new(IMMEDIATE, [token.into()])
                    }
                })
                .or_else(|| {
                    self.lexer
                        .eat(FLOAT)
                        .inspect(|token| {
                            if token.kind == ERROR {
                                self.report_error_token(token, Message::Description("invalid float literal"));
                            }
                        })
                        .map(|token| GreenNode::new(IMMEDIATE, [token.into()]))
                }),
            b'$' => self.lexer.next(IDENT).map(|token| {
                let token = self.intern_token(token);
                GreenNode::new(IMMEDIATE, [token.into()])
            }),
            b'a' => self
                .try_parse(Self::parse_mem_arg)
                .map(|child| GreenNode::new(IMMEDIATE, [child.into()]))
                .or_else(|| {
                    self.try_parse(Self::parse_ref_type)
                        .map(|child| GreenNode::new(IMMEDIATE, [child.into()]))
                })
                .or_else(|| {
                    self.try_parse(Self::parse_heap_type::<true>)
                        .map(|child| GreenNode::new(IMMEDIATE, [child]))
                }),
            b'o' => self
                .parse_mem_arg()
                .map(|child| GreenNode::new(IMMEDIATE, [child.into()])),
            b'c' | b'e' | b'f' | b'i' | b'n' | b's' => self
                .lexer
                .eat(FLOAT)
                .inspect(|token| {
                    if token.kind == ERROR {
                        self.report_error_token(token, Message::Description("invalid float literal"));
                    }
                })
                .map(|token| GreenNode::new(IMMEDIATE, [token.into()]))
                .or_else(|| {
                    self.lexer
                        .eat(SHAPE_DESCRIPTOR)
                        .map(|token| GreenNode::new(IMMEDIATE, [token.into()]))
                })
                .or_else(|| {
                    self.try_parse(Self::parse_ref_type)
                        .map(|child| GreenNode::new(IMMEDIATE, [child.into()]))
                })
                .or_else(|| {
                    self.try_parse(Self::parse_heap_type::<true>)
                        .map(|child| GreenNode::new(IMMEDIATE, [child]))
                }),
            b'(' => self
                .try_parse(Self::parse_ref_type_detailed)
                .or_else(|| self.try_parse(Self::parse_type_use))
                .or_else(|| self.try_parse(Self::parse_on_clause))
                .map(|child| GreenNode::new(IMMEDIATE, [child.into()])),
            b'"' => self
                .lexer
                .next(STRING)
                .map(|token| GreenNode::new(IMMEDIATE, [token.into()])),
            _ => None,
        }
    }

    pub(super) fn parse_instr(&mut self) -> Option<GreenNode> {
        if self.lexer.eat(L_PAREN).is_some() {
            let mark = self.start_node();
            self.add_child(green::L_PAREN.clone());
            if !self.lexer.peek_byte()?.is_ascii_alphabetic() {
                self.parse_trivias();
            }
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
                "then" | "else" => None,
                _ => {
                    self.recognize_instr_name(token);
                    self.parse_plain_instr_folded(mark)
                }
            }
        } else {
            let mark = self.start_node();
            let checkpoint = self.checkpoint();
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
                "end" | "else" => None,
                _ => {
                    self.recognize_instr_name(token);
                    self.parse_plain_instr_sequence(mark, checkpoint)
                }
            }
        }
    }

    pub(super) fn parse_mem_arg(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        let checkpoint = self.checkpoint();
        match self.lexer.next(MEM_ARG_KEYWORD)?.text {
            "offset" => self.add_child(green::MEM_ARG_KW_OFFSET.clone()),
            "align" => self.add_child(green::MEM_ARG_KW_ALIGN.clone()),
            _ => return None,
        }

        const MSG: &str = "whitespaces or comments are not allowed inside memory argument";

        let before_trivias = self.lexer.checkpoint().at(self.source);
        if let Some(after_trivias) = self.try_parse_with_trivias(|parser| {
            let after_trivias = parser.lexer.checkpoint().at(parser.source);
            if parser.lexer.next(EQ).is_some() {
                Some(after_trivias)
            } else {
                None
            }
        }) {
            self.add_child(green::EQ.clone());
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

        if let Some((keyword, uint)) = self.lexer.look_back(checkpoint.lexer).and_then(|s| s.split_once('='))
            && let Some(uint) = helpers::parse_small_int(uint)
        {
            match keyword {
                "offset" => {
                    self.elements.truncate(checkpoint.elements);
                    green::MEM_ARG_OFFSET.get(uint).cloned()
                }
                "align" if uint <= 16 => {
                    self.elements.truncate(checkpoint.elements);
                    green::MEM_ARG_ALIGN.get(uint).cloned()
                }
                _ => Some(self.finish_node(MEM_ARG, mark)),
            }
        } else {
            Some(self.finish_node(MEM_ARG, mark))
        }
    }

    pub(super) fn parse_on_clause(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        self.lexer.next(L_PAREN)?;
        self.add_child(green::L_PAREN.clone());
        if !self.lexer.peek_byte()?.is_ascii_alphabetic() {
            self.parse_trivias();
        }
        self.lexer.keyword("on")?;
        self.add_child(green::KW_ON.clone());

        if !self.recover(Self::parse_index) {
            self.report_missing(Message::Name("tag index"));
        }

        if let Some(modifier_keyword) = self.try_parse_with_trivias(|parser| {
            parser
                .lexer
                .next(MODIFIER_KEYWORD)
                .filter(|token| token.text == "switch")
        }) {
            self.add_child(modifier_keyword);
        } else if !self.recover(Self::parse_index) {
            self.report_missing(Message::Name("label index"));
        }

        self.expect_right_paren();
        Some(self.finish_node(ON_CLAUSE, mark))
    }

    fn parse_plain_instr_folded(&mut self, mark: NodeMark) -> Option<GreenNode> {
        while let Some(node_or_token) = self.try_parse_with_trivias(|parser| {
            parser.parse_immediate().map(GreenElement::from).or_else(|| {
                parser.lexer.eat(ERROR).map(|token| {
                    parser.report_error_token(&token, Message::Description("invalid immediate"));
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

    fn parse_plain_instr_sequence(&mut self, mark: NodeMark, checkpoint: Checkpoint<'s>) -> Option<GreenNode> {
        while let Some(node) = self.try_parse_with_trivias(Self::parse_immediate) {
            self.add_child(node);
        }
        let look_back = self.lexer.look_back(checkpoint.lexer);
        if let Some((instr_name, immediate)) = look_back.and_then(|s| s.split_once(' ')) {
            match instr_name {
                "local.get" if let Some(i) = helpers::parse_small_int(immediate) => {
                    self.elements.truncate(checkpoint.elements);
                    green::LOCAL_GET.get(i).cloned()
                }
                "i32.const" if let Some(i) = helpers::parse_small_int(immediate) => {
                    self.elements.truncate(checkpoint.elements);
                    green::I32_CONST.get(i).cloned()
                }
                "local.set" if let Some(i) = helpers::parse_small_int(immediate) => {
                    self.elements.truncate(checkpoint.elements);
                    green::LOCAL_SET.get(i).cloned()
                }
                "local.tee" if let Some(i) = helpers::parse_small_int(immediate) => {
                    self.elements.truncate(checkpoint.elements);
                    green::LOCAL_TEE.get(i).cloned()
                }
                "br" if let Some(i) = helpers::parse_small_int(immediate) => {
                    self.elements.truncate(checkpoint.elements);
                    green::BR.get(i).cloned()
                }
                "br_if" if let Some(i) = helpers::parse_small_int(immediate) => {
                    self.elements.truncate(checkpoint.elements);
                    green::BR_IF.get(i).cloned()
                }
                "i32.load"
                    if let Some(("offset", uint)) = immediate.split_once('=')
                        && let Some(uint) = helpers::parse_small_int(uint) =>
                {
                    self.elements.truncate(checkpoint.elements);
                    green::I32_LOAD_OFFSET.get(uint).cloned()
                }
                "i32.store"
                    if let Some(("offset", uint)) = immediate.split_once('=')
                        && let Some(uint) = helpers::parse_small_int(uint) =>
                {
                    self.elements.truncate(checkpoint.elements);
                    green::I32_STORE_OFFSET.get(uint).cloned()
                }
                _ => Some(self.finish_node(PLAIN_INSTR, mark)),
            }
        } else if let Some(rest) = look_back.and_then(|s| s.strip_prefix("i32.")) {
            match rest {
                "add" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_ADD.clone())
                }
                "load" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_LOAD.clone())
                }
                "eq" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_EQ.clone())
                }
                "eqz" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_EQZ.clone())
                }
                "ne" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_NE.clone())
                }
                "and" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_AND.clone())
                }
                "sub" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_SUB.clone())
                }
                "gt_s" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_GT_S.clone())
                }
                "gt_u" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_GT_U.clone())
                }
                "ge_s" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_GE_S.clone())
                }
                "ge_u" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_GE_U.clone())
                }
                "lt_s" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_LT_S.clone())
                }
                "lt_u" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_LT_U.clone())
                }
                "le_s" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_LE_S.clone())
                }
                "le_u" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_LE_U.clone())
                }
                "mul" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_MUL.clone())
                }
                "div_s" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_DIV_S.clone())
                }
                "div_u" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_DIV_U.clone())
                }
                "shl" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_SHL.clone())
                }
                "shr_s" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_SHR_S.clone())
                }
                "shr_u" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_SHR_U.clone())
                }
                "or" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_OR.clone())
                }
                "xor" => {
                    self.elements.truncate(checkpoint.elements);
                    Some(green::I32_XOR.clone())
                }
                _ => Some(self.finish_node(PLAIN_INSTR, mark)),
            }
        } else {
            Some(self.finish_node(PLAIN_INSTR, mark))
        }
    }

    fn parse_then_block(&mut self) -> Option<GreenNode> {
        let mark = self.start_node();
        self.lexer.next(L_PAREN)?;
        self.add_child(green::L_PAREN.clone());
        if !self.lexer.peek_byte()?.is_ascii_alphabetic() {
            self.parse_trivias();
        }
        self.lexer.keyword("then")?;
        self.add_child(green::KW_THEN.clone());
        while self.recover(Self::parse_instr) {}
        self.expect_right_paren();
        Some(self.finish_node(BLOCK_IF_THEN, mark))
    }

    fn recognize_instr_name(&mut self, token: Token<'s>) {
        match token.text {
            "local.get" => self.add_child(green::INSTR_LOCAL_GET.clone()),
            "i32.const" => self.add_child(green::INSTR_I32_CONST.clone()),
            "i32.add" => self.add_child(green::INSTR_I32_ADD.clone()),
            "i32.load" => self.add_child(green::INSTR_I32_LOAD.clone()),
            "local.set" => self.add_child(green::INSTR_LOCAL_SET.clone()),
            "local.tee" => self.add_child(green::INSTR_LOCAL_TEE.clone()),
            "call" => self.add_child(green::INSTR_CALL.clone()),
            "br" => self.add_child(green::INSTR_BR.clone()),
            "br_if" => self.add_child(green::INSTR_BR_IF.clone()),
            "i32.store" => self.add_child(green::INSTR_I32_STORE.clone()),
            "i64.const" => self.add_child(green::INSTR_I64_CONST.clone()),
            "i64.load" => self.add_child(green::INSTR_I64_LOAD.clone()),
            "i64.store" => self.add_child(green::INSTR_I64_STORE.clone()),
            _ => {
                let token = self.intern_token(token);
                self.add_child(token);
            }
        }
    }
}
