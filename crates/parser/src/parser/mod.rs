pub use self::token::is_id_char;
use self::{
    module::module,
    token::{block_comment, error_term, error_token, line_comment, trivias, ws},
};
use crate::error::{Message, SyntaxError};
use rowan::{GreenNode, GreenToken, NodeOrToken};
use wat_syntax::{SyntaxKind, SyntaxNode};
use winnow::{
    combinator::{alt, repeat},
    error::FromRecoverableError,
    stream::{Recover, Recoverable, Stream},
    LocatingSlice, Parser, RecoverableParser,
};

mod instr;
mod module;
mod token;
mod ty;

/// Parse the code into a rowan syntax node.
pub fn parse(source: &str) -> (SyntaxNode, Vec<SyntaxError>) {
    let (green, errors) = parse_to_green(source);
    (SyntaxNode::new_root(green), errors)
}

/// Parse the code into a rowan green node.
pub fn parse_to_green(source: &str) -> (GreenNode, Vec<SyntaxError>) {
    let (_, tree, errors) = root.recoverable_parse(*Input::new(LocatingSlice::new(source)));
    (
        tree.expect("parser should always succeed even if there are syntax errors"),
        errors,
    )
}

type GreenElement = NodeOrToken<GreenNode, GreenToken>;
type GreenResult = Result<GreenElement, SyntaxError>;
pub(crate) type Input<'s> = Recoverable<LocatingSlice<&'s str>, SyntaxError>;

fn tok(kind: SyntaxKind, text: &str) -> GreenElement {
    NodeOrToken::Token(GreenToken::new(kind.into(), text))
}

fn node<I>(kind: SyntaxKind, children: I) -> GreenElement
where
    I: IntoIterator<Item = GreenElement>,
    I::IntoIter: ExactSizeIterator,
{
    NodeOrToken::Node(GreenNode::new(kind.into(), children))
}

fn root(input: &mut Input) -> Result<GreenNode, SyntaxError> {
    (
        repeat::<_, _, Vec<_>, _, _>(0.., retry_once(module, [])),
        repeat(
            0..,
            alt((ws, line_comment, block_comment, error_token(true))),
        ),
    )
        .parse_next(input)
        .map(|(modules, mut trivias)| {
            let mut children = Vec::with_capacity(3 + modules.len());
            modules
                .into_iter()
                .for_each(|mut module| children.append(&mut module));
            children.append(&mut trivias);
            GreenNode::new(SyntaxKind::ROOT.into(), children)
        })
}

/// Note: use `retry_once` instead if you're using `retry` in `repeat` or `repeat_till`.
// copied and modified from https://github.com/winnow-rs/winnow/blob/95e0c100656a98a0ff3bc8420fc8844edff6b615/src/combinator/parser.rs#L963
fn retry<'s, P, const N: usize>(
    mut parser: P,
    allowed_names: [&'static str; N],
) -> impl Parser<Input<'s>, Vec<GreenElement>, SyntaxError>
where
    P: Parser<Input<'s>, GreenElement, SyntaxError>,
{
    move |input: &mut Input<'s>| {
        let mut error_token_parser =
            error_token(false).context(Message::Description("unexpected token"));
        let mut error_term_parser = error_term(allowed_names);
        let mut tokens = Vec::with_capacity(1);
        loop {
            let trivia_start = input.checkpoint();
            let mut trivia_tokens = match trivias.parse_next(input) {
                Ok(trivias) => trivias,
                Err(err) => return Err(err),
            };
            let token_start = input.checkpoint();
            let mut err = match parser.parse_next(input) {
                Ok(o) => {
                    tokens.append(&mut trivia_tokens);
                    tokens.push(o);
                    return Ok(tokens);
                }
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
            } else if let Ok(mut error_tokens) = error_term_parser.parse_next(input) {
                let i_eof_offset = input.eof_offset();
                if err_start_eof_offset == i_eof_offset {
                    // didn't advance so bubble the error up
                } else if let Err(err_) = input.record_err(&token_start, &err_start, err) {
                    err = err_;
                } else {
                    tokens.append(&mut trivia_tokens);
                    tokens.append(&mut error_tokens);
                    // unlike `error_token_parser`, `error_term_parser` consumes many tokens,
                    // so we should exit the loop instead of continuing
                    return Ok(tokens);
                }
            }

            input.reset(&trivia_start);
            err = SyntaxError::from_recoverable_error(&token_start, &err_start, input, err);
            return Err(err);
        }
    }
}

/// If you're try using `retry` in `repeat` or `repeat_till`,
/// you should use `retry_once` instead.
fn retry_once<'s, P, const N: usize>(
    mut parser: P,
    allowed_names: [&'static str; N],
) -> impl Parser<Input<'s>, Vec<GreenElement>, SyntaxError>
where
    P: Parser<Input<'s>, GreenElement, SyntaxError>,
{
    move |input: &mut Input<'s>| {
        let mut error_token_parser =
            error_token(false).context(Message::Description("unexpected token"));
        let mut error_term_parser = error_term(allowed_names);
        let mut tokens = Vec::with_capacity(1);

        let trivia_start = input.checkpoint();
        let mut trivia_tokens = match trivias.parse_next(input) {
            Ok(trivias) => trivias,
            Err(err) => return Err(err),
        };
        let token_start = input.checkpoint();
        let mut err = match parser.parse_next(input) {
            Ok(o) => {
                tokens.append(&mut trivia_tokens);
                tokens.push(o);
                return Ok(tokens);
            }
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
                return Ok(tokens);
            }
        } else if let Ok(mut error_tokens) = error_term_parser.parse_next(input) {
            let i_eof_offset = input.eof_offset();
            if err_start_eof_offset == i_eof_offset {
                // didn't advance so bubble the error up
            } else if let Err(err_) = input.record_err(&token_start, &err_start, err) {
                err = err_;
            } else {
                tokens.append(&mut trivia_tokens);
                tokens.append(&mut error_tokens);
                return Ok(tokens);
            }
        }

        input.reset(&trivia_start);
        err = SyntaxError::from_recoverable_error(&token_start, &err_start, input, err);
        Err(err)
    }
}

/// This is similar to [`opt`](winnow::combinator::opt),
/// but it will record recoverable error if the parser fails.
/// This can be used to avoid switch to another branch of [`alt`](winnow::combinator::alt).
fn must<'s, P, O>(mut parser: P) -> impl Parser<Input<'s>, Option<O>, SyntaxError>
where
    P: Parser<Input<'s>, O, SyntaxError>,
{
    move |input: &mut Input<'s>| {
        let start = input.checkpoint();
        let mut err = match parser.parse_next(input) {
            Ok(o) => return Ok(Some(o)),
            Err(err) => err,
        };
        let err_start = input.checkpoint();
        if let Err(err_) = input.record_err(&start, &err_start, err) {
            err = err_;
        } else {
            input.reset(&start);
            return Ok(None);
        }
        input.reset(&start);
        err = SyntaxError::from_recoverable_error(&start, &err_start, input, err);
        Err(err)
    }
}
