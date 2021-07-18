use crate::ast::*;

pub use error::*;
pub use vm::*;

mod error;
mod vm;

#[derive(Default)]
pub struct Interpreter {
    vm: Vm,
}

impl Interpreter {
    pub fn add(&self, lhs: Value, rhs: Value) -> Value {
        match (lhs, rhs) {
            (Value::Numeric(lhs), Value::Numeric(rhs)) => Value::Numeric(lhs + rhs),
            (lhs, rhs) => todo!("add: {:?}, {:?}", lhs, rhs),
        }
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
/// let program = Program(vec![BlockItem::Statement(Statement::Expression(
///     Expression::BinaryOp {
///         kind: BinaryOp::Add,
///         lhs: Box::new(Expression::Member(MemberExpression::Literal(
///             Literal::Numeric(100),
///         ))),
///         rhs: Box::new(Expression::BinaryOp {
///             kind: BinaryOp::Add,
///             lhs: Box::new(Expression::Member(MemberExpression::Literal(
///                 Literal::Numeric(50),
///             ))),
///             rhs: Box::new(Expression::Member(MemberExpression::Literal(
///                 Literal::Numeric(17),
///             ))),
///         }),
///     },
/// ))]);
///
/// let mut it = Interpreter::default();
/// assert_eq!(program.eval(&mut it), Ok(Value::Numeric(167)));
/// ```
///
/// ```rust
/// # use jakescript::ast::*;
/// # use jakescript::interpreter::*;
/// let program = Program(vec![
///     BlockItem::Declaration(Declaration::Variable {
///         kind: VariableDeclKind::Let,
///         var_name: "a".to_owned(),
///         initialiser: Some(Expression::Member(MemberExpression::Literal(
///             Literal::Numeric(100),
///         ))),
///     }),
///     BlockItem::Declaration(Declaration::Variable {
///         kind: VariableDeclKind::Let,
///         var_name: "b".to_owned(),
///         initialiser: Some(Expression::Member(MemberExpression::Literal(
///             Literal::Numeric(50),
///         ))),
///     }),
///     BlockItem::Statement(Statement::Expression(Expression::BinaryOp {
///         kind: BinaryOp::Add,
///         lhs: Box::new(Expression::Member(MemberExpression::Identifier(
///             "a".to_owned(),
///         ))),
///         rhs: Box::new(Expression::Member(MemberExpression::Identifier(
///             "b".to_owned(),
///         ))),
///     })),
/// ]);
///
/// let mut it = Interpreter::default();
/// assert_eq!(program.eval(&mut it), Ok(Value::Numeric(150)));
/// ```
pub trait Eval {
    fn eval(&self, it: &mut Interpreter) -> Result<Value>;
}

impl Eval for Program {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        self.0.eval(it)
    }
}

impl Eval for Vec<BlockItem> {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        let mut result = Value::Undefined;
        for item in self.iter() {
            match item {
                BlockItem::Declaration(..) => {
                    item.eval(it)?;
                }
                BlockItem::Statement(..) => {
                    result = item.eval(it)?;
                }
            }
        }
        Ok(result)
    }
}

impl Eval for BlockItem {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        match self {
            Self::Declaration(decl) => decl.eval(it),
            Self::Statement(stmt) => stmt.eval(it),
        }
    }
}

impl Eval for Declaration {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        match self {
            Self::Variable {
                kind,
                var_name,
                initialiser,
            } => {
                match kind {
                    VariableDeclKind::Let => {}
                    kind => todo!("eval: decl: {:?}", kind),
                };
                let value = if let Some(initialiser) = initialiser {
                    initialiser.eval(it)?
                } else {
                    Value::Undefined
                };
                it.vm()
                    .peek_scope_mut()
                    .init_local(var_name.clone(), value)?;
                Ok(Value::Undefined)
            }
        }
    }
}

impl Eval for Statement {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        match self {
            Self::Assertion { condition } => {
                let value = condition.eval(it)?;
                if value.as_boolean() {
                    Ok(Value::Undefined)
                } else {
                    Err(AssertionFailedError::new(value).into())
                }
            }
            Self::Block(items) => items.eval(it),
            Self::Expression(expr) => expr.eval(it),
            stmt => todo!("eval: stmt: {:?}", stmt),
        }
    }
}

impl Eval for Expression {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        match self {
            Self::BinaryOp { kind, lhs, rhs } => {
                let lhs = lhs.eval(it)?;
                let rhs = rhs.eval(it)?;
                Ok(match kind {
                    BinaryOp::Add => it.add(lhs, rhs),
                    BinaryOp::Identical => it.is_identical(lhs, rhs),
                    kind => todo!("eval: binary_op: {:?}", kind),
                })
            }
            Self::Member(expr) => expr.eval(it),
            expr => todo!("eval: expr: {:?}", expr),
        }
    }
}

impl Eval for MemberExpression {
    fn eval(&self, it: &mut Interpreter) -> Result<Value> {
        match self {
            MemberExpression::Identifier(ref name) => {
                let value = it.vm().peek_scope().lookup_local(name)?;
                Ok(value.clone())
            }
            MemberExpression::Literal(lit) => Ok(match lit {
                Literal::Boolean(value) => Value::Boolean(*value),
                Literal::Null => Value::Null,
                Literal::Numeric(value) => Value::Numeric(*value),
                Literal::String(value) => Value::String(value.clone()),
            }),
            expr => todo!("eval: member_expr: {:?}", expr),
        }
    }
}
