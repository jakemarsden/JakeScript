#![feature(assert_matches)]

use jakescript::interpreter::{ExecutionState, Value};
use std::assert_matches::assert_matches;

pub mod harness;

#[test]
fn exit_with_end_of_input() {
    harness::init();
    let source_code = r#"
let a = 1 + 2;
console.assert(a === 3);
"#;
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.success_value(), Some(Value::Undefined));
    assert_matches!(report.vm_state(), Some(ExecutionState::Advance));
}

#[test]
fn exit_with_exit_function() {
    harness::init();
    let source_code = r#"
let a = 1 + 2;
console.assert(a === 3);
exit();
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
console.assert(a === 3);
exit();
console.assertNotReached();
"#;
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.success_value(), Some(Value::Undefined));
    assert_matches!(report.vm_state(), Some(ExecutionState::Exit));
}

#[test]
fn statements_in_loop_after_exit_are_not_reached() {
    harness::init();
    let source_code = r#"
let i = 0;
while (i < 10) {
    if (i === 3) {
        exit();
    }
    if (i >= 3) {
        console.assertNotReached();
    }
    i += 1;
}
console.assertNotReached();
"#;
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.success_value(), Some(Value::Undefined));
    assert_matches!(report.vm_state(), Some(ExecutionState::Exit));
}
