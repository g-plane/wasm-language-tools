use super::{
    lexer::{Checkpoint, Lexer, Token},
    GreenElement, Parser,
};
use crate::error::{Message, SyntaxError};
use wat_syntax::SyntaxKind;

impl<'s> Parser<'s> {
    #[must_use]
    pub(super) fn recover<T: Into<GreenElement>>(
        &mut self,
        parser: fn(&mut Self) -> Option<T>,
        children: &mut Vec<GreenElement>,
    ) -> bool {
        let trivias = self.parse_trivias_deferred();
        let checkpoint = self.lexer.checkpoint();
        if let Some(node_or_token) = parser(self) {
            trivias.commit(children);
            children.push(node_or_token.into());
            return true;
        }
        self.lexer.reset(checkpoint);
        if let Some(mut tokens) = self.parse_errors() {
            trivias.commit(children);
            children.append(&mut tokens);
            true
        } else {
            trivias.rollback(&mut self.lexer);
            false
        }
    }

    pub(super) fn try_parse<T>(
        &mut self,
        parser: impl FnOnce(&mut Self) -> Option<T>,
    ) -> Option<T> {
        let checkpoint = self.lexer.checkpoint();
        let result = parser(self);
        if result.is_none() {
            self.lexer.reset(checkpoint);
        }
        result
    }

    pub(super) fn expect(&mut self, kind: SyntaxKind) -> Option<Token<'s>> {
        match self.lexer.expect(kind) {
            Ok(token) => Some(token),
            Err(error) => {
                if let Some(error) = error {
                    self.errors.push(error);
                }
                None
            }
        }
    }

    /// "Eat" a token with processing the trivias before the token.
    /// After parsed, trivias and token will be added to `children`.
    pub(super) fn eat(&mut self, kind: SyntaxKind, children: &mut Vec<GreenElement>) -> bool {
        let trivias = self.parse_trivias_deferred();
        if let Some(token) = self.lexer.eat(kind) {
            trivias.commit(children);
            children.push(token.into());
            true
        } else {
            trivias.rollback(&mut self.lexer);
            false
        }
    }

    pub(super) fn parse_keyword(&mut self, keyword: &'static str) -> Option<GreenElement> {
        self.lexer
            .next(SyntaxKind::KEYWORD)
            .filter(|token| token.text == keyword)
            .map(GreenElement::from)
    }

    /// Accept error tokens with parens and trivias.
    fn parse_errors(&mut self) -> Option<Vec<GreenElement>> {
        if let Some(mut token) = self.lexer.eat(SyntaxKind::L_PAREN) {
            token.kind = SyntaxKind::ERROR;
            let mut tokens = vec![token.into()];
            let mut stack = 1u16;
            loop {
                let trivias = self.parse_trivias_deferred();
                if let Some(mut token) = self.lexer.eat(SyntaxKind::L_PAREN) {
                    trivias.commit(&mut tokens);
                    token.kind = SyntaxKind::ERROR;
                    tokens.push(token.into());
                    stack += 1;
                } else if let Some(mut token) = self.lexer.eat(SyntaxKind::R_PAREN) {
                    trivias.commit(&mut tokens);
                    token.kind = SyntaxKind::ERROR;
                    tokens.push(token.into());
                    stack -= 1;
                    if stack == 0 {
                        break;
                    }
                } else if let Some(token) = self.lexer.eat(SyntaxKind::ERROR) {
                    trivias.commit(&mut tokens);
                    self.report_error_token(&token, Message::Description("unexpected token"));
                    tokens.push(token.into());
                } else {
                    trivias.rollback(&mut self.lexer);
                    break;
                }
            }
            Some(tokens)
        } else {
            self.lexer.eat(SyntaxKind::ERROR).map(|token| {
                self.report_error_token(&token, Message::Description("unexpected token"));
                vec![token.into()]
            })
        }
    }

    pub(super) fn parse_trivias(&mut self, children: &mut Vec<GreenElement>) {
        while let Some(token) = self.lexer.trivia() {
            children.push(token.into());
        }
    }

    fn parse_trivias_deferred(&mut self) -> Trivias<'s> {
        let checkpoint = self.lexer.checkpoint();
        let mut tokens = Vec::new();
        while let Some(token) = self.lexer.trivia() {
            tokens.push(token);
        }
        Trivias { tokens, checkpoint }
    }

    pub(super) fn expect_right_paren(&mut self, children: &mut Vec<GreenElement>) {
        loop {
            let checkpoint = self.lexer.checkpoint();
            let trivias = self.parse_trivias_deferred();
            if let Some(token) = self.lexer.next(SyntaxKind::R_PAREN) {
                trivias.commit(children);
                children.push(token.into());
                return;
            }

            self.lexer.reset(checkpoint);
            if let Some(token) = self.lexer.peek(SyntaxKind::L_PAREN) {
                trivias.rollback(&mut self.lexer);
                // Unlike using `report_missing`,
                // we expect right paren immediately after rolling back, even there're trivias.
                let start = token.text.as_ptr().addr() - self.source.as_ptr().addr();
                self.errors.push(SyntaxError {
                    start,
                    end: start + 1,
                    message: Message::Char(')'),
                });
                return;
            }
            if let Some(mut tokens) = self.parse_errors() {
                trivias.commit(children);
                children.append(&mut tokens);
            } else {
                trivias.rollback(&mut self.lexer);
                let start = self.source.len();
                self.errors.push(SyntaxError {
                    start,
                    end: start,
                    message: Message::Char(')'),
                });
                return;
            }
        }
    }

    pub(super) fn report_missing(&mut self, message: Message) {
        let checkpoint = self.lexer.checkpoint();
        while self.lexer.trivia().is_some() {}

        if let Some(token) = self
            .lexer
            .peek(SyntaxKind::R_PAREN)
            .or_else(|| self.lexer.peek(SyntaxKind::L_PAREN))
            .or_else(|| self.lexer.peek(SyntaxKind::ERROR))
        {
            self.report_error_token(&token, message);
        } else {
            let start = self.source.len();
            self.errors.push(SyntaxError {
                start,
                end: start,
                message,
            });
        }
        self.lexer.reset(checkpoint);
    }

    pub(super) fn report_error_token(&mut self, token: &Token<'s>, message: Message) {
        let start = token.text.as_ptr().addr() - self.source.as_ptr().addr();
        self.errors.push(SyntaxError {
            start,
            end: start + token.text.len(),
            message,
        });
    }
}

#[derive(Debug)]
pub(super) struct Trivias<'s> {
    tokens: Vec<Token<'s>>,
    checkpoint: Checkpoint<'s>,
}
impl<'s> Trivias<'s> {
    pub(super) fn commit(self, children: &mut Vec<GreenElement>) {
        children.reserve(self.tokens.len() + 1);
        children.extend(self.tokens.into_iter().map(GreenElement::from));
    }
    pub(super) fn rollback(self, lexer: &mut Lexer<'s>) {
        lexer.reset(self.checkpoint);
    }
}
