use super::{is_id_char, GreenElement};
use crate::error::{Message, SyntaxError};
use rowan::{GreenToken, TextRange, TextSize};
use wat_syntax::SyntaxKind;

#[derive(Clone, Debug)]
pub(super) struct Token<'s> {
    pub kind: SyntaxKind,
    pub text: &'s str,
}
impl From<Token<'_>> for GreenElement {
    fn from(token: Token<'_>) -> Self {
        GreenElement::Token(GreenToken::new(token.kind.into(), token.text))
    }
}

#[derive(Debug)]
pub(super) struct Lexer<'s> {
    source: &'s str,
    input: &'s str,
    pub top_level: bool,
}

impl<'s> Lexer<'s> {
    pub fn new(source: &'s str) -> Self {
        Lexer {
            source,
            input: source,
            top_level: true,
        }
    }

    pub fn checkpoint(&self) -> Checkpoint<'s> {
        Checkpoint(self.input)
    }
    pub fn reset(&mut self, checkpoint: Checkpoint<'s>) {
        self.input = checkpoint.0;
    }

    /// Move and advance the lexer for single token. If it doesn't match, raise a syntax error.
    pub fn expect(&mut self, kind: SyntaxKind) -> Result<Token<'s>, Option<SyntaxError>> {
        self.eat(kind).ok_or_else(|| {
            self.error().map(|text| {
                let message = match kind {
                    SyntaxKind::L_PAREN => Message::Char('('),
                    SyntaxKind::R_PAREN => Message::Char(')'),
                    SyntaxKind::KEYWORD => Message::Name("keyword"),
                    SyntaxKind::INSTR_NAME => Message::Name("instruction name"),
                    SyntaxKind::TYPE_KEYWORD => Message::Name("type keyword"),
                    SyntaxKind::IDENT => Message::Name("identifier"),
                    SyntaxKind::STRING => Message::Name("string"),
                    SyntaxKind::INT => Message::Name("integer"),
                    SyntaxKind::UNSIGNED_INT => Message::Name("unsigned integer"),
                    SyntaxKind::FLOAT => Message::Name("float"),
                    SyntaxKind::EQ => Message::Char('='),
                    SyntaxKind::MEM_ARG_KEYWORD => Message::Name("memory argument keyword"),
                    _ => unreachable!(),
                };
                let origin = self.source.as_ptr().addr();
                SyntaxError {
                    range: TextRange::new(
                        TextSize::new((text.as_ptr().addr() - origin) as u32),
                        TextSize::new((self.input.as_ptr().addr() - origin) as u32),
                    ),
                    message,
                }
            })
        })
    }

    /// Move and advance the lexer for single token if it matches the given kind, otherwise don't advance.
    pub fn eat(&mut self, kind: SyntaxKind) -> Option<Token<'s>> {
        let checkpoint = self.input;
        let token = self.next(kind);
        if token.is_none() {
            self.input = checkpoint;
        }
        token
    }

    /// Preview next token without advancing the lexer.
    pub fn peek(&mut self, kind: SyntaxKind) -> Option<Token<'s>> {
        let checkpoint = self.input;
        while self.trivia().is_some() {}
        let token = self.next(kind);
        self.input = checkpoint;
        token
    }

    pub fn next(&mut self, kind: SyntaxKind) -> Option<Token<'s>> {
        match kind {
            SyntaxKind::L_PAREN => self.ascii_char::<b'('>(SyntaxKind::L_PAREN),
            SyntaxKind::R_PAREN => self.ascii_char::<b')'>(SyntaxKind::R_PAREN),
            SyntaxKind::KEYWORD | SyntaxKind::INSTR_NAME | SyntaxKind::TYPE_KEYWORD => {
                self.word().map(|text| Token { kind, text })
            }
            SyntaxKind::IDENT => self.ident(),
            SyntaxKind::STRING => self.string(),
            SyntaxKind::INT => self.int(),
            SyntaxKind::UNSIGNED_INT => self.unsigned_int(),
            SyntaxKind::FLOAT => self.float(),
            SyntaxKind::EQ => self.ascii_char::<b'='>(SyntaxKind::EQ),
            SyntaxKind::MEM_ARG_KEYWORD => self.mem_arg_keyword(),
            SyntaxKind::ERROR => self.error().map(|text| Token {
                kind: SyntaxKind::ERROR,
                text,
            }),
            _ => unreachable!(),
        }
    }

    pub fn keyword(&mut self, keyword: &'static str) -> Option<Token<'s>> {
        debug_assert!(keyword.chars().all(|c| c.is_ascii_lowercase()));
        let bytes = self.input.as_bytes();
        if bytes.starts_with(keyword.as_bytes())
            && bytes
                .get(keyword.len())
                .is_none_or(|b| !is_id_char(*b as char))
        {
            // SAFETY: `keyword` is a static ASCII string in UTF-8
            unsafe {
                Some(Token {
                    kind: SyntaxKind::KEYWORD,
                    text: self.split_advance(keyword.len()),
                })
            }
        } else {
            None
        }
    }

    fn ascii_char<const C: u8>(&mut self, kind: SyntaxKind) -> Option<Token<'s>> {
        if self.input.starts_with(C as char) {
            // SAFETY: `C` is an ASCII char
            unsafe {
                Some(Token {
                    kind,
                    text: self.split_advance(1),
                })
            }
        } else {
            None
        }
    }

    fn ident(&mut self) -> Option<Token<'s>> {
        if self.input.starts_with('$') {
            let end = self
                .input
                .find(|c| !is_id_char(c))
                .unwrap_or(self.input.len());
            if end == 1 {
                // identifier can't be only `$`
                None
            } else {
                // SAFETY: the `find` result or the length of the input is guaranteed to be valid UTF-8 boundary
                unsafe {
                    Some(Token {
                        kind: SyntaxKind::IDENT,
                        text: self.split_advance(end),
                    })
                }
            }
        } else {
            None
        }
    }

    fn word(&mut self) -> Option<&'s str> {
        if self.input.starts_with(|c: char| c.is_ascii_lowercase()) {
            let end = self
                .input
                .find(|c| !is_id_char(c))
                .unwrap_or(self.input.len());
            // SAFETY: the `find` result or the length of the input is guaranteed to be valid UTF-8 boundary
            unsafe { Some(self.split_advance(end)) }
        } else {
            None
        }
    }

    fn string(&mut self) -> Option<Token<'s>> {
        if self.input.starts_with('"') {
            let mut bytes = self.input.get(1..)?.bytes().enumerate();
            loop {
                match bytes.next() {
                    Some((end, b'"')) => {
                        // SAFETY: `"` is ASCII char, and `+ 2` means it contains the quotes
                        unsafe {
                            return Some(Token {
                                kind: SyntaxKind::STRING,
                                text: self.split_advance(end + 2),
                            });
                        }
                    }
                    Some((_, b'\\')) => {
                        bytes.next();
                    }
                    Some((_, b'\n' | b'\r')) | None => return None,
                    _ => {}
                }
            }
        } else {
            None
        }
    }

    fn int(&mut self) -> Option<Token<'s>> {
        let checkpoint = self.input;
        if let Some(rest) = self.input.strip_prefix(['-', '+']) {
            self.input = rest;
        }
        self.unsigned_int_raw()?;
        // SAFETY: the difference of two valid UTF-8 strings is valid
        unsafe {
            Some(Token {
                kind: SyntaxKind::INT,
                text: checkpoint.get_unchecked(..checkpoint.len() - self.input.len()),
            })
        }
    }

    fn unsigned_int(&mut self) -> Option<Token<'s>> {
        self.unsigned_int_raw().map(|text| Token {
            kind: SyntaxKind::UNSIGNED_INT,
            text,
        })
    }

    fn unsigned_int_raw(&mut self) -> Option<&'s str> {
        let checkpoint = self.input;
        let text = if let Some(rest) = self.input.strip_prefix("0x") {
            self.input = rest;
            self.unsigned_hex::<true>()?;
            // SAFETY: the difference of two valid UTF-8 strings is valid
            unsafe { Some(checkpoint.get_unchecked(..checkpoint.len() - self.input.len())) }
        } else {
            self.unsigned_dec::<true>()
        };
        if self.input.starts_with(is_id_char) {
            None
        } else {
            text
        }
    }

    fn unsigned_dec<const VALIDATE: bool>(&mut self) -> Option<&'s str> {
        if self.input.starts_with(|c: char| c.is_ascii_digit()) {
            let end = self
                .input
                .find(|c: char| !c.is_ascii_digit() && c != '_')
                .unwrap_or(self.input.len());
            // SAFETY: the `find` result or the length of the input is guaranteed to be valid UTF-8 boundary
            let text = unsafe { self.split_advance(end) };
            if VALIDATE {
                let mut bytes = text.bytes();
                while let Some(b) = bytes.next() {
                    if b == b'_' && !bytes.next().is_some_and(|b| b.is_ascii_digit()) {
                        return None;
                    }
                }
            }
            Some(text)
        } else {
            None
        }
    }

    fn unsigned_hex<const VALIDATE: bool>(&mut self) -> Option<&'s str> {
        if self.input.starts_with(|c: char| c.is_ascii_hexdigit()) {
            let end = self
                .input
                .find(|c: char| !c.is_ascii_hexdigit() && c != '_')
                .unwrap_or(self.input.len());
            // SAFETY: the `find` result or the length of the input is guaranteed to be valid UTF-8 boundary
            let text = unsafe { self.split_advance(end) };
            if VALIDATE {
                let mut bytes = text.bytes();
                while let Some(b) = bytes.next() {
                    if b == b'_' && !bytes.next().is_some_and(|b| b.is_ascii_hexdigit()) {
                        return None;
                    }
                }
            }
            Some(text)
        } else {
            None
        }
    }

    fn float(&mut self) -> Option<Token<'s>> {
        let mut valid = true;
        let checkpoint = self.input;
        if let Some(rest) = self.input.strip_prefix(['-', '+']) {
            self.input = rest;
        }
        if let Some(rest) = self.input.strip_prefix("0x") {
            self.input = rest;
            valid &= self.unsigned_hex::<true>().is_some();
            if let Some(rest) = self.input.strip_prefix('.') {
                self.input = rest;
                if rest.starts_with(|c: char| c.is_ascii_hexdigit()) {
                    valid &= self.unsigned_hex::<true>().is_some();
                }
            }
            if let Some(rest) = self.input.strip_prefix(['p', 'P']) {
                self.input = rest.strip_prefix(['-', '+']).unwrap_or(rest);
                valid &= self
                    .unsigned_hex::<true>()
                    .filter(|text| text.bytes().all(|b| b.is_ascii_digit() || b == b'_'))
                    .is_some();
            }
        } else if self.input.starts_with(|c: char| c.is_ascii_digit()) {
            valid &= self.unsigned_dec::<true>().is_some();
            if let Some(rest) = self.input.strip_prefix('.') {
                self.input = rest;
                if rest.starts_with(|c: char| c.is_ascii_digit()) {
                    valid &= self.unsigned_dec::<true>().is_some();
                }
            }
            if let Some(rest) = self.input.strip_prefix(['e', 'E']) {
                self.input = rest.strip_prefix(['-', '+']).unwrap_or(rest);
                valid &= self.unsigned_dec::<true>().is_some();
            }
        } else if let Some(rest) = self
            .input
            .strip_prefix("inf")
            .filter(|rest| !rest.starts_with(is_id_char))
        {
            self.input = rest;
        } else if let Some(rest) = self
            .input
            .strip_prefix("nan")
            .filter(|rest| rest.starts_with(":0x") || !rest.starts_with(is_id_char))
        {
            self.input = rest;
            if let Some(rest) = rest.strip_prefix(":0x") {
                self.input = rest;
                valid &= self.unsigned_hex::<true>().is_some();
            }
        } else {
            return None;
        }
        if self.input.starts_with(is_id_char) {
            None
        } else {
            // SAFETY: the difference of two valid UTF-8 strings is valid
            let text = unsafe { checkpoint.get_unchecked(..checkpoint.len() - self.input.len()) };
            Some(Token {
                kind: if valid {
                    SyntaxKind::FLOAT
                } else {
                    SyntaxKind::ERROR
                },
                text,
            })
        }
    }

    fn mem_arg_keyword(&mut self) -> Option<Token<'s>> {
        let checkpoint = self.input;
        self.input = self
            .input
            .strip_prefix("offset")
            .or_else(|| self.input.strip_prefix("align"))?;
        if self.input.starts_with(|c: char| c.is_ascii_alphabetic()) {
            None
        } else {
            checkpoint
                .get(..checkpoint.len() - self.input.len())
                .map(|text| Token {
                    kind: SyntaxKind::MEM_ARG_KEYWORD,
                    text,
                })
        }
    }

    fn error(&mut self) -> Option<&'s str> {
        let mut chars = self.input.chars().peekable();
        match chars.next()? {
            ' ' | '\n' | '\t' | '\r' | '(' => None,
            ';' if matches!(chars.peek(), Some(';')) => None,
            ')' if !self.top_level => None,
            c if is_id_char(c) => {
                let end = self
                    .input
                    .find(|c| !is_id_char(c))
                    .unwrap_or(self.input.len());
                // SAFETY: the `find` result or the length of the input is guaranteed to be valid UTF-8 boundary
                unsafe { Some(self.split_advance(end)) }
            }
            '"' => {
                let checkpoint = self.input;
                let _ = self.string();
                checkpoint.get(..checkpoint.len() - self.input.len())
            }
            c => {
                // SAFETY: using the length in UTF-8
                unsafe { Some(self.split_advance(c.len_utf8())) }
            }
        }
    }

    pub fn trivia(&mut self) -> Option<Token<'s>> {
        let bytes = self.input.as_bytes();
        bytes.first().and_then(|b| match b {
            b' ' | b'\n' | b'\t' | b'\r' => {
                let end = self
                    .input
                    .bytes()
                    .position(|b| !matches!(b, b' ' | b'\n' | b'\t' | b'\r'))
                    .unwrap_or(self.input.len());
                // SAFETY: the `find` result or the length of the input is guaranteed to be valid UTF-8 boundary
                unsafe {
                    Some(Token {
                        kind: SyntaxKind::WHITESPACE,
                        text: self.split_advance(end),
                    })
                }
            }
            b'(' if matches!(bytes.get(1), Some(b';')) => {
                let checkpoint = self.input;
                let mut stack = 0u8;
                while !self.input.is_empty() {
                    match self.input.as_bytes() {
                        [b'(', b';', ..] => {
                            stack += 1;
                            self.input = self.input.get(2..)?;
                        }
                        [b';', b')', ..] => {
                            stack -= 1;
                            self.input = self.input.get(2..)?;
                            if stack == 0 {
                                break;
                            }
                        }
                        [b';' | b'(', ..] => {
                            self.input = self.input.get(1..)?;
                        }
                        _ => break,
                    }
                    (_, self.input) = self
                        .input
                        .split_at(self.input.find([';', '(']).unwrap_or(self.input.len()));
                }
                Some(Token {
                    kind: SyntaxKind::BLOCK_COMMENT,
                    text: checkpoint.get(..checkpoint.len() - self.input.len())?,
                })
            }
            b';' if matches!(bytes.get(1), Some(b';')) => {
                let end = self.input.find('\n').unwrap_or(self.input.len());
                // SAFETY: the `find` result or the length of the input is guaranteed to be valid UTF-8 boundary
                unsafe {
                    Some(Token {
                        kind: SyntaxKind::LINE_COMMENT,
                        text: self.split_advance(end),
                    })
                }
            }
            _ => None,
        })
    }

    unsafe fn split_advance(&mut self, mid: usize) -> &'s str {
        let left = self.input.get_unchecked(0..mid);
        self.input = self.input.get_unchecked(mid..);
        left
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct Checkpoint<'s>(&'s str);
impl<'s> Checkpoint<'s> {
    pub fn at(&self, source: &'s str) -> TextSize {
        TextSize::new((self.0.as_ptr().addr() - source.as_ptr().addr()) as u32)
    }
}
