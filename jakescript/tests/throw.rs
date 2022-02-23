#![feature(assert_matches)]
#![feature(process_exitcode_placeholder)]
#![feature(termination_trait_lib)]

use jakescript::interpreter::{ExecutionState, Number, Value};
use std::assert_matches::assert_matches;

pub mod harness;

#[test]
fn throw() {
    harness::init();
    let source_code = r#"
let a = 1 + 2;
console.assert(a === 3);
throw 42;
"#;
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.success_value(), Some(Value::Undefined));
    assert_matches!(
        report.vm_state(),
        Some(ExecutionState::Exception(Value::Number(Number::Int(42))))
    );
}

#[test]
fn throw_variable() {
    harness::init();
    let source_code = r#"
let a = 1 + 2;
console.assert(a === 3);
throw a;
"#;
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.success_value(), Some(Value::Undefined));
    assert_matches!(
        report.vm_state(),
        Some(ExecutionState::Exception(Value::Number(Number::Int(3))))
    );
}

#[test]
fn throw_undefined() {
    harness::init();
    let source_code = r#"
let a = 1 + 2;
console.assert(a === 3);
throw undefined;
"#;
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.success_value(), Some(Value::Undefined));
    assert_matches!(
        report.vm_state(),
        Some(ExecutionState::Exception(Value::Undefined))
    );
}

#[test]
fn statements_after_throw_are_not_reached() {
    harness::init();
    let source_code = r#"
let a = 1 + 2;
console.assert(a === 3);
throw 42;
console.assertNotReached();
"#;
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.success_value(), Some(Value::Undefined));
    assert_matches!(
        report.vm_state(),
        Some(ExecutionState::Exception(Value::Number(Number::Int(42))))
    );
}

#[test]
fn statements_in_loop_after_throw_are_not_reached() {
    harness::init();
    let source_code = r#"
let i = 0;
while (i < 10) {
    if (i === 3) {
        throw i;
    }
    console.assert(i < 3);
    i += 1;
}
console.assertNotReached();
"#;
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.success_value(), Some(Value::Undefined));
    assert_matches!(
        report.vm_state(),
        Some(ExecutionState::Exception(Value::Number(Number::Int(3))))
    );
}
