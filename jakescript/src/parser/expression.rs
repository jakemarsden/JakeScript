use super::error::AllowToken::{AnyOf, Unspecified};
use super::error::{Error, Result};
use super::Parser;
use crate::ast::*;
use crate::iter::peek_fallible::PeekableNthFallibleIterator;
use crate::lexer;
use crate::non_empty_str;
use crate::token::Keyword::Function;
use crate::token::Punctuator::{
    self, Amp, AmpAmp, AmpEq, Bang, BangEq, BangEqEq, Caret, CaretEq, CloseBracket, CloseParen,
    Colon, Comma, Dot, Eq, EqEq, EqEqEq, Gt, GtEq, GtGt, GtGtEq, GtGtGt, GtGtGtEq, Lt, LtEq, LtLt,
    LtLtEq, Minus, MinusEq, MinusMinus, OpenBrace, OpenBracket, OpenParen, Percent, PercentEq,
    Pipe, PipeEq, PipePipe, Plus, PlusEq, PlusPlus, Question, Slash, SlashEq, Star, StarEq,
    StarStar, StarStarEq, Tilde,
};
use crate::token::{Element, Token};
use fallible_iterator::FallibleIterator;

trait TryParse {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self>
    where
        Self: Sized;
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

impl<I: FallibleIterator<Item = Element, Error = lexer::Error>> Parser<I> {
    pub(super) fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_expression_impl(Precedence::MIN)
    }

    fn parse_expression_impl(&mut self, min_precedence: Precedence) -> Result<Expression> {
        let mut expression = self.parse_primary_expression()?;
        loop {
            self.skip_non_tokens()?;
            if !matches!(
                self.source.peek()?,
                Some(Element::Token(Token::Punctuator(_)))
            ) {
                break;
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
            Some(Element::Token(Token::Identifier(_))) => {
                Expression::IdentifierReference(self.parse_identifier_reference_expression()?)
            }
            Some(Element::Token(Token::Literal(_))) => {
                Expression::Literal(self.parse_literal_expression()?)
            }
            Some(Element::Token(Token::Punctuator(OpenBracket))) => {
                Expression::Array(self.parse_array_literal_expression()?)
            }
            Some(Element::Token(Token::Punctuator(OpenBrace))) => {
                Expression::Object(self.parse_object_literal_expression()?)
            }
            Some(Element::Token(Token::Keyword(Function))) => {
                Expression::Function(Box::new(self.parse_function_literal_expression()?))
            }
            Some(Element::Token(Token::Punctuator(_))) => self.parse_primary_prefix_expression()?,
            actual => return Err(Error::unexpected(Unspecified, actual.cloned())),
        })
    }

    fn parse_primary_prefix_expression(&mut self) -> Result<Expression> {
        let punc = match self.source.next()? {
            Some(Element::Token(Token::Punctuator(punc))) => punc,
            actual => return Err(Error::unexpected(Unspecified, actual)),
        };
        Ok(match Operator::try_parse(punc, Position::Prefix) {
            Some(Operator::Unary(op_kind)) => {
                self.skip_non_tokens()?;
                let operand = self.parse_expression_impl(op_kind.precedence())?;
                Expression::Unary(UnaryExpression {
                    op: op_kind,
                    operand: Box::new(operand),
                })
            }
            Some(Operator::Update(op_kind)) => {
                self.skip_non_tokens()?;
                let operand = self.parse_expression_impl(op_kind.precedence())?;
                Expression::Update(UpdateExpression {
                    op: op_kind,
                    operand: Box::new(operand),
                })
            }
            Some(Operator::Grouping) => {
                self.skip_non_tokens()?;
                let inner = self.parse_expression()?;
                self.skip_non_tokens()?;
                self.expect_punctuator(CloseParen)?;
                Expression::Grouping(GroupingExpression {
                    inner: Box::new(inner),
                })
            }
            Some(op_kind) => unreachable!("{:?}", op_kind),
            None => {
                return Err(Error::unexpected_token(
                    Unspecified,
                    Element::Token(Token::Punctuator(punc)),
                ));
            }
        })
    }

    fn parse_secondary_expression(
        &mut self,
        lhs: Expression,
        min_precedence: Precedence,
    ) -> Result<std::result::Result<Expression, Expression>> {
        let punc = match self.source.peek()? {
            Some(Element::Token(Token::Punctuator(punc))) => punc,
            actual => return Err(Error::unexpected(Unspecified, actual.cloned())),
        };
        let op_kind = match Operator::try_parse(*punc, Position::PostfixOrInfix) {
            Some(op) => op,
            None => return Ok(Err(lhs)),
        };
        if op_kind.precedence() <= min_precedence {
            return Ok(Err(lhs));
        }
        self.source.next()?.unwrap();
        let secondary_expression = match op_kind {
            Operator::Assignment(kind) => {
                self.skip_non_tokens()?;
                let rhs = self.parse_expression_impl(op_kind.precedence())?;
                Expression::Assignment(AssignmentExpression {
                    op: kind,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                })
            }
            Operator::Binary(kind) => {
                self.skip_non_tokens()?;
                let rhs = self.parse_expression_impl(op_kind.precedence())?;
                Expression::Binary(BinaryExpression {
                    op: kind,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                })
            }
            Operator::Relational(kind) => {
                self.skip_non_tokens()?;
                let rhs = self.parse_expression_impl(op_kind.precedence())?;
                Expression::Relational(RelationalExpression {
                    op: kind,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                })
            }
            Operator::Unary(kind) => Expression::Unary(UnaryExpression {
                op: kind,
                operand: Box::new(lhs),
            }),
            Operator::Update(kind) => Expression::Update(UpdateExpression {
                op: kind,
                operand: Box::new(lhs),
            }),
            Operator::Member(MemberOperator::FunctionCall) => {
                self.skip_non_tokens()?;
                let arguments = self.parse_fn_arguments()?;
                self.skip_non_tokens()?;
                self.expect_punctuator(CloseParen)?;
                Expression::Member(MemberExpression::FunctionCall(FunctionCallExpression {
                    function: Box::new(lhs),
                    arguments,
                }))
            }
            Operator::Member(MemberOperator::MemberAccess) => {
                self.skip_non_tokens()?;
                let rhs = match self.parse_expression_impl(op_kind.precedence())? {
                    Expression::IdentifierReference(member_expr) => member_expr.identifier,
                    member_expr => todo!(
                        "Unsupported member access expression (only simple `a.b` expressions are \
                         supported): {:?}",
                        member_expr
                    ),
                };
                Expression::Member(MemberExpression::MemberAccess(MemberAccessExpression {
                    base: Box::new(lhs),
                    member: rhs,
                }))
            }
            Operator::Member(MemberOperator::ComputedMemberAccess) => {
                self.skip_non_tokens()?;
                let rhs = self.parse_expression()?;
                self.skip_non_tokens()?;
                self.expect_punctuator(CloseBracket)?;
                Expression::Member(MemberExpression::ComputedMemberAccess(
                    ComputedMemberAccessExpression {
                        base: Box::new(lhs),
                        member: Box::new(rhs),
                    },
                ))
            }
            Operator::Grouping => {
                self.skip_non_tokens()?;
                self.expect_punctuator(CloseParen)?;
                Expression::Grouping(GroupingExpression {
                    inner: Box::new(lhs),
                })
            }
            Operator::Ternary => {
                let condition = lhs;
                self.skip_non_tokens()?;
                let lhs = self.parse_expression_impl(op_kind.precedence())?;
                self.skip_non_tokens()?;
                self.expect_punctuator(Colon)?;
                self.skip_non_tokens()?;
                let rhs = self.parse_expression_impl(op_kind.precedence())?;
                Expression::Ternary(TernaryExpression {
                    condition: Box::new(condition),
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                })
            }
        };
        Ok(Ok(secondary_expression))
    }

    fn parse_identifier_reference_expression(&mut self) -> Result<IdentifierReferenceExpression> {
        let identifier = self.expect_identifier(non_empty_str!("identifier_reference"))?;
        Ok(IdentifierReferenceExpression { identifier })
    }

    fn parse_fn_arguments(&mut self) -> Result<Vec<Expression>> {
        if matches!(
            self.source.peek()?,
            Some(Element::Token(Token::Punctuator(CloseParen)))
        ) {
            return Ok(vec![]);
        }

        let mut args = Vec::new();
        loop {
            self.skip_non_tokens()?;
            args.push(self.parse_expression()?);
            match self.source.peek()? {
                Some(Element::Token(Token::Punctuator(Comma))) => {
                    self.source.next()?.unwrap();
                }
                Some(Element::Token(Token::Punctuator(CloseParen))) => {
                    break Ok(args);
                }
                actual => {
                    return Err(Error::unexpected(
                        AnyOf(
                            Token::Punctuator(Comma),
                            Token::Punctuator(CloseParen),
                            vec![],
                        ),
                        actual.cloned(),
                    ))
                }
            }
        }
    }
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
            (OpenParen, Position::PostfixOrInfix) => Self::FunctionCall,
            (Dot, Position::PostfixOrInfix) => Self::MemberAccess,
            (OpenBracket, Position::PostfixOrInfix) => Self::ComputedMemberAccess,
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
