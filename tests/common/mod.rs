use jakescript::ast::Program;
use jakescript::interpreter::{Error, Eval, Interpreter, Value};
use jakescript::lexer::Lexer;
use jakescript::parser::Parser;
use std::fs;
use std::path::Path;

// dead_code: See https://github.com/rust-lang/rust/issues/46379
#[allow(dead_code)]
pub fn eval_from_source_file(path: impl AsRef<Path>) -> (Result<Value, Error>, Program) {
    let source_code = fs::read_to_string(path).expect("Failed to read source code from file");
    eval_from_source_code(&source_code)
}

// dead_code: See https://github.com/rust-lang/rust/issues/46379
#[allow(dead_code)]
pub fn eval_from_source_code(source_code: &str) -> (Result<Value, Error>, Program) {
    let lexer = Lexer::for_str(source_code);
    let parser = Parser::for_lexer(lexer);
    let ast = parser.execute();

    let mut it = Interpreter::default();
    let result = ast.eval(&mut it);
    (result, ast)
}
