#![feature(assert_matches)]

use jakescript::interpreter::{Error, Value};

mod common;

#[test]
fn test() {
    assertion_passes(r#"assert true;"#);
    assertion_passes(r#"assert 1;"#);
    assertion_passes(r#"assert "a";"#);

    assertion_fails(r#"assert false;"#);
    assertion_fails(r#"assert 0;"#);
    assertion_fails(r#"assert "";"#);
    assertion_fails(r#"assert null;"#);

    assertion_passes(r#"assert false || true;"#);
    assertion_fails(r#"assert true && false;"#);

    assertion_passes(r#"assert 17 === 0 + 10 + 7;"#);
}

fn assertion_passes(source_code: &str) {
    assert_matches!(
        common::eval_from_source_code(source_code),
        Ok(Value::Undefined)
    );
}

fn assertion_fails(source_code: &str) {
    assert_matches!(
        common::eval_from_source_code(source_code),
        Err((Error::AssertionFailed(..), _))
    );
}
