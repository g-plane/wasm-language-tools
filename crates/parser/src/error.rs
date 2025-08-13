use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
/// The syntax error comes with location and message.
pub struct SyntaxError {
    pub start: usize,
    pub end: usize,
    pub message: Message,
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
