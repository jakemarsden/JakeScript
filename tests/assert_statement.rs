#![feature(assert_matches)]

use jakescript::interpreter::{Error, Value};

mod common;

#[test]
fn assertion_passes_for_truthy_literal() {
    assertion_passes(r#"assert true;"#);
    assertion_passes(r#"assert 1;"#);
    assertion_passes(r#"assert "a";"#);
}

#[test]
fn assertion_fails_for_falsy_literal() {
    assertion_fails(r#"assert false;"#);
    assertion_fails(r#"assert 0;"#);
    assertion_fails(r#"assert "";"#);
    assertion_fails(r#"assert null;"#);
}

#[test]
fn assertion_passes_for_truthy_expression() {
    assertion_passes(r#"assert false || true;"#);
    assertion_passes(r#"assert 17 === 0 + 10 + 7;"#);
}

#[test]
fn assertion_fails_for_falsy_expression() {
    assertion_fails(r#"assert true && false;"#);
}

fn assertion_passes(source_code: &str) {
    assert_matches!(
        common::eval_from_source_code(source_code),
        (Ok(Value::Undefined), _)
    );
}

fn assertion_fails(source_code: &str) {
    assert_matches!(
        common::eval_from_source_code(source_code),
        (Err(Error::AssertionFailed(..)), _)
    );
}
