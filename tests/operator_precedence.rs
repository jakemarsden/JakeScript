#![feature(assert_matches)]
#![feature(process_exitcode_placeholder)]
#![feature(termination_trait_lib)]

use harness::TestCaseResult;
use jakescript::interpreter::Value;
use std::assert_matches::assert_matches;

pub mod harness;

#[test]
fn add_add() {
    let source_code = r##"50 + 100 + 17;"##;
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.result(), TestCaseResult::Pass(Value::Number(167)));
}

#[test]
fn add_mul() {
    let source_code = r##"2 + 3 * 4;"##;
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.result(), TestCaseResult::Pass(Value::Number(14)));
}

#[test]
fn mul_add() {
    let source_code = r##"2 * 3 + 4;"##;
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.result(), TestCaseResult::Pass(Value::Number(10)));
}

#[test]
fn eq_add() {
    let source_code = r##"30 === 10 + 20;"##;
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.result(), TestCaseResult::Pass(Value::Boolean(true)));
}

#[test]
fn add_eq() {
    let source_code = r##"10 + 20 === 30;"##;
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.result(), TestCaseResult::Pass(Value::Boolean(true)));
}
