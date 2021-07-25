use jakescript::ast::Program;
use jakescript::interpreter::{self, Eval, Interpreter, Value};
use jakescript::lexer::Lexer;
use jakescript::parser::Parser;
use std::path::Path;
use std::{fs, io};

// dead_code: See https://github.com/rust-lang/rust/issues/46379
#[allow(dead_code)]
pub fn eval(ast: &Program) -> Result<Value, interpreter::Error> {
    let mut it = Interpreter::default();
    ast.eval(&mut it)
}

// dead_code: See https://github.com/rust-lang/rust/issues/46379
#[allow(dead_code)]
pub fn parse_from_source_file(path: impl AsRef<Path>) -> Result<Program, io::Error> {
    let source_code = fs::read_to_string(path)?;
    let ast = parse_from_source_code(&source_code);
    Ok(ast)
}

// dead_code: See https://github.com/rust-lang/rust/issues/46379
#[allow(dead_code)]
pub fn parse_from_source_code(source_code: &str) -> Program {
    let lexer = Lexer::for_str(source_code);
    let parser = Parser::for_lexer(lexer);
    parser.execute()
}
