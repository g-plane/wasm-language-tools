use crate::parser::Input;
use winnow::{
    error::{AddContext, FromRecoverableError, ParserError, StrContext},
    stream::{Location, Stream},
};

#[derive(Clone, Debug)]
/// The syntax error comes with location and message.
pub struct SyntaxError {
    pub start: usize,
    pub end: usize,
    pub message: StrContext,
}

impl SyntaxError {
    fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            message: StrContext::Label("<unknown syntax error>"),
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
        e.end = input.location();
        e
    }
}

impl AddContext<Input<'_>, StrContext> for SyntaxError {
    fn add_context(
        mut self,
        input: &Input,
        _token_start: &<Input as Stream>::Checkpoint,
        context: StrContext,
    ) -> Self {
        self.start = input.location();
        self.message = context;
        self
    }
}

impl ParserError<Input<'_>> for SyntaxError {
    fn from_error_kind(_input: &Input, _kind: winnow::error::ErrorKind) -> Self {
        SyntaxError::new()
    }

    fn append(
        self,
        _input: &Input,
        _token_start: &<Input as Stream>::Checkpoint,
        _kind: winnow::error::ErrorKind,
    ) -> Self {
        self
    }
}
