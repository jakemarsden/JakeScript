#![feature(assert_matches)]
#![feature(process_exitcode_placeholder)]
#![feature(termination_trait_lib)]

use harness::FailureReason;
use jakescript::interpreter::{Error, Value};
use std::assert_matches::assert_matches;

pub mod harness;

#[test]
fn assertion_passes_for_truthy_value() {
    harness::init();
    assert_passes(r#"console.assert(true);"#);
    assert_passes(r#"console.assert(1);"#);
    assert_passes(r#"console.assert("a");"#);
    assert_passes(r#"console.assert({});"#);
}

#[test]
fn assertion_passes_for_truthy_value_with_detail_msg() {
    harness::init();
    assert_passes(r#"console.assert(true, "My failing assertion");"#);
    assert_passes(r#"console.assert(true, "Hello", "world", "foo", "bar");"#);
    assert_passes(r#"console.assert(true, {}, 13 + 4);"#);
}

#[test]
fn assertion_fails_for_falsy_value() {
    harness::init();
    assert_fails(r#"console.assert(false);"#, "");
    assert_fails(r#"console.assert(0);"#, "");
    assert_fails(r#"console.assert("");"#, "");
    assert_fails(r#"console.assert(null);"#, "");
    assert_fails(r#"console.assert(undefined);"#, "");
    assert_fails(r#"console.assert();"#, "");
}

#[test]
fn assertion_fails_for_falsy_value_with_detail_msg() {
    harness::init();
    assert_fails(
        r#"console.assert(false, "My failing assertion");"#,
        "My failing assertion",
    );
    assert_fails(
        r#"console.assert(false, "Hello", "world", "foo", "bar");"#,
        "Hello world foo bar",
    );
    assert_fails(
        r#"console.assert(false, {}, 13 + 4);"#,
        "[object Object] 17",
    );
}

fn assert_passes(source_code: &str) {
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.success_value(), Some(Value::Undefined));
}

fn assert_fails(source_code: &str, expected_detail_msg: &str) {
    let report = harness::exec_source_code(source_code);
    assert_matches!(
        report.failure_reason(),
        Some(FailureReason::Runtime(Error::Assertion(..)))
    );
    if let Some(FailureReason::Runtime(Error::Assertion(err))) = report.failure_reason() {
        assert_eq!(err.detail_msg(), expected_detail_msg);
    } else {
        unreachable!();
    }
}
