#![feature(assert_matches)]
#![feature(process_exitcode_placeholder)]
#![feature(termination_trait_lib)]

use jakescript::interpreter::{ExecutionState, Value};
use std::assert_matches::assert_matches;

pub mod harness;

#[test]
fn exit_with_end_of_input() {
    harness::init();
    let source_code = r#"
let a = 1 + 2;
assert a === 3;
"#;
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.success_value(), Some(Value::Undefined));
    assert_matches!(report.vm_state(), Some(ExecutionState::Advance));
}

#[test]
fn exit_with_exit_statement() {
    harness::init();
    let source_code = r#"
let a = 1 + 2;
assert a === 3;
exit;
"#;
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.success_value(), Some(Value::Undefined));
    assert_matches!(report.vm_state(), Some(ExecutionState::Exit));
}

#[test]
fn statements_after_exit_are_not_reached() {
    harness::init();
    let source_code = r#"
let a = 1 + 2;
assert a === 3;
exit;
assert false;
"#;
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.success_value(), Some(Value::Undefined));
    assert_matches!(report.vm_state(), Some(ExecutionState::Exit));
}
