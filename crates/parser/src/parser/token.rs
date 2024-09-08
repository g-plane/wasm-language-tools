use super::{tok, GreenElement, GreenResult, Input};
use crate::SyntaxKind::*;
use winnow::{
    ascii::{hex_digit0, line_ending, multispace1, take_escaped, till_line_ending},
    combinator::{alt, dispatch, empty, eof, fail, opt, peek, repeat, repeat_till},
    error::{ContextError, StrContext, StrContextValue},
    stream::AsChar,
    token::{any, none_of, one_of, take_till, take_until, take_while},
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

pub(super) fn word<'s>(input: &mut Input<'s>) -> PResult<&'s str> {
    (
        one_of(|c: char| c.is_ascii_lowercase()),
        take_while(0.., |c: char| {
            c.is_ascii_alphanumeric()
                || c.is_ascii_punctuation()
                    && !matches!(
                        c,
                        '\'' | '"' | ',' | ';' | '(' | ')' | '[' | ']' | '{' | '}'
                    )
        }),
    )
        .take()
        .parse_next(input)
}

pub(super) fn trivias(input: &mut Input) -> PResult<Vec<GreenElement>> {
    repeat(0.., alt((ws, line_comment, block_comment))).parse_next(input)
}

pub(super) fn trivias_prefixed<'s, P>(
    parser: P,
) -> impl Parser<Input<'s>, Vec<GreenElement>, ContextError>
where
    P: Parser<Input<'s>, GreenElement, ContextError>,
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
fn block_comment_impl<'s>(input: &mut Input<'s>) -> PResult<&'s str> {
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
) -> impl Parser<Input<'s>, GreenElement, ContextError> {
    word.verify(move |word: &str| word == keyword)
        .map(|text| tok(KEYWORD, text))
        .context(StrContext::Expected(StrContextValue::StringLiteral(
            keyword,
        )))
}

pub(super) fn ident(input: &mut Input) -> GreenResult {
    (
        '$',
        take_while(1.., |c: char| {
            c.is_ascii_alphanumeric()
                || c.is_ascii_punctuation()
                    && !matches!(
                        c,
                        '\'' | '"' | ',' | ';' | '(' | ')' | '[' | ']' | '{' | '}'
                    )
        }),
    )
        .take()
        .context(StrContext::Expected(StrContextValue::Description(
            "identifier",
        )))
        .parse_next(input)
        .map(|text| tok(IDENT, text))
}

pub(super) fn string(input: &mut Input) -> GreenResult {
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
        .context(StrContext::Expected(StrContextValue::Description(
            "string literal",
        )))
        .parse_next(input)
        .map(|text| tok(STRING, text))
}

pub(super) fn error_token<'s>(
    allow_parens: bool,
) -> impl Parser<Input<'s>, GreenElement, ContextError> {
    dispatch! {any;
        '$' => take_while(1.., |c: char| {
            c.is_ascii_alphanumeric() ||
                c.is_ascii_punctuation() && !matches!(c, '\'' | '"' | ',' | ';' | '(' | ')' | '[' | ']' | '{' | '}')
            }).void(),
        'a'..='z' | 'A'..='Z' => take_while(0.., AsChar::is_alphanum).void(),
        '0'..='9' => take_while(0.., AsChar::is_dec_digit).void(),
        '(' | ')' if allow_parens => empty,
        '(' | ')' => fail,
        _ => take_till(0.., |c:char| c == '(' || c == ')' || c.is_ascii_whitespace()).void(),
    }
    .take()
    .map(|text| tok(ERROR, text))
}
