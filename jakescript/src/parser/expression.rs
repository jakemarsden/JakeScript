use super::error::{Error, Expected, Result};
use super::op::{ParseOperator, Position};
use super::Parser;
use crate::ast::*;
use crate::iter::peek_fallible::PeekableNthFallibleIterator;
use crate::lexer;
use crate::token::Keyword::{Function, New, This};
use crate::token::Punctuator::{
    CloseBracket, CloseParen, Colon, Comma, OpenBrace, OpenBracket, OpenParen,
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
            if self.source.peek()?.map(Element::punctuator).is_none() {
                break;
            }
            match self.parse_secondary_expression(expression, min_precedence)? {
                ParseSecondaryExpressionOutcome::Secondary(secondary) => {
                    expression = secondary;
                }
                ParseSecondaryExpressionOutcome::NotSecondary(original) => {
                    expression = original;
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
            Some(elem) if elem.keyword() == Some(New) => {
                self.parse_new_expression().map(Expression::New)?
            }
            Some(elem) if elem.keyword() == Some(This) => {
                self.parse_this_expression().map(Expression::This)?
            }

            Some(elem) if elem.punctuator() == Some(OpenBracket) => {
                self.parse_array_expression().map(Expression::Array)?
            }
            Some(elem) if elem.keyword() == Some(Function) => self
                .parse_function_expression()
                .map(Box::new)
                .map(Expression::Function)?,
            Some(elem) if elem.literal().is_some() => {
                self.parse_literal_expression().map(Expression::Literal)?
            }
            Some(elem) if elem.punctuator() == Some(OpenBrace) => {
                self.parse_object_expression().map(Expression::Object)?
            }

            Some(elem) => {
                let elem = elem.clone();
                if let Some(prefix_expression) = self.parse_primary_prefix_expression()? {
                    prefix_expression
                } else {
                    return Err(Error::unexpected(Expected::AnyExpression, elem));
                }
            }
            elem => return Err(Error::unexpected(Expected::AnyExpression, elem.cloned())),
        })
    }

    fn parse_primary_prefix_expression(&mut self) -> Result<Option<Expression>> {
        let Some(punc) = self.source.peek()?.and_then(Element::punctuator) else {
            return Ok(None);
        };
        let (loc, op_kind) = match Operator::try_parse(punc, Position::Prefix) {
            Some(op_kind) => {
                let loc = self.expect_punctuator(punc).unwrap();
                (loc, op_kind)
            }
            None => return Ok(None),
        };
        self.skip_non_tokens()?;

        Ok(Some(match op_kind {
            Operator::Grouping => self
                .parse_primary_grouping_expression(loc)
                .map(Expression::Grouping)?,
            Operator::Unary(op_kind) => self
                .parse_primary_unary_expression(loc, op_kind)
                .map(Expression::Unary)?,
            Operator::Update(op_kind) => self
                .parse_primary_update_expression(loc, op_kind)
                .map(Expression::Update)?,

            op_kind => unreachable!("expected primary prefix expression but was {op_kind:?}"),
        }))
    }

    fn parse_secondary_expression(
        &mut self,
        lhs: Expression,
        min_precedence: Precedence,
    ) -> Result<ParseSecondaryExpressionOutcome> {
        let punc = match self.source.peek()? {
            Some(elem) if let Some(punc) = elem.punctuator() => punc,
            elem => return Err(Error::unexpected(Expected::AnyExpression, elem.cloned())),
        };

        let op_kind = match Operator::try_parse(punc, Position::PostfixOrInfix) {
            Some(op_kind) if op_kind.precedence() > min_precedence => op_kind,
            Some(_) | None => return Ok(ParseSecondaryExpressionOutcome::NotSecondary(lhs)),
        };

        let loc = self.expect_punctuator(punc).unwrap();
        self.skip_non_tokens()?;

        Ok(ParseSecondaryExpressionOutcome::Secondary(match op_kind {
            Operator::Member(kind) => self
                .parse_member_expression(loc, kind, lhs)
                .map(Expression::Member)?,

            Operator::Assignment(kind) => self
                .parse_assignment_expression(loc, kind, lhs)
                .map(Expression::Assignment)?,
            Operator::Binary(kind) => self
                .parse_binary_expression(loc, kind, lhs)
                .map(Expression::Binary)?,
            Operator::Grouping => self
                .parse_grouping_expression(loc, lhs)
                .map(Expression::Grouping)?,
            Operator::Relational(kind) => self
                .parse_relational_expression(loc, kind, lhs)
                .map(Expression::Relational)?,
            Operator::Ternary => self
                .parse_ternary_expression(loc, lhs)
                .map(Expression::Ternary)?,
            Operator::Unary(kind) => {
                Expression::Unary(Self::parse_unary_expression(loc, kind, lhs))
            }
            Operator::Update(kind) => {
                Expression::Update(Self::parse_update_expression(loc, kind, lhs))
            }
        }))
    }

    fn parse_identifier_reference_expression(&mut self) -> Result<IdentifierReferenceExpression> {
        let (identifier, loc) = self.expect_identifier("identifier_reference")?;
        Ok(IdentifierReferenceExpression { loc, identifier })
    }

    fn parse_member_expression(
        &mut self,
        loc: SourceLocation,
        op: MemberOperator,
        lhs: Expression,
    ) -> Result<MemberExpression> {
        match op {
            MemberOperator::ComputedMemberAccess => self
                .parse_computed_member_access_expression(loc, lhs)
                .map(MemberExpression::ComputedMemberAccess),
            MemberOperator::FunctionCall => self
                .parse_function_call_expression(loc, lhs)
                .map(MemberExpression::FunctionCall),
            MemberOperator::MemberAccess => self
                .parse_member_access_expression(loc, lhs)
                .map(MemberExpression::MemberAccess),
        }
    }

    fn parse_computed_member_access_expression(
        &mut self,
        loc: SourceLocation,
        base: Expression,
    ) -> Result<ComputedMemberAccessExpression> {
        let index = self.parse_expression()?;
        self.skip_non_tokens()?;
        self.expect_punctuator(CloseBracket)?;
        Ok(ComputedMemberAccessExpression {
            loc,
            base: Box::new(base),
            index: Box::new(index),
        })
    }

    fn parse_function_call_expression(
        &mut self,
        loc: SourceLocation,
        function: Expression,
    ) -> Result<FunctionCallExpression> {
        let arguments = self.parse_fn_arguments()?;
        self.skip_non_tokens()?;
        self.expect_punctuator(CloseParen)?;
        Ok(FunctionCallExpression {
            loc,
            function: Box::new(function),
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

    fn parse_member_access_expression(
        &mut self,
        loc: SourceLocation,
        base: Expression,
    ) -> Result<MemberAccessExpression> {
        let member = match self.parse_expression_impl(MemberOperator::MemberAccess.precedence())? {
            Expression::IdentifierReference(member_expr) => member_expr.identifier,
            member_expr => todo!(
                "Unsupported member access expression (only simple `a.b` expressions are \
                 supported): {member_expr:#?}"
            ),
        };
        Ok(MemberAccessExpression {
            loc,
            base: Box::new(base),
            member,
        })
    }

    fn parse_new_expression(&mut self) -> Result<NewExpression> {
        let loc = self.expect_keyword(New)?;
        self.skip_non_tokens()?;
        let (constructor, _) = self.expect_identifier("type_name")?;
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
            constructor,
            arguments,
        })
    }

    fn parse_this_expression(&mut self) -> Result<ThisExpression> {
        let loc = self.expect_keyword(This)?;
        Ok(ThisExpression { loc })
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

    fn parse_primary_grouping_expression(
        &mut self,
        loc: SourceLocation,
    ) -> Result<GroupingExpression> {
        let inner = self.parse_expression()?;
        self.skip_non_tokens()?;
        self.parse_grouping_expression(loc, inner)
    }

    fn parse_grouping_expression(
        &mut self,
        loc: SourceLocation,
        inner: Expression,
    ) -> Result<GroupingExpression> {
        self.expect_punctuator(CloseParen)?;
        Ok(GroupingExpression {
            loc,
            inner: Box::new(inner),
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

    fn parse_ternary_expression(
        &mut self,
        loc: SourceLocation,
        condition: Expression,
    ) -> Result<TernaryExpression> {
        let true_value = self.parse_expression_impl(Operator::Ternary.precedence())?;
        self.skip_non_tokens()?;
        self.expect_punctuator(Colon)?;
        self.skip_non_tokens()?;
        let false_value = self.parse_expression_impl(Operator::Ternary.precedence())?;
        Ok(TernaryExpression {
            loc,
            condition: Box::new(condition),
            true_value: Box::new(true_value),
            false_value: Box::new(false_value),
        })
    }

    fn parse_primary_unary_expression(
        &mut self,
        loc: SourceLocation,
        op: UnaryOperator,
    ) -> Result<UnaryExpression> {
        let operand = self.parse_expression_impl(op.precedence())?;
        self.skip_non_tokens()?;
        Ok(Self::parse_unary_expression(loc, op, operand))
    }

    fn parse_unary_expression(
        loc: SourceLocation,
        op: UnaryOperator,
        operand: Expression,
    ) -> UnaryExpression {
        UnaryExpression {
            loc,
            op,
            operand: Box::new(operand),
        }
    }

    fn parse_primary_update_expression(
        &mut self,
        loc: SourceLocation,
        op: UpdateOperator,
    ) -> Result<UpdateExpression> {
        let operand = self.parse_expression_impl(op.precedence())?;
        self.skip_non_tokens()?;
        Ok(Self::parse_update_expression(loc, op, operand))
    }

    fn parse_update_expression(
        loc: SourceLocation,
        op: UpdateOperator,
        operand: Expression,
    ) -> UpdateExpression {
        UpdateExpression {
            loc,
            op,
            operand: Box::new(operand),
        }
    }
}

enum ParseSecondaryExpressionOutcome {
    Secondary(Expression),
    NotSecondary(Expression),
}
