use crate::ast::{
    AssertStatement, AssignmentExpression, AssignmentOperator, Associativity, BinaryExpression,
    BinaryOperator, Block, BreakStatement, CatchBlock, ComputedPropertyAccessExpression,
    ContinueStatement, DeclarationStatement, ExitStatement, Expression, FinallyBlock, ForLoop,
    FunctionCallExpression, FunctionDeclaration, GroupingExpression, Identifier, IfStatement,
    Literal, LiteralExpression, Node, NumericLiteral, Op, PrintStatement, Program,
    PropertyAccessExpression, ReturnStatement, Statement, TernaryExpression, ThrowStatement,
    TryStatement, UnaryExpression, UnaryOperator, VariableAccessExpression, VariableDeclaration,
    WhileLoop,
};
use std::assert_matches::assert_matches;
use std::hint::unreachable_unchecked;

pub use error::*;
pub use heap::*;
pub use stack::*;
pub use value::*;
pub use vm::*;

mod error;
mod global_scope;
mod heap;
mod stack;
mod value;
mod vm;

#[derive(Default)]
pub struct Interpreter {
    vm: Vm,
}

impl Interpreter {
    pub fn vm(&self) -> &Vm {
        &self.vm
    }
    pub fn vm_mut(&mut self) -> &mut Vm {
        &mut self.vm
    }
}

pub trait Eval: Node {
    type Output = ();

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output>;
}

impl Eval for Program {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        self.body().eval(it)
    }
}

impl Eval for Block {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let mut result = Value::default();
        for decl in self.hoisted_declarations() {
            if !matches!(it.vm().execution_state(), ExecutionState::Advance) {
                break;
            }
            assert!(decl.is_hoisted());
            decl.eval(it)?;
        }
        for stmt in self.statements() {
            if !matches!(it.vm().execution_state(), ExecutionState::Advance) {
                break;
            }
            if let Statement::Declaration(decl) = stmt {
                assert!(!decl.is_hoisted());
            }
            result = match stmt {
                Statement::Expression(expr) => expr.eval(it),
                stmt => stmt.eval(it).map(|()| Value::default()),
            }?;
        }
        Ok(result)
    }
}

impl Eval for Statement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        match self {
            Self::Assert(node) => node.eval(it),
            Self::Break(node) => node.eval(it),
            Self::Continue(node) => node.eval(it),
            Self::Declaration(node) => node.eval(it),
            Self::Expression(node) => node.eval(it).map(|_| ()),
            Self::Exit(node) => node.eval(it),
            Self::If(node) => node.eval(it),
            Self::Print(node) => node.eval(it),
            Self::Return(node) => node.eval(it),
            Self::Throw(node) => node.eval(it),
            Self::Try(node) => node.eval(it),
            Self::ForLoop(node) => node.eval(it),
            Self::WhileLoop(node) => node.eval(it),
        }
    }
}

impl Eval for DeclarationStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        match self {
            DeclarationStatement::Function(node) => node.eval(it),
            DeclarationStatement::Variable(node) => node.eval(it),
        }
    }
}

impl Eval for AssertStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let value = self.condition.eval(it)?;
        if value.is_truthy() {
            Ok(())
        } else {
            Err(AssertionFailedError::new(self.condition.clone(), value).into())
        }
    }
}

impl Eval for ExitStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        it.vm_mut().set_execution_state(ExecutionState::Exit);
        Ok(())
    }
}

impl Eval for PrintStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let value = self.argument.eval(it)?;
        let string_value = value.coerce_to_string(it);
        // Note: Print to stderr as stdout is swallowed when running in the REPL.
        if self.new_line {
            eprintln!("{}", string_value);
        } else {
            eprint!("{}", string_value);
        }
        Ok(())
    }
}

impl Eval for IfStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let condition = self.condition.eval(it)?;
        if condition.is_truthy() {
            it.vm_mut().stack_mut().frame_mut().push_empty_scope();
            self.success_block.eval(it)?;
            it.vm_mut().stack_mut().frame_mut().pop_scope();
        } else if let Some(ref else_block) = self.else_block {
            it.vm_mut().stack_mut().frame_mut().push_empty_scope();
            else_block.eval(it)?;
            it.vm_mut().stack_mut().frame_mut().pop_scope();
        }
        Ok(())
    }
}

impl Eval for ForLoop {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        if let Some(ref initialiser) = self.initialiser {
            it.vm_mut().stack_mut().frame_mut().push_empty_scope();
            initialiser.eval(it)?;
        }
        loop {
            if let Some(ref condition) = self.condition {
                let condition = condition.eval(it)?;
                if condition.is_falsy() {
                    break;
                }
            }

            it.vm_mut().stack_mut().frame_mut().push_empty_scope();
            self.body.eval(it)?;
            it.vm_mut().stack_mut().frame_mut().pop_scope();

            if let Some(ref incrementor) = self.incrementor {
                incrementor.eval(it)?;
            }

            match it.vm().execution_state() {
                ExecutionState::Advance => {}
                ExecutionState::Break => {
                    it.vm_mut().reset_execution_state();
                    break;
                }
                ExecutionState::BreakContinue => {
                    it.vm_mut().reset_execution_state();
                    continue;
                }
                ExecutionState::Return(_) | ExecutionState::Exception(_) | ExecutionState::Exit => {
                    // Exit the loop, but don't reset the execution state just yet so that it can be
                    // handled/cleared by some calling AST node.
                    break;
                }
            }
        }
        if self.initialiser.is_some() {
            it.vm_mut().stack_mut().frame_mut().pop_scope();
        }
        Ok(())
    }
}

impl Eval for WhileLoop {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        loop {
            let condition = self.condition.eval(it)?;
            if condition.is_falsy() {
                break;
            }

            it.vm_mut().stack_mut().frame_mut().push_empty_scope();
            self.body.eval(it)?;
            it.vm_mut().stack_mut().frame_mut().pop_scope();

            match it.vm().execution_state() {
                ExecutionState::Advance => {}
                ExecutionState::Break => {
                    it.vm_mut().reset_execution_state();
                    break;
                }
                ExecutionState::BreakContinue => {
                    it.vm_mut().reset_execution_state();
                    continue;
                }
                ExecutionState::Return(_) | ExecutionState::Exception(_) | ExecutionState::Exit => {
                    // Exit the loop, but don't reset the execution state just yet so that it can be
                    // handled/cleared by some calling AST node.
                    break;
                }
            }
        }
        Ok(())
    }
}

impl Eval for BreakStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        it.vm_mut().set_execution_state(ExecutionState::Break);
        Ok(())
    }
}

impl Eval for ContinueStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        it.vm_mut()
            .set_execution_state(ExecutionState::BreakContinue);
        Ok(())
    }
}

impl Eval for ReturnStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let value = if let Some(ref expr) = self.expr {
            expr.eval(it)?
        } else {
            Value::Undefined
        };
        it.vm_mut()
            .set_execution_state(ExecutionState::Return(value));
        Ok(())
    }
}

impl Eval for ThrowStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let ex = self.exception.eval(it)?;
        it.vm_mut()
            .set_execution_state(ExecutionState::Exception(ex));
        Ok(())
    }
}

impl Eval for TryStatement {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        self.body.eval(it)?;
        if let Some(ref catch) = self.catch_block {
            if matches!(it.vm().execution_state(), ExecutionState::Exception(..)) {
                catch.eval(it)?;
            }
        }
        if let Some(ref finally) = self.finally_block {
            finally.eval(it)?;
        }
        Ok(())
    }
}

impl Eval for CatchBlock {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let exception = it.vm_mut().clear_exception().unwrap();
        if let Some(ref exception_var_name) = self.exception_identifier {
            let exception_var =
                Variable::new(VariableKind::Let, exception_var_name.clone(), exception);
            let scope_ctx = ScopeCtx::new(vec![exception_var]);
            it.vm_mut()
                .stack_mut()
                .frame_mut()
                .push_scope(scope_ctx, false);
        }
        self.body.eval(it)?;
        if self.exception_identifier.is_some() {
            it.vm_mut().stack_mut().frame_mut().pop_scope();
        }
        Ok(())
    }
}

impl Eval for FinallyBlock {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        it.vm_mut().hide_current_exception();
        let result = self.inner.eval(it).map(|_| ());
        it.vm_mut().restore_hidden_exception();
        result
    }
}

impl Eval for FunctionDeclaration {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let declared_scope = it.vm().stack().frame().scope().clone();
        let callable = Callable::new(self.param_names.clone(), declared_scope, self.body.clone());
        let fn_obj_ref = it.vm_mut().heap_mut().allocate_callable_object(callable)?;
        let variable = Variable::new(
            VariableKind::Var,
            self.fn_name.clone(),
            Value::Reference(fn_obj_ref),
        );
        it.vm_mut()
            .stack_mut()
            .frame_mut()
            .scope_mut()
            .declare_variable(variable)?;
        Ok(())
    }
}

impl Eval for VariableDeclaration {
    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let kind = VariableKind::from(self.kind);
        for entry in &self.entries {
            let variable = if let Some(ref initialiser) = entry.initialiser {
                let initial_value = initialiser.eval(it)?;
                Variable::new(kind, entry.var_name.clone(), initial_value)
            } else {
                Variable::new_unassigned(kind, entry.var_name.clone())
            };
            let curr_scope = it.vm_mut().stack_mut().frame_mut().scope_mut();
            let mut declared_scope = if self.is_escalated() {
                curr_scope.ancestor(true)
            } else {
                curr_scope.clone()
            };
            declared_scope.declare_variable(variable)?;
        }
        Ok(())
    }
}

impl Eval for Expression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        match self {
            Self::Assignment(ref node) => node.eval(it),
            Self::Binary(ref node) => node.eval(it),
            Self::Unary(ref node) => node.eval(it),
            Self::Ternary(ref node) => node.eval(it),
            Self::Grouping(ref node) => node.eval(it),
            Self::FunctionCall(ref node) => node.eval(it),
            Self::PropertyAccess(ref node) => node.eval(it),
            Self::ComputedPropertyAccess(ref node) => node.eval(it),

            Self::Literal(ref node) => node.eval(it),
            Self::VariableAccess(ref node) => node.eval(it),
        }
    }
}

impl Eval for AssignmentExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        fn compute_new_value(
            self_: &AssignmentExpression,
            it: &mut Interpreter,
            getter: impl FnOnce() -> Result,
        ) -> Result {
            let rhs = self_.rhs.eval(it)?;
            Ok(match self_.op {
                AssignmentOperator::Assign => rhs,
                AssignmentOperator::AddAssign => Value::add_or_append(it, &getter()?, &rhs)?,
                AssignmentOperator::SubAssign => Value::sub(&getter()?, &rhs)?,
                AssignmentOperator::MulAssign => Value::mul(&getter()?, &rhs)?,
                AssignmentOperator::DivAssign => Value::div(&getter()?, &rhs)?,
                AssignmentOperator::ModAssign => Value::rem(&getter()?, &rhs)?,
                AssignmentOperator::PowAssign => Value::pow(&getter()?, &rhs)?,
                kind => todo!("AssignmentExpression::eval: kind={:?}", kind),
            })
        }

        assert_matches!(self.op.associativity(), Associativity::RightToLeft);
        match self.lhs.as_ref() {
            Expression::VariableAccess(node) => {
                let mut variable = it
                    .vm()
                    .stack()
                    .frame()
                    .scope()
                    .lookup_variable(&node.var_name)?;
                let new_value = compute_new_value(self, it, || Ok(variable.value().clone()))?;
                variable.set_value(new_value.clone())?;
                Ok(new_value)
            }
            Expression::PropertyAccess(node) => {
                let base_value = node.base.eval(it)?;
                let mut base_obj = match base_value {
                    Value::Reference(ref base_refr) => {
                        it.vm_mut().heap_mut().resolve_mut(base_refr)
                    }
                    base_value => todo!("AssignmentExpression::eval: base_value={:?}", base_value),
                };
                let new_value = compute_new_value(self, it, || {
                    Ok(base_obj
                        .property(&node.property_name)
                        .cloned()
                        .unwrap_or_default())
                })?;
                base_obj.set_property(node.property_name.clone(), new_value.clone());
                Ok(new_value)
            }
            expr => todo!("AssignmentExpression::eval: lhs={:#?}", expr),
        }
    }
}

impl Eval for BinaryExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        Ok(match self.op {
            // Get the boolean ops out of the way first, since they don't let us eval the RHS
            // up-front (which is more ergonomic for all the other ops).
            BinaryOperator::LogicalAnd => {
                assert_matches!(self.op.associativity(), Associativity::LeftToRight);
                match self.lhs.eval(it)? {
                    lhs if lhs.is_truthy() => self.rhs.eval(it)?,
                    lhs => lhs,
                }
            }
            BinaryOperator::LogicalOr => {
                assert_matches!(self.op.associativity(), Associativity::LeftToRight);
                match self.lhs.eval(it)? {
                    lhs if lhs.is_falsy() => self.rhs.eval(it)?,
                    lhs => lhs,
                }
            }

            kind => {
                let (ref lhs, ref rhs) = match kind.associativity() {
                    Associativity::LeftToRight => {
                        let lhs = self.lhs.eval(it)?;
                        let rhs = self.rhs.eval(it)?;
                        (lhs, rhs)
                    }
                    Associativity::RightToLeft => {
                        let rhs = self.rhs.eval(it)?;
                        let lhs = self.lhs.eval(it)?;
                        (lhs, rhs)
                    }
                };
                match kind {
                    // Safety: Unreachable because the possible values are already handled by
                    // previous match arms in the outer match expression.
                    BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr => unsafe {
                        unreachable_unchecked()
                    },

                    BinaryOperator::Add => Value::add_or_append(it, lhs, rhs)?,
                    BinaryOperator::Div => Value::div(lhs, rhs)?,
                    BinaryOperator::Mod => Value::rem(lhs, rhs)?,
                    BinaryOperator::Mul => Value::mul(lhs, rhs)?,
                    BinaryOperator::Pow => Value::pow(lhs, rhs)?,
                    BinaryOperator::Sub => Value::sub(lhs, rhs)?,

                    BinaryOperator::Equal => Value::eq(it, lhs, rhs),
                    BinaryOperator::NotEqual => Value::ne(it, lhs, rhs),
                    BinaryOperator::Identical => Value::identical(it, lhs, rhs),
                    BinaryOperator::NotIdentical => Value::not_identical(it, lhs, rhs),

                    BinaryOperator::LessThan => Value::lt(it, lhs, rhs),
                    BinaryOperator::LessThanOrEqual => Value::le(it, lhs, rhs),
                    BinaryOperator::MoreThan => Value::gt(it, lhs, rhs),
                    BinaryOperator::MoreThanOrEqual => Value::ge(it, lhs, rhs),

                    BinaryOperator::ShiftLeft => Value::shl(lhs, rhs)?,
                    BinaryOperator::ShiftRight => Value::shr_signed(lhs, rhs)?,
                    BinaryOperator::ShiftRightUnsigned => Value::shr_unsigned(lhs, rhs)?,

                    BinaryOperator::BitwiseAnd => Value::bitand(lhs, rhs),
                    BinaryOperator::BitwiseOr => Value::bitor(lhs, rhs),
                    BinaryOperator::BitwiseXOr => Value::bitxor(lhs, rhs),
                }
            }
        })
    }
}

impl Eval for UnaryExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let operand = &self.operand.eval(it)?;
        Ok(match self.op {
            UnaryOperator::IncrementPrefix
            | UnaryOperator::IncrementPostfix
            | UnaryOperator::DecrementPrefix
            | UnaryOperator::DecrementPostfix => {
                fn compute(
                    self_: &UnaryExpression,
                    it: &mut Interpreter,
                    getter: impl FnOnce() -> Result,
                ) -> Result<(Value, Value)> {
                    const ONE: Value = Value::Number(Number::Int(1));
                    let old_value = getter()?;

                    // The new value to assign to the variable or property
                    let new_value = match self_.op {
                        UnaryOperator::IncrementPrefix | UnaryOperator::IncrementPostfix => {
                            Value::add_or_append(it, &old_value, &ONE)?
                        }
                        UnaryOperator::DecrementPrefix | UnaryOperator::DecrementPostfix => {
                            Value::sub(&old_value, &ONE)?
                        }
                        _ => unreachable!("{:?}", self_.op),
                    };
                    // The value to use as the result of the expression
                    let result_value = match self_.op {
                        UnaryOperator::IncrementPrefix | UnaryOperator::DecrementPrefix => {
                            new_value.clone()
                        }
                        UnaryOperator::IncrementPostfix | UnaryOperator::DecrementPostfix => {
                            old_value
                        }
                        _ => unreachable!("{:?}", self_.op),
                    };
                    Ok((new_value, result_value))
                }

                assert_matches!(self.op.associativity(), Associativity::RightToLeft);
                match self.operand.as_ref() {
                    Expression::VariableAccess(node) => {
                        let mut variable = it
                            .vm()
                            .stack()
                            .frame()
                            .scope()
                            .lookup_variable(&node.var_name)?;
                        let (new_value, result_value) =
                            compute(self, it, || Ok(variable.value().clone()))?;
                        variable.set_value(new_value)?;
                        result_value
                    }
                    Expression::PropertyAccess(node) => {
                        let base_value = node.base.eval(it)?;
                        let mut base_obj = match base_value {
                            Value::Reference(ref base_refr) => {
                                it.vm_mut().heap_mut().resolve_mut(base_refr)
                            }
                            base_value => {
                                todo!("AssignmentExpression::eval: base_value={:?}", base_value)
                            }
                        };
                        let (new_value, result_value) = compute(self, it, || {
                            Ok(base_obj
                                .property(&node.property_name)
                                .cloned()
                                .unwrap_or_default())
                        })?;
                        base_obj.set_property(node.property_name.clone(), new_value);
                        result_value
                    }
                    _ => todo!("UnaryExpression::eval: self={:#?}", self),
                }
            }

            UnaryOperator::BitwiseNot => Value::bitnot(operand),
            UnaryOperator::LogicalNot => Value::not(operand),
            UnaryOperator::NumericNegate => Value::neg(operand)?,
            UnaryOperator::NumericPlus => Value::plus(operand),
        })
    }
}

impl Eval for TernaryExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let condition = self.condition.eval(it)?;
        if condition.is_truthy() {
            self.lhs.eval(it)
        } else {
            self.rhs.eval(it)
        }
    }
}

impl Eval for GroupingExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        self.inner.eval(it)
    }
}

impl Eval for LiteralExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        Ok(match self.value {
            Literal::Boolean(ref value) => Value::Boolean(*value),
            Literal::Numeric(NumericLiteral::Int(ref value)) => {
                Value::Number(Number::try_from(*value).unwrap())
            }
            Literal::String(ref value) => Value::String(value.clone()),
            Literal::Array(ref elem_exprs) => {
                let mut elems = Vec::with_capacity(elem_exprs.len());
                for elem_expr in elem_exprs {
                    elems.push(elem_expr.eval(it)?);
                }
                let obj_ref = it.vm_mut().heap_mut().allocate_array(elems)?;
                Value::Reference(obj_ref)
            }
            Literal::Function {
                ref name,
                ref param_names,
                ref body,
            } => {
                let declared_scope = it.vm().stack().frame().scope().clone();
                let callable = match name {
                    Some(name) => Callable::new_named(
                        name.clone(),
                        param_names.clone(),
                        declared_scope,
                        body.clone(),
                    ),
                    None => Callable::new(param_names.clone(), declared_scope, body.clone()),
                };
                let fn_obj_ref = it.vm_mut().heap_mut().allocate_callable_object(callable)?;
                Value::Reference(fn_obj_ref)
            }
            Literal::Object => {
                let obj_ref = it.vm_mut().heap_mut().allocate_empty_object()?;
                Value::Reference(obj_ref)
            }
            Literal::Null => Value::Null,
        })
    }
}

impl Eval for FunctionCallExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        match self.function.eval(it)? {
            Value::Reference(fn_obj_ref) => {
                let fn_obj = it.vm().heap().resolve(&fn_obj_ref);
                let function = if let Some(callable) = fn_obj.callable() {
                    callable
                } else {
                    return Err(Error::NotCallable(NotCallableError));
                };

                let parameters = function.declared_parameters();
                if parameters.len() != self.arguments.len() {
                    return Err(FunctionArgumentMismatchError.into());
                }
                let mut argument_variables = Vec::with_capacity(parameters.len());
                for (idx, parameter_name) in parameters.iter().enumerate() {
                    let argument_expr = &self.arguments[idx];
                    let argument_value = argument_expr.eval(it)?;
                    argument_variables.push(Variable::new(
                        VariableKind::Let,
                        parameter_name.clone(),
                        argument_value,
                    ));
                }

                let declared_scope = function.declared_scope().clone();
                let fn_scope_ctx = ScopeCtx::new(argument_variables);

                it.vm_mut().stack_mut().push_frame(declared_scope);
                if let Some(fn_name) = function.name() {
                    // Create an outer scope with nothing but the function's name, which points to itself,
                    // so that named function literals may recurse using their name, without making the name
                    // visible outside of the function body. It has its own outer scope so it can still be
                    // shadowed by arguments with the same name.
                    let fn_scope_ctx_outer = ScopeCtx::new(vec![Variable::new(
                        VariableKind::Var,
                        fn_name.clone(),
                        Value::Reference(fn_obj_ref.clone()),
                    )]);
                    it.vm_mut()
                        .stack_mut()
                        .frame_mut()
                        .push_scope(fn_scope_ctx_outer, false);
                }
                it.vm_mut()
                    .stack_mut()
                    .frame_mut()
                    .push_scope(fn_scope_ctx, true);
                function.body().eval(it)?;
                it.vm_mut().stack_mut().frame_mut().pop_scope();
                if function.name().is_some() {
                    it.vm_mut().stack_mut().frame_mut().pop_scope();
                }
                it.vm_mut().stack_mut().pop_frame();

                Ok(match it.vm_mut().reset_execution_state() {
                    ExecutionState::Advance => Value::Undefined,
                    ExecutionState::Return(value) => value,
                    execution_state => panic!("Unexpected execution state: {:?}", execution_state),
                })
            }
            Value::NativeFunction(f) => {
                let mut args = Vec::with_capacity(self.arguments.len());
                for arg in &self.arguments {
                    let arg_value = arg.eval(it)?;
                    args.push(arg_value);
                }
                let result = f.apply(it.vm_mut(), &args);
                Ok(result)
            }
            _ => Err(Error::NotCallable(NotCallableError))
        }
    }
}

impl Eval for PropertyAccessExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let base_value = self.base.eval(it)?;
        let base_obj = match base_value {
            Value::Reference(ref base_refr) => it.vm().heap().resolve(base_refr),
            base_value => todo!("PropertyExpression::eval: base={:?}", base_value),
        };
        Ok(base_obj
            .property(&self.property_name)
            .cloned()
            .unwrap_or_default())
    }
}

impl Eval for ComputedPropertyAccessExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let base_value = self.base.eval(it)?;
        let base_obj = match base_value {
            Value::Reference(ref base_refr) => it.vm().heap().resolve(base_refr),
            base_value => todo!("ComputedPropertyExpression::eval: base={:?}", base_value),
        };
        let property_value = self.property.eval(it)?;
        let property = match property_value {
            Value::Number(Number::Int(n)) => Identifier::from(n),
            property => todo!("ComputedPropertyExpression::eval: property={:?}", property),
        };
        Ok(base_obj.property(&property).cloned().unwrap_or_default())
    }
}

impl Eval for VariableAccessExpression {
    type Output = Value;

    fn eval(&self, it: &mut Interpreter) -> Result<Self::Output> {
        let variable = it
            .vm()
            .stack()
            .frame()
            .scope()
            .lookup_variable(&self.var_name)?;
        let value = variable.value().clone();
        Ok(value)
    }
}
