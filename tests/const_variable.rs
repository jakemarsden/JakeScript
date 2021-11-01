#![feature(assert_matches)]
#![feature(process_exitcode_placeholder)]
#![feature(termination_trait_lib)]

use harness::TestCaseResult;
use jakescript::interpreter::{Error, Value};
use std::assert_matches::assert_matches;

pub mod harness;

#[test]
fn declare_const_variable_with_initialiser() {
    harness::init();
    let source_code = r##"
const a = 10;
assert a === 10;
"##;
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.result(), TestCaseResult::Pass(Value::Undefined));
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
        report.result(),
        TestCaseResult::InterpreterError(Error::AssignToConstVariable(..))
    );
}
