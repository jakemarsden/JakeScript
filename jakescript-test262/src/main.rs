//! It is expected that the _[test262](https://github.com/tc39/test262)_ repository is checked out
//! next to this repository.

use jakescript::interpreter::{Eval, Interpreter, Vm};
use jakescript::lexer::Lexer;
use jakescript::parser::Parser;
use jakescript::token::SourceLocation;
use std::path::Path;
use test262_harness::{Error, Harness, Test};

static TEST262_ROOT_DIR: &str = "../test262";
static TEST262_TEST_DIR: &str = "../test262/test";

fn main() {
    let harness = Harness::new(TEST262_ROOT_DIR).expect("failed to initialize harness");
    for test in harness {
        match test {
            Ok(test) if is_test_we_care_about(&test.path) => {
                println!("running test from {:?}", test.path);
                exec_test(&test);
            }
            Err(Error::DescriptionInvalid(path)) if is_test_we_care_about(&path) => {
                eprintln!("description invalid: {}", path.display());
            }

            Ok(..) | Err(Error::DescriptionInvalid(..)) => {}
            Err(err) => panic!("{}", err),
        };
    }
}

fn is_test_we_care_about(path: impl AsRef<Path>) -> bool {
    path.as_ref().starts_with(TEST262_TEST_DIR)
}

fn exec_test(test: &Test) {
    let lexer = Lexer::for_str(&test.source, SourceLocation::at_start_of(&test.path));
    let parser = Parser::for_lexer(lexer);

    let script = match parser.execute() {
        Ok(script) => script,
        Err(err) => {
            eprintln!("FAIL (parse) {}: {}", test.path.display(), err);
            return;
        }
    };

    let mut it = Interpreter::new(Vm::new().unwrap());
    match script.eval(&mut it) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("FAIL (runtime) {}: {}", test.path.display(), err);
            return;
        }
    }

    println!("PASS {}", test.path.display());
}
