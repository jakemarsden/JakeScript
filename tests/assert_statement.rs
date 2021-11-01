#![feature(assert_matches)]

use harness::TestOutput;
use jakescript::interpreter::{Error, Value};
use std::assert_matches::assert_matches;

pub mod harness;

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
    let result = harness::exec_source_code(source_code);
    assert_matches!(result.output(), TestOutput::Pass(Value::Undefined));
}

fn assertion_fails(source_code: &str) {
    let result = harness::exec_source_code(source_code);
    assert_matches!(
        result.output(),
        TestOutput::InterpreterError(Error::AssertionFailed(..))
    );
}
