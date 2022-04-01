use super::block::Braces;
use super::error::AllowToken::{AnyOf, Unspecified};
use super::error::{Error, Result};
use super::Parser;
use crate::ast::*;
use crate::iter::peek_fallible::PeekableNthFallibleIterator;
use crate::lexer;
use crate::non_empty_str;
use crate::token::{self, Keyword, Punctuator, Token};
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

impl<I: FallibleIterator<Item = Token, Error = lexer::Error>> Parser<I> {
    pub(super) fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_expression_impl(Precedence::MIN)
    }

    fn parse_expression_impl(&mut self, min_precedence: Precedence) -> Result<Expression> {
        let mut expression = self.parse_primary_expression()?;
        while let Some(&Token::Punctuator(punctuator)) = self.tokens.peek()? {
            if let Some(op_kind) = Operator::try_parse(punctuator, Position::PostfixOrInfix) {
                if op_kind.precedence() > min_precedence {
                    self.tokens.next().unwrap().unwrap();
                    expression = self.parse_secondary_expression(expression, op_kind)?;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(expression)
    }

    fn parse_primary_expression(&mut self) -> Result<Expression> {
        Ok(match self.tokens.next()? {
            Some(Token::Identifier(identifier)) => {
                Expression::IdentifierReference(IdentifierReferenceExpression {
                    identifier: Identifier::from(identifier),
                })
            }
            Some(Token::Literal(literal)) => Expression::Literal(LiteralExpression {
                value: match literal {
                    token::Literal::Boolean(value) => Literal::Boolean(value),
                    token::Literal::Numeric(
                        token::NumericLiteral::BinInt(value)
                        | token::NumericLiteral::OctInt(value)
                        | token::NumericLiteral::DecInt(value)
                        | token::NumericLiteral::HexInt(value),
                    ) => Literal::Numeric(NumericLiteral::Int(value)),
                    token::Literal::Numeric(token::NumericLiteral::Decimal(value)) => {
                        Literal::Numeric(NumericLiteral::Float(value))
                    }
                    token::Literal::String(value) => {
                        Literal::String(StringLiteral { value: value.value })
                    }
                    token::Literal::RegEx(value) => {
                        // FIXME: Support Literal::RegEx properly.
                        Literal::String(StringLiteral {
                            value: value.to_string(),
                        })
                    }
                    token::Literal::Null => Literal::Null,
                },
            }),
            Some(Token::Punctuator(Punctuator::OpenBracket)) => {
                let declared_elements = self.parse_array_literal_elements()?;
                self.expect_punctuator(Punctuator::CloseBracket)?;
                Expression::Array(ArrayExpression { declared_elements })
            }
            Some(Token::Punctuator(Punctuator::OpenBrace)) => {
                let declared_properties = self.parse_object_properties()?;
                self.expect_punctuator(Punctuator::CloseBrace)?;
                Expression::Object(ObjectExpression {
                    declared_properties,
                })
            }
            Some(Token::Keyword(Keyword::Function)) => {
                let binding = match self.tokens.peek()? {
                    Some(Token::Identifier(_)) => {
                        let name = self
                            .expect_identifier(non_empty_str!("function_name"))
                            .unwrap();
                        Some(Identifier::from(name))
                    }
                    Some(Token::Punctuator(Punctuator::OpenParen)) => None,
                    token => {
                        return Err(Error::unexpected(
                            AnyOf(
                                Token::Punctuator(Punctuator::OpenParen),
                                Token::Identifier(non_empty_str!("function_name")),
                                vec![],
                            ),
                            token.cloned(),
                        ))
                    }
                };
                let formal_parameters = self.parse_fn_parameters()?;
                let body = self.parse_block(Braces::Require)?;
                Expression::Function(Box::new(FunctionExpression {
                    binding,
                    formal_parameters,
                    body,
                }))
            }
            Some(Token::Punctuator(punc)) => match Operator::try_parse(punc, Position::Prefix) {
                Some(Operator::Unary(op_kind)) => {
                    let operand = self.parse_expression_impl(op_kind.precedence())?;
                    Expression::Unary(UnaryExpression {
                        op: op_kind,
                        operand: Box::new(operand),
                    })
                }
                Some(Operator::Update(op_kind)) => {
                    let operand = self.parse_expression_impl(op_kind.precedence())?;
                    Expression::Update(UpdateExpression {
                        op: op_kind,
                        operand: Box::new(operand),
                    })
                }
                Some(Operator::Grouping) => {
                    let inner = self.parse_expression()?;
                    self.expect_punctuator(Punctuator::CloseParen)?;
                    Expression::Grouping(GroupingExpression {
                        inner: Box::new(inner),
                    })
                }
                Some(actual) => unreachable!("{:?}", actual),
                None => {
                    return Err(Error::unexpected_token(
                        Unspecified,
                        Token::Punctuator(punc),
                    ));
                }
            },
            actual => return Err(Error::unexpected(Unspecified, actual)),
        })
    }

    fn parse_secondary_expression(
        &mut self,
        lhs: Expression,
        op_kind: Operator,
    ) -> Result<Expression> {
        Ok(match op_kind {
            Operator::Assignment(kind) => Expression::Assignment(AssignmentExpression {
                op: kind,
                lhs: Box::new(lhs),
                rhs: Box::new(self.parse_expression_impl(op_kind.precedence())?),
            }),
            Operator::Binary(kind) => Expression::Binary(BinaryExpression {
                op: kind,
                lhs: Box::new(lhs),
                rhs: Box::new(self.parse_expression_impl(op_kind.precedence())?),
            }),
            Operator::Relational(kind) => Expression::Relational(RelationalExpression {
                op: kind,
                lhs: Box::new(lhs),
                rhs: Box::new(self.parse_expression_impl(op_kind.precedence())?),
            }),
            Operator::Unary(kind) => Expression::Unary(UnaryExpression {
                op: kind,
                operand: Box::new(lhs),
            }),
            Operator::Update(kind) => Expression::Update(UpdateExpression {
                op: kind,
                operand: Box::new(lhs),
            }),
            Operator::Member(MemberOperator::FunctionCall) => {
                Expression::Member(MemberExpression::FunctionCall(FunctionCallExpression {
                    function: Box::new(lhs),
                    arguments: self.parse_fn_arguments(false)?,
                }))
            }
            Operator::Member(MemberOperator::MemberAccess) => {
                let rhs = self.parse_expression_impl(op_kind.precedence())?;
                Expression::Member(MemberExpression::MemberAccess(MemberAccessExpression {
                    base: Box::new(lhs),
                    member: match rhs {
                        Expression::IdentifierReference(IdentifierReferenceExpression {
                            identifier: var_name,
                        }) => var_name,
                        rhs_expr => todo!(
                            "Unsupported property access expression (only simple `a.b` \
                             expressions are currently supported): {:?}",
                            rhs_expr
                        ),
                    },
                }))
            }
            Operator::Member(MemberOperator::ComputedMemberAccess) => {
                let rhs = self.parse_expression()?;
                self.expect_punctuator(Punctuator::CloseBracket)?;
                Expression::Member(MemberExpression::ComputedMemberAccess(
                    ComputedMemberAccessExpression {
                        base: Box::new(lhs),
                        member: Box::new(rhs),
                    },
                ))
            }
            Operator::Grouping => Expression::Grouping(GroupingExpression {
                inner: Box::new(lhs),
            }),
            Operator::Ternary => {
                let condition = lhs;
                let lhs = self.parse_expression_impl(op_kind.precedence())?;
                self.expect_punctuator(Punctuator::Colon)?;
                let rhs = self.parse_expression_impl(op_kind.precedence())?;
                Expression::Ternary(TernaryExpression {
                    condition: Box::new(condition),
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                })
            }
        })
    }

    fn parse_fn_arguments(&mut self, consume_open_paren: bool) -> Result<Vec<Expression>> {
        if consume_open_paren {
            self.expect_punctuator(Punctuator::OpenParen)?;
        }
        if self
            .tokens
            .next_if_eq(&Token::Punctuator(Punctuator::CloseParen))?
            .is_some()
        {
            return Ok(Vec::with_capacity(0));
        }

        let mut args = Vec::new();
        loop {
            args.push(self.parse_expression()?);
            match self.tokens.next()? {
                Some(Token::Punctuator(Punctuator::Comma)) => {}
                Some(Token::Punctuator(Punctuator::CloseParen)) => break Ok(args),
                actual => {
                    return Err(Error::unexpected(
                        AnyOf(
                            Token::Punctuator(Punctuator::Comma),
                            Token::Punctuator(Punctuator::CloseParen),
                            vec![],
                        ),
                        actual,
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
            Punctuator::Eq => Self::Assign,
            Punctuator::PlusEq => Self::ComputeAssign(BinaryOperator::Addition),
            Punctuator::MinusEq => Self::ComputeAssign(BinaryOperator::Subtraction),
            Punctuator::StarEq => Self::ComputeAssign(BinaryOperator::Multiplication),
            Punctuator::SlashEq => Self::ComputeAssign(BinaryOperator::Division),
            Punctuator::PercentEq => Self::ComputeAssign(BinaryOperator::Modulus),
            Punctuator::StarStarEq => Self::ComputeAssign(BinaryOperator::Exponentiation),
            Punctuator::AmpEq => Self::ComputeAssign(BinaryOperator::BitwiseAnd),
            Punctuator::PipeEq => Self::ComputeAssign(BinaryOperator::BitwiseOr),
            Punctuator::CaretEq => Self::ComputeAssign(BinaryOperator::BitwiseXOr),
            Punctuator::LtLtEq => Self::ComputeAssign(BinaryOperator::BitwiseLeftShift),
            Punctuator::GtGtEq => Self::ComputeAssign(BinaryOperator::BitwiseRightShift),
            Punctuator::GtGtGtEq => Self::ComputeAssign(BinaryOperator::BitwiseRightShiftUnsigned),
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
            Punctuator::Plus => Self::Addition,
            Punctuator::Minus => Self::Subtraction,
            Punctuator::Star => Self::Multiplication,
            Punctuator::Slash => Self::Division,
            Punctuator::Percent => Self::Modulus,
            Punctuator::StarStar => Self::Exponentiation,
            Punctuator::Amp => Self::BitwiseAnd,
            Punctuator::Pipe => Self::BitwiseOr,
            Punctuator::Caret => Self::BitwiseXOr,
            Punctuator::AmpAmp => Self::LogicalAnd,
            Punctuator::PipePipe => Self::LogicalOr,
            Punctuator::LtLt => Self::BitwiseLeftShift,
            Punctuator::GtGt => Self::BitwiseRightShift,
            Punctuator::GtGtGt => Self::BitwiseRightShiftUnsigned,
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
            Punctuator::EqEq => Self::Equality,
            Punctuator::BangEq => Self::Inequality,
            Punctuator::EqEqEq => Self::StrictEquality,
            Punctuator::BangEqEq => Self::StrictInequality,
            Punctuator::Gt => Self::GreaterThan,
            Punctuator::GtEq => Self::GreaterThanOrEqual,
            Punctuator::Lt => Self::LessThan,
            Punctuator::LtEq => Self::LessThanOrEqual,
            _ => return None,
        })
    }
}

impl TryParse for UnaryOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        Some(match (punc, pos) {
            (Punctuator::Plus, Position::Prefix) => Self::NumericPlus,
            (Punctuator::Minus, Position::Prefix) => Self::NumericNegation,
            (Punctuator::Tilde, Position::Prefix) => Self::BitwiseNot,
            (Punctuator::Bang, Position::Prefix) => Self::LogicalNot,
            (_, _) => return None,
        })
    }
}

impl TryParse for UpdateOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        Some(match (punc, pos) {
            (Punctuator::PlusPlus, Position::PostfixOrInfix) => Self::GetAndIncrement,
            (Punctuator::PlusPlus, Position::Prefix) => Self::IncrementAndGet,
            (Punctuator::MinusMinus, Position::PostfixOrInfix) => Self::GetAndDecrement,
            (Punctuator::MinusMinus, Position::Prefix) => Self::DecrementAndGet,
            (_, _) => return None,
        })
    }
}

impl TryParse for MemberOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        Some(match (punc, pos) {
            (Punctuator::OpenParen, Position::PostfixOrInfix) => Self::FunctionCall,
            (Punctuator::Dot, Position::PostfixOrInfix) => Self::MemberAccess,
            (Punctuator::OpenBracket, Position::PostfixOrInfix) => Self::ComputedMemberAccess,
            (_, _) => return None,
        })
    }
}

impl TryParse for GroupingOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        matches!((punc, pos), (Punctuator::OpenParen, Position::Prefix)).then_some(Self)
    }
}

impl TryParse for TernaryOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        matches!(
            (punc, pos),
            (Punctuator::Question, Position::PostfixOrInfix)
        )
        .then_some(Self)
    }
}
