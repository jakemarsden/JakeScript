#![feature(assert_matches)]

use common::{exec_source_code, TestOutput};
use jakescript::interpreter::Value;
use std::assert_matches::assert_matches;

pub mod common;

#[test]
fn add_add() {
    let source_code = r##"50 + 100 + 17;"##;
    let result = exec_source_code(source_code);
    assert_matches!(result.output(), TestOutput::Pass(Value::Number(167)));
}

#[test]
fn add_mul() {
    let source_code = r##"2 + 3 * 4;"##;
    let result = exec_source_code(source_code);
    assert_matches!(result.output(), TestOutput::Pass(Value::Number(14)));
}

#[test]
fn mul_add() {
    let source_code = r##"2 * 3 + 4;"##;
    let result = exec_source_code(source_code);
    assert_matches!(result.output(), TestOutput::Pass(Value::Number(10)));
}

#[test]
fn eq_add() {
    let source_code = r##"30 === 10 + 20;"##;
    let result = exec_source_code(source_code);
    assert_matches!(result.output(), TestOutput::Pass(Value::Boolean(true)));
}

#[test]
fn add_eq() {
    let source_code = r##"10 + 20 === 30;"##;
    let result = exec_source_code(source_code);
    assert_matches!(result.output(), TestOutput::Pass(Value::Boolean(true)));
}
