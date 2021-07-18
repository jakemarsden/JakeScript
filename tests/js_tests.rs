use jakescript::interpreter::{Eval, Interpreter, Result, Value};
use jakescript::lexer::Lexer;
use jakescript::parser::Parser;
use std::fs;
use std::path::Path;

#[test]
fn js_tests() {
    let mut success_count = 0_usize;
    let mut failure_count = 0_usize;
    for dir_entry in fs::read_dir("tests-js").unwrap() {
        let source_file = dir_entry.unwrap().path();
        match eval_from_source_file(&source_file) {
            Ok(_) => {
                success_count += 1;
                println!("JS :: [passed] {:?}", source_file);
            }
            Err(err) => {
                failure_count += 1;
                println!("JS :: [failed] {:?}: {}", source_file, err);
            }
        }
    }
    println!("JS :: {} passed, {} failed", success_count, failure_count);
    assert_eq!(failure_count, 0);
}

fn eval_from_source_file(path: impl AsRef<Path>) -> Result<Value> {
    let source_code = fs::read_to_string(path).unwrap();
    eval_from_source(&source_code)
}

fn eval_from_source(source_code: &str) -> Result<Value> {
    let lexer = Lexer::for_str(source_code);
    let parser = Parser::for_lexer(lexer);
    let program = parser.execute();

    let mut it = Interpreter::default();
    program.eval(&mut it)
}
