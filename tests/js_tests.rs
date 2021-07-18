use jakescript::ast::Program;
use jakescript::interpreter::{Error, Value};
use std::fs;
use std::path::Path;

mod common;

#[test]
fn js_tests() {
    let mut success_count = 0_usize;
    let mut failure_count = 0_usize;
    for dir_entry in fs::read_dir("tests-js").unwrap() {
        let source_file = dir_entry.unwrap().path();
        match eval_from_source_file(&source_file) {
            Ok(..) => {
                success_count += 1;
                println!("JS :: [passed] {:?}", source_file);
            }
            Err((err, program)) => {
                failure_count += 1;
                println!("JS :: [failed] {:?}: {}", source_file, err);
                println!("{:#?}", program);
            }
        }
    }
    println!("JS :: {} passed, {} failed", success_count, failure_count);
    assert_eq!(failure_count, 0);
}

fn eval_from_source_file(path: impl AsRef<Path>) -> Result<Value, (Error, Program)> {
    let source_code = fs::read_to_string(path).unwrap();
    common::eval_from_source_code(&source_code)
}
