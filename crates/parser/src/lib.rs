#![doc = include_str!("../README.md")]

mod error;
mod parser;
mod parser2;

pub use crate::{
    error::{Message, SyntaxError},
    parser::{is_id_char, parse, parse_to_green},
    parser2::{parse as parse2, parse_to_green as parse_to_green2},
};
