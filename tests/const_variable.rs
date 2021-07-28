#![feature(assert_matches)]

use jakescript::ast::Value;
use jakescript::interpreter::Error;

mod common;

#[test]
fn declare_const_variable_with_initialiser() {
    let source_code = r##"
const a = 10;
assert a === 10;
"##;
    let ast = common::parse_from_source_code(source_code);
    let result = common::eval(&ast);
    assert_matches!(result, Ok(Value::Undefined));
}

#[test]
fn set_initialised_const_variable() {
    let source_code = r##"
const a = 10;
a = 20;
"##;
    let ast = common::parse_from_source_code(source_code);
    let result = common::eval(&ast);
    assert_matches!(result, Err(Error::VariableIsConst(..)));
}
