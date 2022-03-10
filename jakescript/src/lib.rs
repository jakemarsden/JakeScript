#![feature(assert_matches)]
#![feature(associated_type_defaults)]
#![feature(bool_to_option)]
#![feature(derive_default_enum)]
#![feature(iter_advance_by)]
#![feature(iter_intersperse)]

pub mod ast;

pub(crate) mod iter;

pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod runtime;
pub mod str;
