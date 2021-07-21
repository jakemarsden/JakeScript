#![feature(assert_matches)]

use jakescript::interpreter::{Error, Value};

mod common;

#[test]
fn declare_const_variable_with_initialiser() {
    assert_matches!(
        common::eval_from_source_code(
            r##"
const a = 10;
assert a === 10;
"##
        ),
        (Ok(Value::Undefined), _)
    );
}

#[test]
fn set_initialised_const_variable() {
    assert_matches!(
        common::eval_from_source_code(
            r##"
const a = 10;
a = 20;
"##
        ),
        (Err(Error::VariableIsConst(..)), _)
    );
}
