#![feature(assert_matches)]

use harness::FailureReason;
use jakescript::interpreter::ErrorKind;
use jakescript::token::{SourceLocation, SourcePosition};

pub mod harness;

#[test]
fn assertion_fails() {
    harness::init();
    assert_fails(
        r#"console.assertNotReached();"#,
        "entered unreachable code: ",
        SourcePosition::at(0, 24),
    );
}

#[test]
fn assertion_fails_with_detail_msg() {
    harness::init();
    assert_fails(
        r#"console.assertNotReached("msg");"#,
        "entered unreachable code: msg",
        SourcePosition::at(0, 24),
    );
    assert_fails(
        r#"console.assertNotReached("Hello", "world", "foo", "bar");"#,
        "entered unreachable code: Hello world foo bar",
        SourcePosition::at(0, 24),
    );
    assert_fails(
        r#"console.assertNotReached({}, 13 + 4);"#,
        "entered unreachable code: [object Object] 17",
        SourcePosition::at(0, 24),
    );
}

fn assert_fails(source_code: &str, expected_detail_msg: &str, fail_at: SourcePosition) {
    let report = harness::exec_source_code(source_code);
    let err = match report.failure_reason() {
        Some(FailureReason::Runtime(err)) => err,
        err => unreachable!("{err:#?}"),
    };
    if let ErrorKind::Assertion(err_source) = err.kind() {
        assert_eq!(err_source.detail_msg(), expected_detail_msg);
        assert_eq!(
            err.source_location(),
            &SourceLocation::new("untitled", fail_at)
        );
    } else {
        unreachable!("{err:#?}",);
    }
}
