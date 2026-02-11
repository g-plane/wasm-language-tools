use super::{GreenElement, Parser, green, lexer::Token};
use crate::error::{Message, SyntaxError};
use wat_syntax::{SyntaxKind, TextRange, TextSize};

impl<'s> Parser<'s> {
    #[must_use]
    pub(super) fn recover<T: Into<GreenElement>>(&mut self, parser: fn(&mut Self) -> Option<T>) -> bool {
        let checkpoint_before_trivias = self.checkpoint();
        self.parse_trivias();
        let checkpoint_after_trivias = self.checkpoint();
        if let Some(node_or_token) = parser(self) {
            self.add_child(node_or_token);
            return true;
        }
        self.reset(checkpoint_after_trivias);
        if self.parse_errors() {
            true
        } else {
            self.reset(checkpoint_before_trivias);
            false
        }
    }

    pub(super) fn retry<T: Into<GreenElement>>(&mut self, parser: fn(&mut Self) -> Option<T>) -> bool {
        loop {
            let checkpoint_before_trivias = self.checkpoint();
            self.parse_trivias();
            let checkpoint_after_trivias = self.checkpoint();
            if let Some(node_or_token) = parser(self) {
                self.add_child(node_or_token);
                return true;
            }
            self.reset(checkpoint_after_trivias);
            if let Some(token) = self.lexer.eat(SyntaxKind::ERROR) {
                self.report_error_token(&token, Message::UnexpectedToken);
                self.add_child(token);
            } else {
                self.reset(checkpoint_before_trivias);
                return false;
            }
        }
    }

    pub(super) fn try_parse<T>(&mut self, parser: impl FnOnce(&mut Self) -> Option<T>) -> Option<T> {
        let checkpoint = self.checkpoint();
        let result = parser(self);
        if result.is_none() {
            self.reset(checkpoint);
        }
        result
    }
    pub(super) fn try_parse_with_trivias<T>(&mut self, parser: impl FnOnce(&mut Self) -> Option<T>) -> Option<T> {
        let checkpoint = self.checkpoint();
        self.parse_trivias();
        match parser(self) {
            Some(result) => Some(result),
            None => {
                self.reset(checkpoint);
                None
            }
        }
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
    pub(super) fn eat(&mut self, kind: SyntaxKind) -> bool {
        let checkpoint = self.checkpoint();
        self.parse_trivias();
        if let Some(token) = self.lexer.eat(kind) {
            self.add_child(token);
            true
        } else {
            self.reset(checkpoint);
            false
        }
    }

    /// Accept error tokens with parens and trivias.
    fn parse_errors(&mut self) -> bool {
        if let Some(mut token) = self.lexer.eat(SyntaxKind::L_PAREN) {
            token.kind = SyntaxKind::ERROR;
            self.add_child(token);
            let mut stack = 1u16;
            loop {
                let checkpoint = self.checkpoint();
                self.parse_trivias();
                if let Some(mut token) = self.lexer.eat(SyntaxKind::L_PAREN) {
                    token.kind = SyntaxKind::ERROR;
                    self.report_error_token(&token, Message::UnexpectedToken);
                    self.add_child(token);
                    stack += 1;
                } else if let Some(mut token) = self.lexer.eat(SyntaxKind::R_PAREN) {
                    token.kind = SyntaxKind::ERROR;
                    self.report_error_token(&token, Message::UnexpectedToken);
                    self.add_child(token);
                    stack -= 1;
                    if stack == 0 {
                        break;
                    }
                } else if let Some(token) = self.lexer.eat(SyntaxKind::ERROR) {
                    self.report_error_token(&token, Message::UnexpectedToken);
                    self.add_child(token);
                } else {
                    self.reset(checkpoint);
                    break;
                }
            }
            true
        } else if let Some(token) = self.lexer.eat(SyntaxKind::ERROR) {
            self.report_error_token(&token, Message::UnexpectedToken);
            self.add_child(token);
            true
        } else {
            false
        }
    }

    pub(super) fn parse_trivias(&mut self) {
        while let Some(token) = self.lexer.trivia() {
            if token.kind == SyntaxKind::WHITESPACE && token.text.as_bytes() == [b' '] {
                self.add_child(green::SINGLE_SPACE.clone());
            } else {
                self.add_child(token);
            }
        }
    }

    pub(super) fn expect_right_paren(&mut self) {
        loop {
            let checkpoint = self.checkpoint();
            self.parse_trivias();
            if self.lexer.next(SyntaxKind::R_PAREN).is_some() {
                self.add_child(green::R_PAREN.clone());
                return;
            }
            if let Some(token) = self.lexer.peek(SyntaxKind::L_PAREN) {
                // a trick:
                // if there're newlines before next left paren, we should exit from current parsing node
                if self
                    .elements
                    .get(checkpoint.elements..)
                    .into_iter()
                    .flat_map(|slice| slice.iter())
                    .any(|node_or_token| {
                        if let GreenElement::Token(token) = node_or_token {
                            token.text().contains('\n')
                        } else {
                            false
                        }
                    })
                {
                    self.reset(checkpoint);
                    self.report_error_token(&token, Message::Char(')'));
                    return;
                }
            }
            if !self.parse_errors() {
                self.reset(checkpoint);
                let start = self.source.len();
                self.errors.push(SyntaxError {
                    range: TextRange::new(TextSize::new(start as u32), TextSize::new(start as u32)),
                    message: Message::Char(')'),
                });
                return;
            }
        }
    }

    pub(super) fn report_missing(&mut self, message: Message) {
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
                range: TextRange::new(TextSize::new(start as u32), TextSize::new(start as u32)),
                message,
            });
        }
    }

    pub(super) fn report_error_token(&mut self, token: &Token<'s>, message: Message) {
        let start = token.text.as_ptr().addr() - self.source.as_ptr().addr();
        let range = TextRange::new(
            TextSize::new(start as u32),
            TextSize::new((start + token.text.len()) as u32),
        );
        if self
            .errors
            .last()
            .is_none_or(|error| error.range != range || message != Message::UnexpectedToken)
        {
            self.errors.push(SyntaxError { range, message });
        }
    }
}
