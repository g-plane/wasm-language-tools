#![doc = include_str!("../README.md")]

mod error;
mod parser;

pub use crate::{
    error::{Message, SyntaxError},
    parser::{is_id_char, parse, parse_to_green},
};
