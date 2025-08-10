use super::{node, GreenElement, Parser};
use crate::error::Message;
use rowan::{GreenNode, Language, NodeOrToken};
use wat_syntax::SyntaxKind::*;

impl Parser<'_> {
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

    pub(super) fn parse_module(&mut self) -> Option<GreenNode> {
        let mut children = Vec::with_capacity(5);
        children.push(self.lexer.next(L_PAREN)?.into());
        self.parse_trivias(&mut children);
        children.push(self.parse_keyword("module")?);
        self.lexer.top_level = false;
        self.eat(IDENT, &mut children);

        while self.recover(Self::parse_module_field, &mut children) {}
        self.expect_right_paren(&mut children);
        self.lexer.top_level = true;
        Some(node(MODULE, children))
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
            "global" => self.parse_module_field_global(children),
            "rec" => self.parse_rec_type(children),
            _ => None,
        }
    }

    fn parse_module_field_func(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        self.eat(IDENT, &mut children);

        if let Some((mut trivias, import)) = self.try_parse_with_trivias(Self::parse_import) {
            children.append(&mut trivias);
            children.push(import.into());
        } else if let Some((mut trivias, export)) = self.try_parse_with_trivias(Self::parse_export)
        {
            children.append(&mut trivias);
            children.push(export.into());
        }

        if let Some((mut trivias, type_use)) = self.try_parse_with_trivias(Self::parse_type_use) {
            children.append(&mut trivias);
            children.push(type_use.into());
        }
        while let Some((mut trivias, node)) = self.try_parse_with_trivias(Self::parse_local) {
            children.append(&mut trivias);
            children.push(node.into());
        }

        while self.recover(Self::parse_instr, &mut children) {}
        self.expect_right_paren(&mut children);
        Some(node(MODULE_FIELD_FUNC, children))
    }

    fn parse_module_field_global(&mut self, mut children: Vec<GreenElement>) -> Option<GreenNode> {
        self.eat(IDENT, &mut children);

        if let Some((mut trivias, import)) = self.try_parse_with_trivias(Self::parse_import) {
            children.append(&mut trivias);
            children.push(import.into());
        } else if let Some((mut trivias, export)) = self.try_parse_with_trivias(Self::parse_export)
        {
            children.append(&mut trivias);
            children.push(export.into());
        }

        if !self.recover(Self::parse_global_type, &mut children) {
            self.report_missing(Message::Name("global type"));
        }

        while self.recover(Self::parse_instr, &mut children) {}
        self.expect_right_paren(&mut children);
        Some(node(MODULE_FIELD_GLOBAL, children))
    }

    fn parse_module_name(&mut self) -> Option<GreenNode> {
        self.expect(STRING)
            .map(|token| node(MODULE_NAME, [token.into()]))
    }

    fn parse_name(&mut self) -> Option<GreenNode> {
        self.expect(STRING).map(|token| node(NAME, [token.into()]))
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

        while let Some((mut trivias, node)) = self.try_parse_with_trivias(Self::parse_param) {
            children.append(&mut trivias);
            children.push(node.into());
        }
        while let Some((mut trivias, node)) = self.try_parse_with_trivias(Self::parse_result) {
            children.append(&mut trivias);
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
