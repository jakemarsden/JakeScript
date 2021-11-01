#![feature(assert_matches)]

use common::{exec_source_code, TestOutput};
use jakescript::interpreter::{Error, Value};
use std::assert_matches::assert_matches;

pub mod common;

#[test]
fn declare_const_variable_with_initialiser() {
    let source_code = r##"
const a = 10;
assert a === 10;
"##;
    let result = exec_source_code(source_code);
    assert_matches!(result.output(), TestOutput::Pass(Value::Undefined));
}

#[test]
fn set_initialised_const_variable() {
    let source_code = r##"
const a = 10;
a = 20;
"##;
    let result = exec_source_code(source_code);
    assert_matches!(
        result.output(),
        TestOutput::InterpreterError(Error::AssignToConstVariable(..))
    );
}
