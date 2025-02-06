use crate::parser::Input;
use std::fmt;
use winnow::{
    error::{AddContext, FromRecoverableError, ParserError},
    stream::{Location, Stream},
};

#[derive(Clone, Debug, PartialEq, Eq)]
/// The syntax error comes with location and message.
pub struct SyntaxError {
    pub start: usize,
    pub end: usize,
    pub message: Message,
}

impl SyntaxError {
    fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            message: Message::Description("<unknown syntax error>"),
        }
    }
}

impl FromRecoverableError<Input<'_>, SyntaxError> for SyntaxError {
    fn from_recoverable_error(
        _token_start: &<Input as Stream>::Checkpoint,
        _err_start: &<Input as Stream>::Checkpoint,
        input: &Input,
        mut e: SyntaxError,
    ) -> Self {
        e.end = input.current_token_start();
        e
    }
}

impl AddContext<Input<'_>, Message> for SyntaxError {
    fn add_context(
        mut self,
        input: &Input,
        _token_start: &<Input as Stream>::Checkpoint,
        message: Message,
    ) -> Self {
        self.start = input.current_token_start();
        self.message = message;
        self
    }
}

impl ParserError<Input<'_>> for SyntaxError {
    type Inner = Self;

    fn from_input(_input: &Input<'_>) -> Self {
        SyntaxError::new()
    }

    fn into_inner(self) -> Result<Self::Inner, Self> {
        Ok(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Syntax error message.
pub enum Message {
    Char(char),
    Str(&'static str),
    Name(&'static str),
    Description(&'static str),
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Char(c) => write!(f, "expected `{c}`"),
            Self::Str(c) => write!(f, "expected `{c}`"),
            Self::Name(c) => write!(f, "expected {c}"),
            Self::Description(c) => c.fmt(f),
        }
    }
}
