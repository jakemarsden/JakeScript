#![feature(assert_matches)]

use harness::FailureReason;
use jakescript::interpreter::{ErrorKind, Value};
use jakescript::token::SourceLocation;
use std::assert_matches::assert_matches;

pub mod harness;

#[test]
fn assertion_passes_for_equal_values() {
    harness::init();
    assert_passes(r#"console.assertEqual(true, true);"#);
    assert_passes(r#"console.assertEqual(1, 1);"#);
    assert_passes(r#"console.assertEqual("a", "a");"#);
    assert_passes(r#"console.assertEqual(NaN, NaN);"#);
    assert_passes(r#"console.assertEqual(undefined, undefined);"#);
    assert_passes(r#"console.assertEqual(true);"#);
}

#[test]
fn assertion_fails_for_unequal_values() {
    harness::init();
    assert_fails(
        r#"console.assertEqual(1, true);"#,
        "expected 'true' but was '1': ",
    );
    assert_fails(
        r#"console.assertEqual(0, 1);"#,
        "expected '1' but was '0': ",
    );
    assert_fails(
        r#"console.assertEqual("b", "a");"#,
        "expected 'a' but was 'b': ",
    );
    assert_fails(
        r#"console.assertEqual("1", "NaN");"#,
        "expected 'NaN' but was '1': ",
    );
    assert_fails(
        r#"console.assertEqual(undefined, true);"#,
        "expected 'true' but was 'undefined': ",
    );
    assert_fails(
        r#"console.assertEqual(1);"#,
        "expected 'true' but was '1': ",
    );
    assert_fails(
        r#"console.assertEqual();"#,
        "expected 'true' but was 'undefined': ",
    );
}

#[test]
fn assertion_passes_for_equal_values_with_detail_msg() {
    harness::init();
    assert_passes(r#"console.assertEqual(true, true, "msg");"#);
    assert_passes(r#"console.assertEqual(1, 1, "msg");"#);
    assert_passes(r#"console.assertEqual("a", "a", "msg");"#);
    assert_passes(r#"console.assertEqual(NaN, NaN, "msg");"#);
    assert_passes(r#"console.assertEqual(undefined, undefined, "msg");"#);
    assert_passes(r#"console.assertEqual(true, true, "Hello", "world", "foo", "bar");"#);
    assert_passes(r#"console.assertEqual(true, true, {}, 13 + 4);"#);
}

#[test]
fn assertion_fails_for_unequal_values_with_detail_msg() {
    harness::init();
    assert_fails(
        r#"console.assertEqual(1, true, "msg");"#,
        "expected 'true' but was '1': msg",
    );
    assert_fails(
        r#"console.assertEqual(0, 1, "msg");"#,
        "expected '1' but was '0': msg",
    );
    assert_fails(
        r#"console.assertEqual("b", "a", "msg");"#,
        "expected 'a' but was 'b': msg",
    );
    assert_fails(
        r#"console.assertEqual("1", "NaN", "msg");"#,
        "expected 'NaN' but was '1': msg",
    );
    assert_fails(
        r#"console.assertEqual(undefined, true, "msg");"#,
        "expected 'true' but was 'undefined': msg",
    );
    assert_fails(
        r#"console.assertEqual(1, true, "Hello", "world", "foo", "bar");"#,
        "expected 'true' but was '1': Hello world foo bar",
    );
    assert_fails(
        r#"console.assertEqual(1, true, {}, 13 + 4);"#,
        "expected 'true' but was '1': [object Object] 17",
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
        err => unreachable!("{err:#?}"),
    };
    if let ErrorKind::Assertion(err_source) = err.kind() {
        assert_eq!(err_source.detail_msg(), expected_detail_msg);
        assert_eq!(
            err.source_location(),
            &SourceLocation::at_start_of("untitled")
        );
    } else {
        unreachable!("{err:#?}");
    }
}
