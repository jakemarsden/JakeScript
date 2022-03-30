#![feature(assert_matches)]

use harness::FailureReason;
use jakescript::interpreter::{Error, Value};
use std::assert_matches::assert_matches;

pub mod harness;

#[test]
fn declare_const_variable_with_initialiser() {
    harness::init();
    let source_code = r##"
const a = 10;
console.assert(a === 10);
"##;
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.success_value(), Some(Value::Undefined));
}

#[test]
fn set_initialised_const_variable() {
    harness::init();
    let source_code = r##"
const a = 10;
a = 20;
"##;
    let report = harness::exec_source_code(source_code);
    assert_matches!(
        report.failure_reason(),
        Some(FailureReason::Runtime(Error::AssignToConstVariable(..)))
    );
}
