use super::error::{Error, Result};
use super::Parser;
use crate::ast::*;
use crate::iter::peek_fallible::PeekableNthFallibleIterator;
use crate::lexer;
use crate::parser::Expected;
use crate::token::Keyword::{Function, New, This};
use crate::token::Punctuator::{
    self, Amp, AmpAmp, AmpEq, Bang, BangEq, BangEqEq, Caret, CaretEq, CloseBracket, CloseParen,
    Colon, Comma, Dot, Eq, EqEq, EqEqEq, Gt, GtEq, GtGt, GtGtEq, GtGtGt, GtGtGtEq, Lt, LtEq, LtLt,
    LtLtEq, Minus, MinusEq, MinusMinus, OpenBrace, OpenBracket, OpenParen, Percent, PercentEq,
    Pipe, PipeEq, PipePipe, Plus, PlusEq, PlusPlus, Question, Slash, SlashEq, Star, StarEq,
    StarStar, StarStarEq, Tilde,
};
use crate::token::{Element, SourceLocation};
use fallible_iterator::FallibleIterator;

impl<I: FallibleIterator<Item = Element, Error = lexer::Error>> Parser<I> {
    pub(super) fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_expression_impl(Precedence::MIN)
    }

    fn parse_expression_impl(&mut self, min_precedence: Precedence) -> Result<Expression> {
        let mut expression = self.parse_primary_expression()?;
        loop {
            self.skip_non_tokens()?;
            match self.source.peek()? {
                Some(elem) if elem.punctuator().is_some() => {}
                _ => break,
            }
            match self.parse_secondary_expression(expression, min_precedence)? {
                Ok(secondary_expression) => {
                    expression = secondary_expression;
                }
                Err(original_expression) => {
                    expression = original_expression;
                    break;
                }
            }
        }
        Ok(expression)
    }

    fn parse_primary_expression(&mut self) -> Result<Expression> {
        Ok(match self.source.peek()? {
            Some(elem) if elem.identifier().is_some() => self
                .parse_identifier_reference_expression()
                .map(Expression::IdentifierReference)?,
            Some(elem) if elem.keyword() == Some(This) => {
                self.parse_this_expression().map(Expression::This)?
            }
            Some(elem) if elem.keyword() == Some(New) => {
                self.parse_new_expression().map(Expression::New)?
            }
            Some(elem) if elem.literal().is_some() => {
                self.parse_literal_expression().map(Expression::Literal)?
            }
            Some(elem) if elem.punctuator() == Some(OpenBracket) => self
                .parse_array_literal_expression()
                .map(Expression::Array)?,
            Some(elem) if elem.punctuator() == Some(OpenBrace) => self
                .parse_object_literal_expression()
                .map(Expression::Object)?,
            Some(elem) if elem.keyword() == Some(Function) => self
                .parse_function_literal_expression()
                .map(Box::new)
                .map(Expression::Function)?,
            Some(elem) if elem.punctuator().is_some() => self.parse_primary_prefix_expression()?,
            elem => return Err(Error::unexpected(Expected::AnyExpression, elem.cloned())),
        })
    }

    fn parse_primary_prefix_expression(&mut self) -> Result<Expression> {
        let (elem, punc) = match self.source.next()? {
            Some(elem) if let Some(punc) = elem.punctuator() => (elem, punc),
            elem => return Err(Error::unexpected(Expected::AnyExpression, elem)),
        };
        let loc = elem.source_location().clone();
        self.skip_non_tokens()?;
        Ok(match Operator::try_parse(punc, Position::Prefix) {
            Some(Operator::Unary(op_kind)) => self
                .parse_unary_expression(loc, op_kind)
                .map(Expression::Unary)?,
            Some(Operator::Update(op_kind)) => self
                .parse_update_expression(loc, op_kind)
                .map(Expression::Update)?,
            Some(Operator::Grouping) => self
                .parse_grouping_expression(loc)
                .map(Expression::Grouping)?,

            Some(op_kind) => unreachable!("{:?}", op_kind),
            None => {
                return Err(Error::unexpected_token(Expected::AnyExpression, elem));
            }
        })
    }

    fn parse_secondary_expression(
        &mut self,
        lhs: Expression,
        min_precedence: Precedence,
    ) -> Result<std::result::Result<Expression, Expression>> {
        let punc = match self.source.peek()? {
            Some(elem) if let Some(punc) = elem.punctuator() => punc,
            elem => return Err(Error::unexpected(Expected::AnyExpression, elem.cloned())),
        };
        let op_kind = match Operator::try_parse(punc, Position::PostfixOrInfix) {
            Some(op) => op,
            None => return Ok(Err(lhs)),
        };
        if op_kind.precedence() <= min_precedence {
            return Ok(Err(lhs));
        }
        let loc = lhs.source_location().clone();
        self.source.next()?.unwrap();
        self.skip_non_tokens()?;
        let secondary_expression = match op_kind {
            Operator::Assignment(kind) => self
                .parse_assignment_expression(loc, kind, lhs)
                .map(Expression::Assignment)?,
            Operator::Binary(kind) => self
                .parse_binary_expression(loc, kind, lhs)
                .map(Expression::Binary)?,
            Operator::Relational(kind) => self
                .parse_relational_expression(loc, kind, lhs)
                .map(Expression::Relational)?,
            Operator::Unary(kind) => Expression::Unary(UnaryExpression {
                loc,
                op: kind,
                operand: Box::new(lhs),
            }),
            Operator::Update(kind) => Expression::Update(UpdateExpression {
                loc,
                op: kind,
                operand: Box::new(lhs),
            }),
            Operator::Member(kind) => self
                .parse_member_expression(loc, kind, lhs)
                .map(Expression::Member)?,
            Operator::Grouping => {
                self.expect_punctuator(CloseParen)?;
                Expression::Grouping(GroupingExpression {
                    loc,
                    inner: Box::new(lhs),
                })
            }
            Operator::Ternary => self
                .parse_ternary_expression(loc, lhs)
                .map(Expression::Ternary)?,
        };
        Ok(Ok(secondary_expression))
    }

    fn parse_identifier_reference_expression(&mut self) -> Result<IdentifierReferenceExpression> {
        let (identifier, loc) = self.expect_identifier("identifier_reference")?;
        Ok(IdentifierReferenceExpression { loc, identifier })
    }

    fn parse_this_expression(&mut self) -> Result<ThisExpression> {
        let loc = self.expect_keyword(This)?;
        Ok(ThisExpression { loc })
    }

    fn parse_new_expression(&mut self) -> Result<NewExpression> {
        let loc = self.expect_keyword(New)?;
        self.skip_non_tokens()?;
        let (type_name, _) = self.expect_identifier("type_name")?;
        self.skip_non_tokens()?;
        let arguments = if self
            .source
            .next_if(|elem| elem.punctuator() == Some(OpenParen))?
            .is_some()
        {
            let args = self.parse_fn_arguments()?;
            self.skip_non_tokens()?;
            self.expect_punctuator(CloseParen)?;
            args
        } else {
            vec![]
        };
        Ok(NewExpression {
            loc,
            type_name,
            arguments,
        })
    }

    fn parse_assignment_expression(
        &mut self,
        loc: SourceLocation,
        op: AssignmentOperator,
        lhs: Expression,
    ) -> Result<AssignmentExpression> {
        let rhs = self.parse_expression_impl(op.precedence())?;
        Ok(AssignmentExpression {
            loc,
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }

    fn parse_binary_expression(
        &mut self,
        loc: SourceLocation,
        op: BinaryOperator,
        lhs: Expression,
    ) -> Result<BinaryExpression> {
        let rhs = self.parse_expression_impl(op.precedence())?;
        Ok(BinaryExpression {
            loc,
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }

    fn parse_relational_expression(
        &mut self,
        loc: SourceLocation,
        op: RelationalOperator,
        lhs: Expression,
    ) -> Result<RelationalExpression> {
        let rhs = self.parse_expression_impl(op.precedence())?;
        Ok(RelationalExpression {
            loc,
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }

    fn parse_unary_expression(
        &mut self,
        loc: SourceLocation,
        op: UnaryOperator,
    ) -> Result<UnaryExpression> {
        let operand = self.parse_expression_impl(op.precedence())?;
        Ok(UnaryExpression {
            loc,
            op,
            operand: Box::new(operand),
        })
    }

    fn parse_update_expression(
        &mut self,
        loc: SourceLocation,
        op: UpdateOperator,
    ) -> Result<UpdateExpression> {
        let operand = self.parse_expression_impl(op.precedence())?;
        Ok(UpdateExpression {
            loc,
            op,
            operand: Box::new(operand),
        })
    }

    fn parse_member_expression(
        &mut self,
        loc: SourceLocation,
        op: MemberOperator,
        lhs: Expression,
    ) -> Result<MemberExpression> {
        match op {
            MemberOperator::MemberAccess => self
                .parse_member_access_expression(loc, lhs)
                .map(MemberExpression::MemberAccess),
            MemberOperator::ComputedMemberAccess => self
                .parse_computed_member_access_expression(loc, lhs)
                .map(MemberExpression::ComputedMemberAccess),
            MemberOperator::FunctionCall => self
                .parse_function_call_expression(loc, lhs)
                .map(MemberExpression::FunctionCall),
        }
    }

    fn parse_member_access_expression(
        &mut self,
        loc: SourceLocation,
        lhs: Expression,
    ) -> Result<MemberAccessExpression> {
        let rhs = match self.parse_expression_impl(MemberOperator::MemberAccess.precedence())? {
            Expression::IdentifierReference(member_expr) => member_expr.identifier,
            member_expr => todo!(
                "Unsupported member access expression (only simple `a.b` expressions are \
                 supported): {member_expr:?}"
            ),
        };
        Ok(MemberAccessExpression {
            loc,
            base: Box::new(lhs),
            member: rhs,
        })
    }

    fn parse_computed_member_access_expression(
        &mut self,
        loc: SourceLocation,
        lhs: Expression,
    ) -> Result<ComputedMemberAccessExpression> {
        let rhs = self.parse_expression()?;
        self.skip_non_tokens()?;
        self.expect_punctuator(CloseBracket)?;
        Ok(ComputedMemberAccessExpression {
            loc,
            base: Box::new(lhs),
            member: Box::new(rhs),
        })
    }

    fn parse_function_call_expression(
        &mut self,
        loc: SourceLocation,
        lhs: Expression,
    ) -> Result<FunctionCallExpression> {
        let arguments = self.parse_fn_arguments()?;
        self.skip_non_tokens()?;
        self.expect_punctuator(CloseParen)?;
        Ok(FunctionCallExpression {
            loc,
            function: Box::new(lhs),
            arguments,
        })
    }

    fn parse_fn_arguments(&mut self) -> Result<Vec<Expression>> {
        if let Some(elem) = self.source.peek()? && elem.punctuator() == Some(CloseParen) {
            return Ok(vec![]);
        }

        let mut args = Vec::new();
        loop {
            self.skip_non_tokens()?;
            args.push(self.parse_expression()?);
            match self.source.peek()? {
                Some(elem) if elem.punctuator() == Some(Comma) => {
                    self.source.next()?.unwrap();
                }
                Some(elem) if elem.punctuator() == Some(CloseParen) => {
                    break Ok(args);
                }
                elem => return Err(Error::unexpected((Comma, CloseParen), elem.cloned())),
            }
        }
    }

    fn parse_grouping_expression(&mut self, loc: SourceLocation) -> Result<GroupingExpression> {
        let inner = self.parse_expression()?;
        self.skip_non_tokens()?;
        self.expect_punctuator(CloseParen)?;
        Ok(GroupingExpression {
            loc,
            inner: Box::new(inner),
        })
    }

    fn parse_ternary_expression(
        &mut self,
        loc: SourceLocation,
        lhs: Expression,
    ) -> Result<TernaryExpression> {
        let condition = lhs;
        self.skip_non_tokens()?;
        let lhs = self.parse_expression_impl(Operator::Ternary.precedence())?;
        self.skip_non_tokens()?;
        self.expect_punctuator(Colon)?;
        self.skip_non_tokens()?;
        let rhs = self.parse_expression_impl(Operator::Ternary.precedence())?;
        Ok(TernaryExpression {
            loc,
            condition: Box::new(condition),
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(super) enum Position {
    /// For example:
    /// - `++a`
    Prefix,
    /// For example:
    /// - `a++`
    /// - `a + b`
    PostfixOrInfix,
}

trait TryParse {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self>
    where
        Self: Sized;
}

impl TryParse for Operator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        AssignmentOperator::try_parse(punc, pos)
            .map(Self::Assignment)
            .or_else(|| BinaryOperator::try_parse(punc, pos).map(Self::Binary))
            .or_else(|| RelationalOperator::try_parse(punc, pos).map(Self::Relational))
            .or_else(|| UnaryOperator::try_parse(punc, pos).map(Self::Unary))
            .or_else(|| UpdateOperator::try_parse(punc, pos).map(Self::Update))
            .or_else(|| MemberOperator::try_parse(punc, pos).map(Self::Member))
            .or_else(|| GroupingOperator::try_parse(punc, pos).map(|_| Self::Grouping))
            .or_else(|| TernaryOperator::try_parse(punc, pos).map(|_| Self::Ternary))
    }
}

impl TryParse for AssignmentOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        if pos != Position::PostfixOrInfix {
            return None;
        }
        Some(match punc {
            Eq => Self::Assign,
            PlusEq => Self::ComputeAssign(BinaryOperator::Addition),
            MinusEq => Self::ComputeAssign(BinaryOperator::Subtraction),
            StarEq => Self::ComputeAssign(BinaryOperator::Multiplication),
            SlashEq => Self::ComputeAssign(BinaryOperator::Division),
            PercentEq => Self::ComputeAssign(BinaryOperator::Modulus),
            StarStarEq => Self::ComputeAssign(BinaryOperator::Exponentiation),
            AmpEq => Self::ComputeAssign(BinaryOperator::BitwiseAnd),
            PipeEq => Self::ComputeAssign(BinaryOperator::BitwiseOr),
            CaretEq => Self::ComputeAssign(BinaryOperator::BitwiseXOr),
            LtLtEq => Self::ComputeAssign(BinaryOperator::BitwiseLeftShift),
            GtGtEq => Self::ComputeAssign(BinaryOperator::BitwiseRightShift),
            GtGtGtEq => Self::ComputeAssign(BinaryOperator::BitwiseRightShiftUnsigned),
            _ => return None,
        })
    }
}

impl TryParse for BinaryOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        if pos != Position::PostfixOrInfix {
            return None;
        }
        Some(match punc {
            Plus => Self::Addition,
            Minus => Self::Subtraction,
            Star => Self::Multiplication,
            Slash => Self::Division,
            Percent => Self::Modulus,
            StarStar => Self::Exponentiation,
            Amp => Self::BitwiseAnd,
            Pipe => Self::BitwiseOr,
            Caret => Self::BitwiseXOr,
            AmpAmp => Self::LogicalAnd,
            PipePipe => Self::LogicalOr,
            LtLt => Self::BitwiseLeftShift,
            GtGt => Self::BitwiseRightShift,
            GtGtGt => Self::BitwiseRightShiftUnsigned,
            _ => return None,
        })
    }
}

impl TryParse for RelationalOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        if pos != Position::PostfixOrInfix {
            return None;
        }
        Some(match punc {
            EqEq => Self::Equality,
            BangEq => Self::Inequality,
            EqEqEq => Self::StrictEquality,
            BangEqEq => Self::StrictInequality,
            Gt => Self::GreaterThan,
            GtEq => Self::GreaterThanOrEqual,
            Lt => Self::LessThan,
            LtEq => Self::LessThanOrEqual,
            _ => return None,
        })
    }
}

impl TryParse for UnaryOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        Some(match (punc, pos) {
            (Plus, Position::Prefix) => Self::NumericPlus,
            (Minus, Position::Prefix) => Self::NumericNegation,
            (Tilde, Position::Prefix) => Self::BitwiseNot,
            (Bang, Position::Prefix) => Self::LogicalNot,
            (_, _) => return None,
        })
    }
}

impl TryParse for UpdateOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        Some(match (punc, pos) {
            (PlusPlus, Position::PostfixOrInfix) => Self::GetAndIncrement,
            (PlusPlus, Position::Prefix) => Self::IncrementAndGet,
            (MinusMinus, Position::PostfixOrInfix) => Self::GetAndDecrement,
            (MinusMinus, Position::Prefix) => Self::DecrementAndGet,
            (_, _) => return None,
        })
    }
}

impl TryParse for MemberOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        Some(match (punc, pos) {
            (Dot, Position::PostfixOrInfix) => Self::MemberAccess,
            (OpenBracket, Position::PostfixOrInfix) => Self::ComputedMemberAccess,
            (OpenParen, Position::PostfixOrInfix) => Self::FunctionCall,
            (_, _) => return None,
        })
    }
}

impl TryParse for GroupingOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        matches!((punc, pos), (OpenParen, Position::Prefix)).then_some(Self)
    }
}

impl TryParse for TernaryOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        matches!((punc, pos), (Question, Position::PostfixOrInfix)).then_some(Self)
    }
}
