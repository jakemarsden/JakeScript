use jakescript::ast::*;
use jakescript::interpreter::Value;

mod common;

#[test]
fn add_mul() {
    let source_code = r##"2 + 3 * 4"##;
    let ast = common::parse_from_source_code(source_code);
    assert_eq!(
        ast,
        Program(vec![BlockItem::Statement(Statement::Expression(
            Expression::BinaryOp {
                kind: BinaryOp::Add,
                lhs: Box::new(Expression::Member(MemberExpression::Literal(
                    Literal::Numeric(2)
                ))),
                rhs: Box::new(Expression::BinaryOp {
                    kind: BinaryOp::Mul,
                    lhs: Box::new(Expression::Member(MemberExpression::Literal(
                        Literal::Numeric(3)
                    ))),
                    rhs: Box::new(Expression::Member(MemberExpression::Literal(
                        Literal::Numeric(4)
                    ))),
                }),
            }
        ))])
    );

    let result = common::eval(&ast);
    assert_eq!(result, Ok(Value::Numeric(14)));
}

#[test]
fn mul_add() {
    let source_code = r##"2 * 3 + 4"##;
    let ast = common::parse_from_source_code(source_code);
    assert_eq!(
        ast,
        Program(vec![BlockItem::Statement(Statement::Expression(
            Expression::BinaryOp {
                kind: BinaryOp::Add,
                lhs: Box::new(Expression::BinaryOp {
                    kind: BinaryOp::Mul,
                    lhs: Box::new(Expression::Member(MemberExpression::Literal(
                        Literal::Numeric(2)
                    ))),
                    rhs: Box::new(Expression::Member(MemberExpression::Literal(
                        Literal::Numeric(3)
                    ))),
                }),
                rhs: Box::new(Expression::Member(MemberExpression::Literal(
                    Literal::Numeric(4)
                ))),
            }
        ))])
    );

    let result = common::eval(&ast);
    assert_eq!(result, Ok(Value::Numeric(10)));
}

#[test]
fn eq_add() {
    let source_code = r##"
30 === 10 + 20;
"##;
    let ast = common::parse_from_source_code(source_code);
    assert_eq!(
        ast,
        Program(vec![BlockItem::Statement(Statement::Expression(
            Expression::BinaryOp {
                kind: BinaryOp::Identical,
                lhs: Box::new(Expression::Member(MemberExpression::Literal(
                    Literal::Numeric(30)
                ))),
                rhs: Box::new(Expression::BinaryOp {
                    kind: BinaryOp::Add,
                    lhs: Box::new(Expression::Member(MemberExpression::Literal(
                        Literal::Numeric(10)
                    ))),
                    rhs: Box::new(Expression::Member(MemberExpression::Literal(
                        Literal::Numeric(20)
                    ))),
                }),
            }
        ))])
    );

    let result = common::eval(&ast);
    assert_eq!(result, Ok(Value::Boolean(true)));
}

#[test]
fn add_eq() {
    let source_code = r##"
10 + 20 === 30;
"##;
    let ast = common::parse_from_source_code(source_code);
    assert_eq!(
        ast,
        Program(vec![BlockItem::Statement(Statement::Expression(
            Expression::BinaryOp {
                kind: BinaryOp::Identical,
                lhs: Box::new(Expression::BinaryOp {
                    kind: BinaryOp::Add,
                    lhs: Box::new(Expression::Member(MemberExpression::Literal(
                        Literal::Numeric(10)
                    ))),
                    rhs: Box::new(Expression::Member(MemberExpression::Literal(
                        Literal::Numeric(20)
                    ))),
                }),
                rhs: Box::new(Expression::Member(MemberExpression::Literal(
                    Literal::Numeric(30)
                ))),
            }
        ))])
    );

    let result = common::eval(&ast);
    assert_eq!(result, Ok(Value::Boolean(true)));
}
