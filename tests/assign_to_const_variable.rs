#![feature(assert_matches)]

use jakescript::interpreter::Error;

mod common;

const SOURCE_CODE: &str = r##"
const a = 10;
a = 20;
"##;

#[test]
fn test() {
    assert_matches!(
        common::eval_from_source_code(SOURCE_CODE),
        Err((Error::VariableIsConst(..), ..))
    );
}
