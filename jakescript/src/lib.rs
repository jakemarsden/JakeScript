#![feature(assert_matches)]
#![feature(associated_type_defaults)]
#![feature(bool_to_option)]
#![feature(box_patterns)]
#![feature(derive_default_enum)]
#![feature(if_let_guard)]
#![feature(iter_advance_by)]
#![feature(iter_intersperse)]
#![feature(let_chains)]

pub mod ast;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod runtime;
pub mod str;
pub mod token;

pub(crate) mod iter;
