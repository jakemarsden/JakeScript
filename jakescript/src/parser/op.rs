use crate::ast::*;
use crate::token::Punctuator::{
    self, Amp, AmpAmp, AmpEq, Bang, BangEq, BangEqEq, Caret, CaretEq, Dot, Eq, EqEq, EqEqEq, Gt,
    GtEq, GtGt, GtGtEq, GtGtGt, GtGtGtEq, Lt, LtEq, LtLt, LtLtEq, Minus, MinusEq, MinusMinus,
    OpenBracket, OpenParen, Percent, PercentEq, Pipe, PipeEq, PipePipe, Plus, PlusEq, PlusPlus,
    Question, Slash, SlashEq, Star, StarEq, StarStar, StarStarEq, Tilde,
};

pub(super) trait ParseOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self>
    where
        Self: Sized;
}

impl ParseOperator for Operator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        AssignmentOperator::try_parse(punc, pos)
            .map(Self::Assignment)
            .or_else(|| BinaryOperator::try_parse(punc, pos).map(Self::Binary))
            .or_else(|| RelationalOperator::try_parse(punc, pos).map(Self::Relational))
            .or_else(|| UnaryOperator::try_parse(punc, pos).map(Self::Unary))
            .or_else(|| UpdateOperator::try_parse(punc, pos).map(Self::Update))
            .or_else(|| {
                Some(match (punc, pos) {
                    (OpenBracket, Position::PostfixOrInfix) => Self::ComputedMemberAccess,
                    (Dot, Position::PostfixOrInfix) => Self::MemberAccess,

                    (OpenParen, Position::PostfixOrInfix) => Self::FunctionCall,

                    (OpenParen, Position::Prefix) => Self::Grouping,
                    (Question, Position::PostfixOrInfix) => Self::Ternary,
                    (_, _) => return None,
                })
            })
    }
}

impl ParseOperator for AssignmentOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        let Position::PostfixOrInfix = pos else { return None };
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

impl ParseOperator for BinaryOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        let Position::PostfixOrInfix = pos else { return None };
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

impl ParseOperator for RelationalOperator {
    fn try_parse(punc: Punctuator, pos: Position) -> Option<Self> {
        let Position::PostfixOrInfix = pos else { return None };
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

impl ParseOperator for UnaryOperator {
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

impl ParseOperator for UpdateOperator {
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
