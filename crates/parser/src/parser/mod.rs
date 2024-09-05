use self::token::*;
use super::{SyntaxKind, SyntaxNode};
use rowan::{GreenNode, GreenToken, NodeOrToken};
use winnow::{
    combinator::{alt, repeat},
    error::{ContextError, ErrMode, FromRecoverableError, StrContext},
    stream::{Recover, Recoverable, Stream},
    PResult, Parser as WinnowParser, RecoverableParser,
};

mod token;
mod ty;

#[derive(Debug)]
pub struct Parser<'s> {
    input: Input<'s>,
    errors: Vec<ContextError>,
}
impl<'s> Parser<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            input: Recoverable::new(source),
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> SyntaxNode {
        let (_, tree, errors) = root.recoverable_parse(&self.input);
        self.errors = errors;
        tree.expect("parser should always succeed even if there are syntax errors")
    }

    pub fn errors(&self) -> &[ContextError] {
        &self.errors
    }
}

type GreenElement = NodeOrToken<GreenNode, GreenToken>;
type GreenResult = PResult<GreenElement>;
type Input<'s> = Recoverable<&'s str, ContextError>;

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

fn root(input: &mut Input) -> PResult<SyntaxNode> {
    (
        retry(self::ty::func_type),
        repeat(
            0..,
            alt((ws, line_comment, block_comment, error_token(true))),
        ),
    )
        .parse_next(input)
        .map(|(mut children, mut trivias)| {
            children.append(&mut trivias);
            SyntaxNode::new_root(GreenNode::new(SyntaxKind::ROOT.into(), children))
        })
}

// copied and modified from https://github.com/winnow-rs/winnow/blob/95e0c100656a98a0ff3bc8420fc8844edff6b615/src/combinator/parser.rs#L963
fn retry<'s, P>(mut parser: P) -> impl WinnowParser<Input<'s>, Vec<GreenElement>, ContextError>
where
    P: WinnowParser<Input<'s>, GreenElement, ContextError>,
{
    move |input: &mut Input<'s>| {
        let mut error_parser = error_token(false).context(StrContext::Label("unexpected token"));
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
            if let Ok(error_token) = error_parser.parse_next(input) {
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
            }

            input.reset(&trivia_start);
            err = err.map(|err| {
                ContextError::from_recoverable_error(&token_start, &err_start, input, err)
            });
            return Err(err);
        }
    }
}

// copied and modified from https://github.com/winnow-rs/winnow/blob/95e0c100656a98a0ff3bc8420fc8844edff6b615/src/combinator/parser.rs#L1061
fn resume<'s, P>(
    mut parser: P,
) -> impl WinnowParser<Input<'s>, Option<Vec<GreenElement>>, ContextError>
where
    P: WinnowParser<Input<'s>, GreenElement, ContextError>,
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
        input.reset(&trivia_start);
        if let Err(err_) = input.record_err(&token_start, &err_start, err) {
            err = err_;
        } else {
            return Ok(None);
        }

        err = err
            .map(|err| ContextError::from_recoverable_error(&token_start, &err_start, input, err));
        Err(err)
    }
}
