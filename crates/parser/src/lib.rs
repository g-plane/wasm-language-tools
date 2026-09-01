#![doc = include_str!("../README.md")]

mod arch;
mod error;
mod parser2;

pub use crate::{
    error::{Message, SyntaxError},
    parser2::{is_id_char, parse, parse_as},
};
