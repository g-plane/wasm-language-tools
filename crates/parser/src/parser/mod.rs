use self::{
    module::module,
    token::{block_comment, error_term, error_token, line_comment, trivias, ws},
};
use crate::error::SyntaxError;
use rowan::{GreenNode, GreenToken, NodeOrToken};
use wat_syntax::{SyntaxKind, SyntaxNode};
use winnow::{
    combinator::{alt, repeat},
    error::{ErrMode, FromRecoverableError, StrContext},
    stream::{Recover, Recoverable, Stream},
    Located, PResult, Parser as WinnowParser, RecoverableParser,
};

mod instr;
mod module;
mod token;
mod ty;

#[derive(Debug)]
pub struct Parser<'s> {
    input: Input<'s>,
    errors: Vec<SyntaxError>,
}
impl<'s> Parser<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            input: Recoverable::new(Located::new(source)),
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> SyntaxNode {
        let (_, tree, errors) = root.recoverable_parse(*self.input);
        self.errors = errors;
        tree.expect("parser should always succeed even if there are syntax errors")
    }

    pub fn errors(&self) -> &[SyntaxError] {
        &self.errors
    }
}

type GreenElement = NodeOrToken<GreenNode, GreenToken>;
type GreenResult = PResult<GreenElement, SyntaxError>;
pub(crate) type Input<'s> = Recoverable<Located<&'s str>, SyntaxError>;

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

fn root(input: &mut Input) -> PResult<SyntaxNode, SyntaxError> {
    (
        repeat::<_, _, Vec<_>, _, _>(0.., retry(module, [])),
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
            SyntaxNode::new_root(GreenNode::new(SyntaxKind::ROOT.into(), children))
        })
}

// copied and modified from https://github.com/winnow-rs/winnow/blob/95e0c100656a98a0ff3bc8420fc8844edff6b615/src/combinator/parser.rs#L963
fn retry<'s, P, const N: usize>(
    mut parser: P,
    allowed_names: [&'static str; N],
) -> impl WinnowParser<Input<'s>, Vec<GreenElement>, SyntaxError>
where
    P: WinnowParser<Input<'s>, GreenElement, SyntaxError>,
{
    move |input: &mut Input<'s>| {
        let mut error_token_parser =
            error_token(false).context(StrContext::Label("unexpected token"));
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
            err = err.map(|err| {
                SyntaxError::from_recoverable_error(&token_start, &err_start, input, err)
            });
            return Err(err);
        }
    }
}

// copied and modified from https://github.com/winnow-rs/winnow/blob/95e0c100656a98a0ff3bc8420fc8844edff6b615/src/combinator/parser.rs#L1061
fn resume<'s, P>(
    mut parser: P,
) -> impl WinnowParser<Input<'s>, Option<Vec<GreenElement>>, SyntaxError>
where
    P: WinnowParser<Input<'s>, GreenElement, SyntaxError>,
{
    move |input: &mut Input<'s>| {
        let trivia_start = input.checkpoint();
        let mut tokens = match trivias.parse_next(input) {
            Ok(trivias) => trivias,
            Err(err) => return Err(err),
        };
        let token_start = input.checkpoint();
        let mut err = match parser.parse_next(input) {
            Ok(o) => {
                tokens.push(o);
                return Ok(Some(tokens));
            }
            Err(ErrMode::Incomplete(e)) => return Err(ErrMode::Incomplete(e)),
            Err(err) => err,
        };
        let err_start = input.checkpoint();
        if error_token(true).parse_next(input).is_ok() || input.eof_offset() == 0 {
            if let Err(err_) = input.record_err(&token_start, &err_start, err) {
                err = err_;
            } else {
                input.reset(&trivia_start);
                return Ok(None);
            }
        }

        input.reset(&trivia_start);
        err = err
            .map(|err| SyntaxError::from_recoverable_error(&token_start, &err_start, input, err));
        Err(err)
    }
}
