use jakescript::interpreter::Value;

mod common;

const SOURCE_CODE: &str = r##"
let a = /* Hello, */ 100;
let b = 50; // world!
a + b;
"##;

#[test]
fn test() {
    assert_eq!(common::eval_from_source(SOURCE_CODE), Value::Numeric(150));
}
