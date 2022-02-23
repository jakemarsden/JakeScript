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
    assertion_passes(r#"console.assert(true);"#);
    assertion_passes(r#"console.assert(1);"#);
    assertion_passes(r#"console.assert("a");"#);
    assertion_passes(r#"console.assert({});"#);
}

#[test]
fn assertion_passes_for_truthy_value_with_detail_msg() {
    harness::init();
    assertion_passes(r#"console.assert(true, "My failing assertion");"#);
    assertion_passes(r#"console.assert(true, "Hello", "world", "foo", "bar");"#);
    assertion_passes(r#"console.assert(true, {}, 13 + 4);"#);
}

#[test]
fn assertion_fails_for_falsy_value() {
    harness::init();
    assertion_fails(r#"console.assert(false);"#, "");
    assertion_fails(r#"console.assert(0);"#, "");
    assertion_fails(r#"console.assert("");"#, "");
    assertion_fails(r#"console.assert(null);"#, "");
    assertion_fails(r#"console.assert(undefined);"#, "");
    assertion_fails(r#"console.assert();"#, "");
}

#[test]
fn assertion_fails_for_falsy_value_with_detail_msg() {
    harness::init();
    assertion_fails(
        r#"console.assert(false, "My failing assertion");"#,
        "My failing assertion",
    );
    assertion_fails(
        r#"console.assert(false, "Hello", "world", "foo", "bar");"#,
        "Hello world foo bar",
    );
    assertion_fails(
        r#"console.assert(false, {}, 13 + 4);"#,
        "[object Object] 17",
    );
}

fn assertion_passes(source_code: &str) {
    let report = harness::exec_source_code(source_code);
    assert_matches!(report.success_value(), Some(Value::Undefined));
}

fn assertion_fails(source_code: &str, expected_detail_msg: &str) {
    let report = harness::exec_source_code(source_code);
    assert_matches!(
        report.failure_reason(),
        Some(FailureReason::Runtime(Error::AssertionFailed(..)))
    );
    if let Some(FailureReason::Runtime(Error::AssertionFailed(err))) = report.failure_reason() {
        assert_eq!(err.detail_msg(), expected_detail_msg);
    } else {
        unreachable!();
    }
}
