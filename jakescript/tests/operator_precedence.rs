#![feature(assert_matches)]
#![feature(process_exitcode_placeholder)]
#![feature(termination_trait_lib)]

// TODO: Do something useful with this test (e.g. check the actual AST), or get rid of it, because
//  it doesn't seem to have much over the straight "js_tests/operator_precedence.js" at the moment.

use jakescript::interpreter::{Number, Value};
use std::assert_matches::assert_matches;

pub mod harness;

#[test]
fn add_add() {
    harness::init();
    let source_code = r##"50 + 100 + 17;"##;
    let report = harness::exec_source_code(source_code);
    assert_matches!(
        report.success_value(),
        Some(Value::Number(Number::Int(167)))
    );
}

#[test]
fn add_mul() {
    harness::init();
    let source_code = r##"2 + 3 * 4;"##;
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.success_value(), Some(Value::Number(Number::Int(14))));
}

#[test]
fn mul_add() {
    harness::init();
    let source_code = r##"2 * 3 + 4;"##;
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.success_value(), Some(Value::Number(Number::Int(10))));
}

#[test]
fn eq_add() {
    harness::init();
    let source_code = r##"30 === 10 + 20;"##;
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.success_value(), Some(Value::Boolean(true)));
}

#[test]
fn add_eq() {
    harness::init();
    let source_code = r##"10 + 20 === 30;"##;
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.success_value(), Some(Value::Boolean(true)));
}
