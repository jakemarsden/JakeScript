#![feature(assert_matches)]
#![feature(process_exitcode_placeholder)]
#![feature(termination_trait_lib)]

use harness::FailureReason;
use jakescript::interpreter::Error;
use std::assert_matches::assert_matches;

pub mod harness;

#[test]
fn assertion_fails() {
    harness::init();
    assert_fails(r#"console.assertNotReached();"#, "");
}

#[test]
fn assertion_fails_with_detail_msg() {
    harness::init();
    assert_fails(
        r#"console.assertNotReached("My failing assertion");"#,
        "My failing assertion",
    );
    assert_fails(
        r#"console.assertNotReached("Hello", "world", "foo", "bar");"#,
        "Hello world foo bar",
    );
    assert_fails(
        r#"console.assertNotReached({}, 13 + 4);"#,
        "[object Object] 17",
    );
}

fn assert_fails(source_code: &str, expected_detail_msg: &str) {
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
