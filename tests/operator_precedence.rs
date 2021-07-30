use jakescript::ast::*;

mod common;

#[test]
fn add_add() {
    let source_code = r##"50 + 100 + 17"##;
    let ast = common::parse_from_source_code(source_code);
    let result = common::eval(&ast);
    assert_eq!(result, Ok(Value::Numeric(167)));
}

#[test]
fn add_mul() {
    let source_code = r##"2 + 3 * 4"##;
    let ast = common::parse_from_source_code(source_code);
    let result = common::eval(&ast);
    assert_eq!(result, Ok(Value::Numeric(14)));
}

#[test]
fn mul_add() {
    let source_code = r##"2 * 3 + 4"##;
    let ast = common::parse_from_source_code(source_code);
    let result = common::eval(&ast);
    assert_eq!(result, Ok(Value::Numeric(10)));
}

#[test]
fn eq_add() {
    let source_code = r##"30 === 10 + 20;"##;
    let ast = common::parse_from_source_code(source_code);
    let result = common::eval(&ast);
    assert_eq!(result, Ok(Value::Boolean(true)));
}

#[test]
fn add_eq() {
    let source_code = r##"10 + 20 === 30;"##;
    let ast = common::parse_from_source_code(source_code);
    let result = common::eval(&ast);
    assert_eq!(result, Ok(Value::Boolean(true)));
}
