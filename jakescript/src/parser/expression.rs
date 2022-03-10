use super::block::Braces;
use super::error::AllowToken::{AnyOf, Unspecified};
use super::error::{Error, Result};
use super::Parser;
use crate::ast::*;
use crate::lexer;
use crate::non_empty_str;
use crate::token::{self, Keyword, Punctuator, StringLiteral, Token};

trait TryParse {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(super) enum Position {
    /// For example:
    /// - `++a`
    Prefix,
    /// For example:
    /// - `a++`
    /// - `a + b`
    PostfixOrInfix,
}

impl<I: Iterator<Item = lexer::Result<Token>>> Parser<I> {
    pub(super) fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_expression_impl(Precedence::MIN)
    }

    fn parse_expression_impl(&mut self, min_precedence: Precedence) -> Result<Expression> {
        let mut expression = self.parse_primary_expression()?;
        while let Some(&Token::Punctuator(punctuator)) = self.tokens.try_peek()? {
            if let Some(op_kind) = Operator::try_parse(punctuator, Position::PostfixOrInfix) {
                if op_kind.precedence() > min_precedence {
                    self.tokens.try_next().unwrap().unwrap();
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
        Ok(match self.tokens.try_next()? {
            Some(Token::Identifier(var_name)) => {
                Expression::VariableAccess(VariableAccessExpression {
                    var_name: Identifier::from(var_name),
                })
            }
            Some(Token::Punctuator(Punctuator::OpenBracket)) => {
                let elems = self.parse_array_literal_elements()?;
                self.expect_punctuator(Punctuator::CloseBracket)?;
                Expression::Literal(LiteralExpression {
                    value: Literal::Array(elems),
                })
            }
            Some(Token::Punctuator(Punctuator::OpenBrace)) => {
                let props = self.parse_object_properties()?;
                self.expect_punctuator(Punctuator::CloseBrace)?;
                Expression::Literal(LiteralExpression {
                    value: Literal::Object(props),
                })
            }
            Some(Token::Punctuator(punc)) => {
                if let Some(op_kind) = UnaryOperator::try_parse(punc, Position::Prefix) {
                    let operand = self.parse_expression_impl(op_kind.precedence())?;
                    Expression::Unary(UnaryExpression {
                        op: op_kind,
                        operand: Box::new(operand),
                    })
                } else if GroupingOperator::try_parse(punc, Position::Prefix).is_some() {
                    let inner = self.parse_expression()?;
                    self.expect_punctuator(Punctuator::CloseParen)?;
                    Expression::Grouping(GroupingExpression {
                        inner: Box::new(inner),
                    })
                } else {
                    return Err(Error::unexpected_token(
                        Unspecified,
                        Token::Punctuator(punc),
                    ));
                }
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
                        todo!("NumericLiteral::Decimal: {}", value)
                    }
                    token::Literal::String(
                        StringLiteral::SingleQuoted(value) | StringLiteral::DoubleQuoted(value),
                    ) => Literal::String(value),
                    token::Literal::RegEx(value) => {
                        // FIXME: Support Literal::RegEx properly"
                        Literal::String(value.to_string())
                    }
                    token::Literal::Null => Literal::Null,
                },
            }),
            Some(Token::Keyword(Keyword::Function)) => {
                let name = match self.tokens.try_peek()? {
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
                let param_names = self.parse_fn_parameters()?;
                let body = self.parse_block(Braces::Require)?;
                Expression::Literal(LiteralExpression {
                    value: Literal::Function {
                        name,
                        param_names,
                        body,
                    },
                })
            }
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
            Operator::Unary(kind) => Expression::Unary(UnaryExpression {
                op: kind,
                operand: Box::new(lhs),
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
            Operator::Grouping => Expression::Grouping(GroupingExpression {
                inner: Box::new(lhs),
            }),
            Operator::FunctionCall => Expression::FunctionCall(FunctionCallExpression {
                function: Box::new(lhs),
                arguments: self.parse_fn_arguments(false)?,
            }),
            Operator::PropertyAccess => {
                let rhs = self.parse_expression_impl(op_kind.precedence())?;
                Expression::PropertyAccess(PropertyAccessExpression {
                    base: Box::new(lhs),
                    property_name: match rhs {
                        Expression::VariableAccess(VariableAccessExpression { var_name }) => {
                            var_name
                        }
                        rhs_expr => todo!(
                            "Unsupported property access expression (only simple `a.b` \
                             expressions are currently supported): {:?}",
                            rhs_expr
                        ),
                    },
                })
            }
            Operator::ComputedPropertyAccess => {
                let rhs = self.parse_expression()?;
                self.expect_punctuator(Punctuator::CloseBracket)?;
                Expression::ComputedPropertyAccess(ComputedPropertyAccessExpression {
                    base: Box::new(lhs),
                    property: Box::new(rhs),
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
            .try_next_if_eq(&Token::Punctuator(Punctuator::CloseParen))?
            .is_some()
        {
            return Ok(Vec::with_capacity(0));
        }

        let mut args = Vec::new();
        loop {
            args.push(self.parse_expression()?);
            match self.tokens.try_next()? {
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
            .or_else(|| UnaryOperator::try_parse(punc, pos).map(Self::Unary))
            .or_else(|| TernaryOperator::try_parse(punc, pos).map(|_| Self::Ternary))
            .or_else(|| GroupingOperator::try_parse(punc, pos).map(|_| Self::Grouping))
            .or_else(|| FunctionCallOperator::try_parse(punc, pos).map(|_| Self::FunctionCall))
            .or_else(|| PropertyAccessOperator::try_parse(punc, pos).map(|_| Self::PropertyAccess))
            .or_else(|| {
                ComputedPropertyAccessOperator::try_parse(punc, pos)
                    .map(|_| Self::ComputedPropertyAccess)
            })
    }
}

impl TryParse for AssignmentOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        if pos != Position::PostfixOrInfix {
            return None;
        }
        Some(match punc {
            Punctuator::Equal => Self::Assign,
            Punctuator::PlusEqual => Self::AddAssign,
            Punctuator::SlashEqual => Self::DivAssign,
            Punctuator::PercentEqual => Self::ModAssign,
            Punctuator::AsteriskEqual => Self::MulAssign,
            Punctuator::DoubleAsteriskEqual => Self::PowAssign,
            Punctuator::MinusEqual => Self::SubAssign,
            Punctuator::DoubleLessThanEqual => Self::ShiftLeftAssign,
            Punctuator::DoubleMoreThanEqual => Self::ShiftRightAssign,
            Punctuator::TripleMoreThanEqual => Self::ShiftRightUnsignedAssign,
            Punctuator::AmpersandEqual => Self::BitwiseAndAssign,
            Punctuator::PipeEqual => Self::BitwiseOrAssign,
            Punctuator::CaretEqual => Self::BitwiseXOrAssign,
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
            Punctuator::Plus => Self::Add,
            Punctuator::Slash => Self::Div,
            Punctuator::Percent => Self::Mod,
            Punctuator::Asterisk => Self::Mul,
            Punctuator::DoubleAsterisk => Self::Pow,
            Punctuator::Minus => Self::Sub,
            Punctuator::DoubleEqual => Self::Equal,
            Punctuator::BangEqual => Self::NotEqual,
            Punctuator::TripleEqual => Self::Identical,
            Punctuator::BangDoubleEqual => Self::NotIdentical,
            Punctuator::LessThan => Self::LessThan,
            Punctuator::LessThanEqual => Self::LessThanOrEqual,
            Punctuator::MoreThan => Self::MoreThan,
            Punctuator::MoreThanEqual => Self::MoreThanOrEqual,
            Punctuator::DoubleLessThan => Self::ShiftLeft,
            Punctuator::DoubleMoreThan => Self::ShiftRight,
            Punctuator::TripleMoreThan => Self::ShiftRightUnsigned,
            Punctuator::Ampersand => Self::BitwiseAnd,
            Punctuator::Pipe => Self::BitwiseOr,
            Punctuator::Caret => Self::BitwiseXOr,
            Punctuator::DoubleAmpersand => Self::LogicalAnd,
            Punctuator::DoublePipe => Self::LogicalOr,
            _ => return None,
        })
    }
}

impl TryParse for UnaryOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        Some(match (punc, pos) {
            (Punctuator::DoubleMinus, Position::Prefix) => Self::DecrementPrefix,
            (Punctuator::DoubleMinus, Position::PostfixOrInfix) => Self::DecrementPostfix,
            (Punctuator::DoublePlus, Position::Prefix) => Self::IncrementPrefix,
            (Punctuator::DoublePlus, Position::PostfixOrInfix) => Self::IncrementPostfix,
            (Punctuator::Tilde, Position::Prefix) => Self::BitwiseNot,
            (Punctuator::Bang, Position::Prefix) => Self::LogicalNot,
            (Punctuator::Minus, Position::Prefix) => Self::NumericNegate,
            (Punctuator::Plus, Position::Prefix) => Self::NumericPlus,
            (_, _) => return None,
        })
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

impl TryParse for GroupingOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        matches!((punc, pos), (Punctuator::OpenParen, Position::Prefix)).then_some(Self)
    }
}

impl TryParse for FunctionCallOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        matches!(
            (punc, pos),
            (Punctuator::OpenParen, Position::PostfixOrInfix)
        )
        .then_some(Self)
    }
}

impl TryParse for PropertyAccessOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        matches!((punc, pos), (Punctuator::Dot, Position::PostfixOrInfix)).then_some(Self)
    }
}

impl TryParse for ComputedPropertyAccessOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        matches!(
            (punc, pos),
            (Punctuator::OpenBracket, Position::PostfixOrInfix)
        )
        .then_some(Self)
    }
}
