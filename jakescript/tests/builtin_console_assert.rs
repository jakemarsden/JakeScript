#![feature(assert_matches)]

use harness::FailureReason;
use jakescript::interpreter::{ErrorKind, Value};
use jakescript::token::SourceLocation;
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
fn assertion_passes_for_truthy_value_with_detail_msg() {
    harness::init();
    assert_passes(r#"console.assert(true, "msg");"#);
    assert_passes(r#"console.assert(1, "msg");"#);
    assert_passes(r#"console.assert("a", "msg");"#);
    assert_passes(r#"console.assert({}, "msg");"#);
    assert_passes(r#"console.assert(true, "Hello", "world", "foo", "bar");"#);
    assert_passes(r#"console.assert(true, {}, 13 + 4);"#);
}

#[test]
fn assertion_fails_for_falsy_value_with_detail_msg() {
    harness::init();
    assert_fails(r#"console.assert(false, "msg");"#, "msg");
    assert_fails(r#"console.assert(0, "msg");"#, "msg");
    assert_fails(r#"console.assert("", "msg");"#, "msg");
    assert_fails(r#"console.assert(null, "msg");"#, "msg");
    assert_fails(r#"console.assert(undefined, "msg");"#, "msg");
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
    let err = match report.failure_reason() {
        Some(FailureReason::Runtime(err)) => err,
        err => unreachable!("{:#?}", err),
    };
    if let ErrorKind::Assertion(err_source) = err.kind() {
        assert_eq!(err_source.detail_msg(), expected_detail_msg);
        assert_eq!(
            err.source_location(),
            &SourceLocation::at_start_of("untitled")
        );
    } else {
        unreachable!("{:#?}", err);
    }
}
