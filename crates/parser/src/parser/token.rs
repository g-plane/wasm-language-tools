use super::{tok, GreenElement, GreenResult, Input};
use crate::error::SyntaxError;
use wat_syntax::SyntaxKind::*;
use winnow::{
    ascii::{hex_digit0, line_ending, multispace1, take_escaped, till_line_ending},
    combinator::{alt, dispatch, empty, eof, fail, opt, peek, repeat, repeat_till},
    error::{StrContext, StrContextValue},
    stream::AsChar,
    token::{any, none_of, one_of, take, take_till, take_until, take_while},
    PResult, Parser,
};

pub(super) fn l_paren(input: &mut Input) -> GreenResult {
    '('.map(|_| tok(L_PAREN, "("))
        .context(StrContext::Expected(StrContextValue::CharLiteral('(')))
        .parse_next(input)
}

pub(super) fn r_paren(input: &mut Input) -> GreenResult {
    ')'.map(|_| tok(R_PAREN, ")"))
        .context(StrContext::Expected(StrContextValue::CharLiteral(')')))
        .parse_next(input)
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
        .context(StrContext::Expected(StrContextValue::StringLiteral(
            keyword,
        )))
}

pub(super) fn ident(input: &mut Input) -> GreenResult {
    ident_impl.parse_next(input).map(|text| tok(IDENT, text))
}
fn ident_impl<'s>(input: &mut Input<'s>) -> PResult<&'s str, SyntaxError> {
    ('$', take_while(1.., is_id_char))
        .take()
        .context(StrContext::Expected(StrContextValue::Description(
            "identifier",
        )))
        .parse_next(input)
}

fn is_id_char(c: char) -> bool {
    c.is_ascii_alphanumeric()
        || c.is_ascii_punctuation()
            && !matches!(
                c,
                '\'' | '"' | ',' | ';' | '(' | ')' | '[' | ']' | '{' | '}'
            )
}

pub(super) fn string(input: &mut Input) -> GreenResult {
    string_impl
        .context(StrContext::Expected(StrContextValue::Description(
            "string literal",
        )))
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
    (opt(one_of(['+', '-'])), unsigned_int_impl)
        .take()
        .parse_next(input)
}

pub(super) fn unsigned_int(input: &mut Input) -> GreenResult {
    unsigned_int_impl
        .parse_next(input)
        .map(|text| tok(UNSIGNED_INT, text))
}
pub(super) fn unsigned_int_impl<'s>(input: &mut Input<'s>) -> PResult<&'s str, SyntaxError> {
    dispatch! {peek(take(2usize));
        "0x" => ("0x", unsigned_hex).take(),
        _ => unsigned_dec,
    }
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
                alt((
                    (unsigned_dec, opt(('.', opt(unsigned_dec)))).void(),
                    ('.', unsigned_dec).void(),
                )),
                opt((one_of(['e', 'E']), opt(one_of(['+', '-'])), unsigned_dec)),
            )
                .void(),
            (
                "0x",
                alt((
                    (unsigned_hex, opt(('.', opt(unsigned_hex)))).void(),
                    ('.', unsigned_hex).void(),
                )),
                opt((one_of(['p', 'P']), opt(one_of(['+', '-'])), unsigned_dec)),
            )
                .void(),
            "inf".void(),
            ("nan", opt((":0x", unsigned_hex))).void(),
        )),
    )
        .take()
        .parse_next(input)
}

pub(super) fn error_token<'s>(
    allow_parens: bool,
) -> impl Parser<Input<'s>, GreenElement, SyntaxError> {
    dispatch! {peek(any);
        '$' => ident_impl,
        '0'..='9' => alt((float_impl, int_impl, unsigned_int_impl)),
        '(' | ')' if allow_parens => alt(("(", ")")),
        '(' | ')' => fail,
        c if is_id_char(c) => word,
        _ => take_till(1.., |c: char| c == '(' || c == ')' || c.is_ascii_whitespace()),
    }
    .map(|text| tok(ERROR, text))
}
