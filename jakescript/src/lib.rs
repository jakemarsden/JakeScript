#![feature(assert_matches)]
#![feature(associated_type_defaults)]
#![feature(box_patterns)]
#![feature(if_let_guard)]
#![feature(iter_advance_by)]
#![feature(iter_intersperse)]
#![feature(let_chains)]

pub mod ast;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod runtime;
pub mod token;

pub(crate) mod iter;
