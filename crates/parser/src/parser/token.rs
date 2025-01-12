use super::{tok, GreenElement, GreenResult, Input};
use crate::error::{Message, SyntaxError};
use wat_syntax::SyntaxKind::*;
use winnow::{
    ascii::{hex_digit0, line_ending, multispace1, take_escaped, till_line_ending},
    combinator::{alt, dispatch, empty, eof, fail, not, opt, peek, preceded, repeat, repeat_till},
    error::{ErrMode, FromRecoverableError},
    stream::{AsChar, Recover, Stream},
    token::{any, none_of, one_of, take_till, take_until, take_while},
    PResult, Parser,
};

pub(super) fn l_paren(input: &mut Input) -> GreenResult {
    '('.map(|_| tok(L_PAREN, "("))
        .context(Message::Char('('))
        .parse_next(input)
}

pub(super) fn r_paren(input: &mut Input) -> PResult<Option<Vec<GreenElement>>, SyntaxError> {
    let mut parser = ')'.context(Message::Char(')'));
    let mut error_token_parser =
        error_token(false).context(Message::Description("unexpected token"));
    let mut tokens = Vec::with_capacity(1);
    let start = input.checkpoint();
    loop {
        let mut trivia_tokens = trivias.parse_next(input)?;
        let token_start = input.checkpoint();
        let mut err = match parser.parse_next(input) {
            Ok(..) => {
                tokens.append(&mut trivia_tokens);
                tokens.push(tok(R_PAREN, ")"));
                return Ok(Some(tokens));
            }
            Err(ErrMode::Incomplete(e)) => return Err(ErrMode::Incomplete(e)),
            Err(err) => err,
        };
        input.reset(&token_start);
        let err_start = input.checkpoint();
        let err_start_eof_offset = input.eof_offset();
        if let Ok(error_token) = error_token_parser.parse_next(input) {
            let i_eof_offset = input.eof_offset();
            if err_start_eof_offset == i_eof_offset {
                // didn't advance so bubble the error up
            } else if let Err(err_) = input.record_err(&token_start, &err_start, err) {
                err = err_;
            } else {
                tokens.append(&mut trivia_tokens);
                tokens.push(error_token);
                continue;
            }
        } else if let Err(err_) = input.record_err(&token_start, &err_start, err) {
            err = err_;
        } else {
            input.reset(&start);
            return Ok(None);
        }

        input.reset(&start);
        err = err
            .map(|err| SyntaxError::from_recoverable_error(&token_start, &err_start, input, err));
        return Err(err);
    }
}

pub(super) fn word<'s>(input: &mut Input<'s>) -> PResult<&'s str, SyntaxError> {
    (
        one_of(|c: char| c.is_ascii_lowercase()),
        take_while(0.., is_id_char),
    )
        .take()
        .parse_next(input)
}

pub(super) fn trivias(input: &mut Input) -> PResult<Vec<GreenElement>, SyntaxError> {
    repeat(0.., alt((ws, line_comment, block_comment))).parse_next(input)
}

pub(super) fn trivias_prefixed<'s, P>(
    parser: P,
) -> impl Parser<Input<'s>, Vec<GreenElement>, SyntaxError>
where
    P: Parser<Input<'s>, GreenElement, SyntaxError>,
{
    (trivias, parser).map(|(mut trivias, element)| {
        trivias.push(element);
        trivias
    })
}

pub(super) fn ws(input: &mut Input) -> GreenResult {
    multispace1
        .parse_next(input)
        .map(|text| tok(WHITESPACE, text))
}

pub(super) fn line_comment(input: &mut Input) -> GreenResult {
    (";;", till_line_ending)
        .take()
        .parse_next(input)
        .map(|text| tok(LINE_COMMENT, text))
}

pub(super) fn block_comment(input: &mut Input) -> GreenResult {
    block_comment_impl
        .parse_next(input)
        .map(|text| tok(BLOCK_COMMENT, text))
}
fn block_comment_impl<'s>(input: &mut Input<'s>) -> PResult<&'s str, SyntaxError> {
    (
        "(;",
        repeat_till::<_, _, (), _, _, _, _>(
            0..,
            (
                take_until(0.., ("(;", ";)")),
                dispatch! {peek(opt("(;"));
                    Some(..) => opt(block_comment_impl).void(),
                    None => empty,
                },
            ),
            ";)",
        ),
    )
        .take()
        .parse_next(input)
}

pub(super) fn keyword<'s>(
    keyword: &'static str,
) -> impl Parser<Input<'s>, GreenElement, SyntaxError> {
    word.verify(move |word: &str| word == keyword)
        .map(|text| tok(KEYWORD, text))
        .context(Message::Str(keyword))
}

pub(super) fn ident(input: &mut Input) -> GreenResult {
    ident_impl.parse_next(input).map(|text| tok(IDENT, text))
}
fn ident_impl<'s>(input: &mut Input<'s>) -> PResult<&'s str, SyntaxError> {
    ('$', take_while(1.., is_id_char))
        .take()
        .context(Message::Name("identifier"))
        .parse_next(input)
}

#[inline]
/// Checks if a character is a valid identifier character.
///
/// ## Examples
///
/// ```
/// # use wat_parser::is_id_char;
/// assert!(is_id_char('a'));
/// assert!(is_id_char('Z'));
/// assert!(is_id_char('0'));
/// assert!(is_id_char('$'));
/// assert!(is_id_char('.'));
/// assert!(!is_id_char('('));
/// assert!(!is_id_char(')'));
/// ```
pub fn is_id_char(c: char) -> bool {
    c.is_ascii_alphanumeric()
        || c.is_ascii_punctuation()
            && !matches!(
                c,
                '\'' | '"' | ',' | ';' | '(' | ')' | '[' | ']' | '{' | '}'
            )
}

pub(super) fn string(input: &mut Input) -> GreenResult {
    string_impl
        .context(Message::Name("string literal"))
        .parse_next(input)
        .map(|text| tok(STRING, text))
}
fn string_impl<'s>(input: &mut Input<'s>) -> PResult<&'s str, SyntaxError> {
    (
        '"',
        take_escaped(
            none_of(['"', '\\', '\n', '\r']),
            '\\',
            dispatch! {any;
                'u' => ('{', hex_digit0, '}').void(),
                _ => empty,
            },
        ),
        alt(("\"", peek(line_ending), eof)),
    )
        .take()
        .parse_next(input)
}

pub(super) fn int(input: &mut Input) -> GreenResult {
    int_impl.parse_next(input).map(|text| tok(INT, text))
}
fn int_impl<'s>(input: &mut Input<'s>) -> PResult<&'s str, SyntaxError> {
    (
        opt(one_of(['+', '-'])),
        unsigned_int_impl,
        peek(none_of(is_id_char)),
    )
        .take()
        .parse_next(input)
}

pub(super) fn unsigned_int(input: &mut Input) -> GreenResult {
    unsigned_int_impl
        .parse_next(input)
        .map(|text| tok(UNSIGNED_INT, text))
}
pub(super) fn unsigned_int_impl<'s>(input: &mut Input<'s>) -> PResult<&'s str, SyntaxError> {
    dispatch! {opt("0x");
        Some(..) => unsigned_hex,
        None => unsigned_dec,
    }
    .take()
    .verify(|text: &str| !text.ends_with('_'))
    .parse_next(input)
}
fn unsigned_hex<'s>(input: &mut Input<'s>) -> PResult<&'s str, SyntaxError> {
    (
        one_of(AsChar::is_hex_digit),
        take_while(0.., |c: char| c.is_ascii_hexdigit() || c == '_'),
    )
        .take()
        .parse_next(input)
}
fn unsigned_dec<'s>(input: &mut Input<'s>) -> PResult<&'s str, SyntaxError> {
    (
        one_of(AsChar::is_dec_digit),
        take_while(0.., |c: char| c.is_ascii_digit() || c == '_'),
    )
        .take()
        .parse_next(input)
}

pub(super) fn float(input: &mut Input) -> GreenResult {
    float_impl.parse_next(input).map(|text| tok(FLOAT, text))
}
fn float_impl<'s>(input: &mut Input<'s>) -> PResult<&'s str, SyntaxError> {
    (
        opt(one_of(['+', '-'])),
        alt((
            (
                "0x",
                (unsigned_hex, opt(('.', opt(unsigned_hex)))),
                opt((one_of(['p', 'P']), opt(one_of(['+', '-'])), unsigned_dec)),
            )
                .void(),
            (
                (unsigned_dec, opt(('.', opt(unsigned_dec)))),
                opt((one_of(['e', 'E']), opt(one_of(['+', '-'])), unsigned_dec)),
            )
                .void(),
            "inf".void(),
            ("nan", opt((":0x", unsigned_hex))).void(),
        )),
        peek(none_of(is_id_char)),
    )
        .take()
        .verify(|text: &str| {
            let is_hex = text.starts_with("0x");
            let mut chars = text.chars();
            while let Some(c) = chars.next() {
                if c == '_'
                    && !chars.next().is_some_and(|c| {
                        if is_hex {
                            c.is_ascii_hexdigit()
                        } else {
                            c.is_ascii_digit()
                        }
                    })
                {
                    return false;
                }
            }
            true
        })
        .parse_next(input)
}

pub(super) fn error_token<'s>(
    allow_parens: bool,
) -> impl Parser<Input<'s>, GreenElement, SyntaxError> {
    dispatch! {peek(any);
        '$' => ('$', take_while(0.., is_id_char)).take(),
        '(' | ')' if allow_parens => alt(("(", ")")),
        '(' | ')' => fail,
        '"' => string_impl,
        c if is_id_char(c) => take_while(1.., is_id_char),
        _ => take_till(1.., |c: char| c == '(' || c == ')' || c.is_ascii_whitespace()),
    }
    .map(|text| tok(ERROR, text))
}

pub(super) fn error_term<'s, const N: usize>(
    allowed_names: [&'static str; N],
) -> impl Parser<Input<'s>, Vec<GreenElement>, SyntaxError> {
    preceded(
        peek(not((
            trivias,
            '(',
            trivias,
            word.verify(move |word| allowed_names.contains(word)),
        ))),
        error_term_inner,
    )
}
fn error_term_inner(input: &mut Input) -> PResult<Vec<GreenElement>, SyntaxError> {
    (
        '('.map(|_| tok(ERROR, "(")),
        repeat(
            0..,
            (
                trivias,
                dispatch! {peek(opt('('));
                    Some(..) => error_term_inner.map(TermItem::Multi),
                    None => error_token(false).map(TermItem::Single),
                },
            ),
        )
        .fold(Vec::<GreenElement>::new, |mut acc, (mut trivias, item)| {
            acc.append(&mut trivias);
            match item {
                TermItem::Single(token) => acc.push(token),
                TermItem::Multi(mut tokens) => acc.append(&mut tokens),
            }
            acc
        }),
        opt(')'.map(|_| tok(ERROR, ")"))),
    )
        .parse_next(input)
        .map(|(l_paren, mut tokens, r_paren)| {
            let mut children = Vec::with_capacity(3);
            children.push(l_paren);
            children.append(&mut tokens);
            if let Some(r_paren) = r_paren {
                children.push(r_paren);
            }
            children
        })
}
enum TermItem {
    Single(GreenElement),
    Multi(Vec<GreenElement>),
}
