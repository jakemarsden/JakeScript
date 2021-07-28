use crate::ast::*;
use std::cmp::Ordering;

pub use error::*;
pub use vm::*;

mod error;
mod vm;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
pub struct Interpreter {
    vm: Vm,
}

impl Interpreter {
    pub fn add(&self, lhs: Value, rhs: Value) -> Value {
        Value::Numeric(lhs.as_numeric() + rhs.as_numeric())
    }

    pub fn sub(&self, lhs: Value, rhs: Value) -> Value {
        Value::Numeric(lhs.as_numeric() - rhs.as_numeric())
    }

    pub fn mul(&self, lhs: Value, rhs: Value) -> Value {
        Value::Numeric(lhs.as_numeric() * rhs.as_numeric())
    }

    pub fn r#mod(&self, lhs: Value, rhs: Value) -> Value {
        Value::Numeric(lhs.as_numeric() % rhs.as_numeric())
    }

    pub fn div(&self, lhs: Value, rhs: Value) -> Value {
        Value::Numeric(lhs.as_numeric() / rhs.as_numeric())
    }

    pub fn pow(&self, lhs: Value, rhs: Value) -> Value {
        let lhs = lhs.as_numeric();
        let rhs = rhs.as_numeric();
        assert!(rhs >= i32::MIN as i64 && rhs <= i32::MAX as i64);
        let result = (lhs as f64).powi(rhs as i32);
        Value::Numeric(result as i64)
    }

    pub fn compare(&self, lhs: Value, rhs: Value) -> Ordering {
        lhs.as_numeric().cmp(&rhs.as_numeric())
    }

    pub fn is_identical(&self, lhs: Value, rhs: Value) -> Value {
        Value::Boolean(match (lhs, rhs) {
            (Value::Boolean(lhs), Value::Boolean(rhs)) => lhs == rhs,
            (Value::Null, Value::Null) => true,
            (Value::Numeric(lhs), Value::Numeric(rhs)) => lhs == rhs,
            (Value::String(lhs), Value::String(rhs)) => lhs == rhs,
            (Value::Undefined, Value::Undefined) => true,
            (_, _) => false,
        })
    }

    pub fn vm(&mut self) -> &mut Vm {
        &mut self.vm
    }
}

/// ```rust
/// # use jakescript::ast::*;
/// # use jakescript::interpreter::*;
/// let program = Program::new(Block::new(vec![Statement::Expression(Expression::Binary(
///     BinaryExpression {
///         kind: BinaryOp::Add,
///         lhs: Box::new(Expression::Literal(Literal::Numeric(100))),
///         rhs: Box::new(Expression::Binary(BinaryExpression {
///             kind: BinaryOp::Add,
///             lhs: Box::new(Expression::Literal(Literal::Numeric(50))),
///             rhs: Box::new(Expression::Literal(Literal::Numeric(17))),
///         })),
///     },
/// ))]));
///
/// let mut it = Interpreter::default();
/// assert_eq!(program.eval(&mut it), Ok(Value::Numeric(167)));
/// ```
///
/// ```rust
/// # use jakescript::ast::*;
/// # use jakescript::interpreter::*;
/// let program = Program::new(Block::new(vec![
///     Statement::VariableDeclaration {
///         kind: VariableDeclKind::Let,
///         var_name: "a".to_owned(),
///         initialiser: Some(Expression::Literal(Literal::Numeric(100))),
///     },
///     Statement::VariableDeclaration {
///         kind: VariableDeclKind::Let,
///         var_name: "b".to_owned(),
///         initialiser: Some(Expression::Literal(Literal::Numeric(50))),
///     },
///     Statement::Expression(Expression::Binary(BinaryExpression {
///         kind: BinaryOp::Add,
///         lhs: Box::new(Expression::VariableAccess("a".to_owned())),
///         rhs: Box::new(Expression::VariableAccess("b".to_owned())),
///     })),
/// ]));
///
/// let mut it = Interpreter::default();
/// assert_eq!(program.eval(&mut it), Ok(Value::Numeric(150)));
/// ```
pub trait Eval {
    fn eval(&self, it: &mut Interpreter) -> Result<Value>;
}

impl Eval for Program {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        self.block().eval(it)
    }
}

impl Eval for Block {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        let mut result = Value::Undefined;
        for stmt in self.iter() {
            result = stmt.eval(it)?;
        }
        Ok(result)
    }
}

impl Eval for Statement {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        match self {
            Self::Assertion(node) => node.eval(it),
            Self::Block(node) => node.eval(it),
            Self::Expression(expr) => expr.eval(it),
            Self::If {
                condition,
                success_block,
                else_block,
            } => {
                let condition = condition.eval(it)?;
                if condition.as_boolean() {
                    it.vm().push_scope();
                    success_block.eval(it)?;
                    it.vm().pop_scope();
                } else if let Some(else_block) = else_block {
                    it.vm().push_scope();
                    else_block.eval(it)?;
                    it.vm().pop_scope();
                }
                Ok(Value::Undefined)
            }
            Self::VariableDeclaration {
                kind,
                var_name,
                initialiser,
            } => {
                let value = if let Some(initialiser) = initialiser {
                    initialiser.eval(it)?
                } else {
                    Value::Undefined
                };
                it.vm()
                    .peek_scope_mut()
                    .init_variable(*kind, var_name.clone(), value)?;
                Ok(Value::Undefined)
            }
            Self::WhileLoop { condition, block } => {
                loop {
                    let condition = condition.eval(it)?;
                    if condition.as_boolean() {
                        it.vm().push_scope();
                        block.eval(it)?;
                        it.vm().pop_scope();
                    } else {
                        break;
                    }
                }
                Ok(Value::Undefined)
            }
        }
    }
}

impl Eval for Assertion {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        let value = self.condition.eval(it)?;
        if value.as_boolean() {
            Ok(Value::Undefined)
        } else {
            Err(AssertionFailedError::new(value).into())
        }
    }
}

impl Eval for Expression {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        match self {
            Self::Assignment(ref node) => node.eval(it),
            Self::Binary(ref node) => node.eval(it),
            Self::Unary(ref node) => node.eval(it),

            Self::Literal(lit) => Ok(match lit {
                Literal::Boolean(value) => Value::Boolean(*value),
                Literal::Null => Value::Null,
                Literal::Numeric(value) => Value::Numeric(*value),
                Literal::String(value) => Value::String(value.clone()),
            }),
            Self::PropertyAccess { .. } => {
                todo!("Expression::eval: {:?}", self)
            }
            Self::VariableAccess(ref var_name) => {
                let value = it.vm().peek_scope().resolve_variable(var_name)?;
                Ok(value.clone())
            }
        }
    }
}

impl Eval for AssignmentExpression {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        let var_name = match self.lhs.as_ref() {
            Expression::VariableAccess(ref var_name) => var_name,
            lhs => todo!("Expression::eval: assignment_op: lhs={:?}", lhs),
        };
        let lhs = it.vm().peek_scope().resolve_variable(var_name)?.clone();
        let rhs = self.rhs.eval(it)?;
        let value = match self.kind {
            AssignmentOp::Assign => rhs,
            AssignmentOp::AddAssign => it.add(lhs, rhs),
            kind => todo!("Expression::eval: kind={:?}", kind),
        };
        it.vm()
            .peek_scope_mut()
            .set_variable(var_name, value.clone())?;
        Ok(value)
    }
}

impl Eval for BinaryExpression {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        let lhs = self.lhs.eval(it)?;
        let rhs = self.rhs.eval(it)?;
        Ok(match self.kind {
            BinaryOp::Add => it.add(lhs, rhs),
            BinaryOp::Sub => it.sub(lhs, rhs),
            BinaryOp::Mul => it.mul(lhs, rhs),
            BinaryOp::Div => it.div(lhs, rhs),
            BinaryOp::Mod => it.r#mod(lhs, rhs),
            BinaryOp::Pow => it.pow(lhs, rhs),
            BinaryOp::Identical => it.is_identical(lhs, rhs),
            BinaryOp::LessThan => Value::Boolean(it.compare(lhs, rhs).is_lt()),
            BinaryOp::LessThanOrEqual => Value::Boolean(it.compare(lhs, rhs).is_le()),
            BinaryOp::LogicalAnd => Value::Boolean(lhs.as_boolean() && rhs.as_boolean()),
            BinaryOp::LogicalOr => Value::Boolean(lhs.as_boolean() || rhs.as_boolean()),
            BinaryOp::MoreThan => Value::Boolean(it.compare(lhs, rhs).is_gt()),
            BinaryOp::MoreThanOrEqual => Value::Boolean(it.compare(lhs, rhs).is_ge()),
            BinaryOp::NotIdentical => !it.is_identical(lhs, rhs),
            kind => todo!("Expression::eval: kind={:?}", kind),
        })
    }
}

impl Eval for UnaryExpression {
    fn eval(&self, _it: &mut Interpreter) -> Result<Value> {
        todo!("UnaryExpression::eval: {:?}", self)
    }
}
