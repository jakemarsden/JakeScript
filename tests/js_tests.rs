use jakescript::interpreter::{Eval, Interpreter, Value};
use jakescript::lexer::Lexer;
use jakescript::parser::Parser;
use std::fs;
use std::path::Path;

#[test]
fn js_tests() {
    for dir_entry in fs::read_dir("tests-js").unwrap() {
        let source_file = dir_entry.unwrap().path();
        eval_from_source_file(&source_file);
    }
}

fn eval_from_source_file(path: impl AsRef<Path>) -> Value {
    let source_code = fs::read_to_string(path).unwrap();
    eval_from_source(&source_code)
}

fn eval_from_source(source_code: &str) -> Value {
    let lexer = Lexer::for_str(source_code);
    let parser = Parser::for_lexer(lexer);
    let program = parser.execute();

    let mut it = Interpreter::default();
    program.eval(&mut it)
}
