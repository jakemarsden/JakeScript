use jakescript::interpreter::Value;

mod common;

const SOURCE_CODE: &str = r##"
100 + 50 + 17;
"##;

#[test]
fn test() {
    assert_eq!(common::eval_from_source(SOURCE_CODE), Value::Numeric(167));
}
