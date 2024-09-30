mod error;
mod parser;

pub use crate::{
    error::SyntaxError,
    parser::{is_id_char, Parser},
};
