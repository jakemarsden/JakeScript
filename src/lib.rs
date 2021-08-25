#![feature(assert_matches)]
#![feature(associated_type_defaults)]
#![feature(derive_default_enum)]

pub mod ast;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub(crate) mod util;
