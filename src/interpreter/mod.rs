use crate::ast::*;

pub use vm::*;

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
///         rhs: Box::new(Expression::Member(MemberExpression::Literal(
///             Literal::Numeric(50),
///         ))),
///     },
/// ))]);
///
/// let mut it = Interpreter::default();
/// assert_eq!(program.eval(&mut it), Value::Numeric(150));
/// ```
pub trait Eval {
    fn eval(&self, it: &mut Interpreter) -> Value;
}

impl Eval for Program {
    fn eval(&self, it: &mut Interpreter) -> Value {
        self.0.eval(it)
    }
}

impl Eval for Vec<BlockItem> {
    fn eval(&self, it: &mut Interpreter) -> Value {
        let mut result = Value::default();
        for item in self.iter() {
            match item {
                BlockItem::Declaration(..) => {
                    item.eval(it);
                }
                BlockItem::Statement(..) => {
                    result = item.eval(it);
                }
            }
        }
        result
    }
}

impl Eval for BlockItem {
    fn eval(&self, it: &mut Interpreter) -> Value {
        match self {
            Self::Declaration(decl) => decl.eval(it),
            Self::Statement(stmt) => stmt.eval(it),
        }
    }
}

impl Eval for Declaration {
    fn eval(&self, _it: &mut Interpreter) -> Value {
        todo!("eval: decl: {:?}", self)
    }
}

impl Eval for Statement {
    fn eval(&self, it: &mut Interpreter) -> Value {
        match self {
            Self::Block(items) => items.eval(it),
            Self::Expression(expr) => expr.eval(it),
            stmt => todo!("eval: stmt: {:?}", stmt),
        }
    }
}

impl Eval for Expression {
    fn eval(&self, it: &mut Interpreter) -> Value {
        match self {
            Self::BinaryOp { kind, lhs, rhs } => {
                let lhs = lhs.eval(it);
                let rhs = rhs.eval(it);
                match kind {
                    BinaryOp::Add => it.add(lhs, rhs),
                    kind => todo!("eval: binary_op: {:?}", kind),
                }
            }
            Self::Member(expr) => expr.eval(it),
            expr => todo!("eval: expr: {:?}", expr),
        }
    }
}

impl Eval for MemberExpression {
    fn eval(&self, _it: &mut Interpreter) -> Value {
        match self {
            MemberExpression::Literal(lit) => match lit {
                Literal::Boolean(value) => Value::Boolean(*value),
                Literal::Null => Value::Null,
                Literal::Numeric(value) => Value::Numeric(*value),
                Literal::String(value) => Value::String(value.clone()),
            },
            expr => todo!("eval: member_expr: {:?}", expr),
        }
    }
}
