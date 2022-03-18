use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Punctuator {
    Amp,
    AmpAmp,
    AmpEq,
    Bang,
    BangEq,
    BangEqEq,
    Caret,
    CaretEq,
    CloseBrace,
    CloseBracket,
    CloseParen,
    Colon,
    Comma,
    Dot,
    DotDotDot,
    Eq,
    EqEq,
    EqEqEq,
    EqGt,
    Gt,
    GtEq,
    GtGt,
    GtGtEq,
    GtGtGt,
    GtGtGtEq,
    Lt,
    LtEq,
    LtLt,
    LtLtEq,
    Minus,
    MinusEq,
    MinusMinus,
    OpenBrace,
    OpenBracket,
    OpenParen,
    Percent,
    PercentEq,
    Pipe,
    PipeEq,
    PipePipe,
    Plus,
    PlusEq,
    PlusPlus,
    Question,
    QuestionQuestion,
    Semi,
    Slash,
    SlashEq,
    Star,
    StarEq,
    StarStar,
    StarStarEq,
    Tilde,
}

impl Punctuator {
    const ALL: &'static [Self] = &[
        Self::Amp,
        Self::AmpAmp,
        Self::AmpEq,
        Self::Bang,
        Self::BangEq,
        Self::BangEqEq,
        Self::Caret,
        Self::CaretEq,
        Self::CloseBrace,
        Self::CloseBracket,
        Self::CloseParen,
        Self::Colon,
        Self::Comma,
        Self::Dot,
        Self::DotDotDot,
        Self::Eq,
        Self::EqEq,
        Self::EqEqEq,
        Self::EqGt,
        Self::Gt,
        Self::GtEq,
        Self::GtGt,
        Self::GtGtEq,
        Self::GtGtGt,
        Self::GtGtGtEq,
        Self::Lt,
        Self::LtEq,
        Self::LtLt,
        Self::LtLtEq,
        Self::Minus,
        Self::MinusEq,
        Self::MinusMinus,
        Self::OpenBrace,
        Self::OpenBracket,
        Self::OpenParen,
        Self::Percent,
        Self::PercentEq,
        Self::Pipe,
        Self::PipeEq,
        Self::PipePipe,
        Self::Plus,
        Self::PlusEq,
        Self::PlusPlus,
        Self::Question,
        Self::QuestionQuestion,
        Self::Semi,
        Self::Slash,
        Self::SlashEq,
        Self::Star,
        Self::StarEq,
        Self::StarStar,
        Self::StarStarEq,
        Self::Tilde,
    ];

    pub fn all() -> &'static [Self] {
        Self::ALL
    }

    /// Unlike for [`Self::all()`], **order is important**. For multiple punctuators which start
    /// with the same substring, the longest needs to come first. This is relied on by the `Lexer`.
    pub fn all_in_lexical_order() -> Vec<Self> {
        let mut values = Self::all().to_vec();
        values.sort_unstable_by_key(|value| usize::MAX - value.as_str().len());
        values
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Amp => "&",
            Self::AmpAmp => "&&",
            Self::AmpEq => "&=",
            Self::Bang => "!",
            Self::BangEq => "!=",
            Self::BangEqEq => "!==",
            Self::Caret => "^",
            Self::CaretEq => "^=",
            Self::CloseBrace => "}",
            Self::CloseBracket => "]",
            Self::CloseParen => ")",
            Self::Colon => ":",
            Self::Comma => ",",
            Self::Dot => ".",
            Self::DotDotDot => "...",
            Self::Eq => "=",
            Self::EqEq => "==",
            Self::EqEqEq => "===",
            Self::EqGt => "=>",
            Self::Gt => ">",
            Self::GtEq => ">=",
            Self::GtGt => ">>",
            Self::GtGtEq => ">>=",
            Self::GtGtGt => ">>>",
            Self::GtGtGtEq => ">>>=",
            Self::Lt => "<",
            Self::LtEq => "<=",
            Self::LtLt => "<<",
            Self::LtLtEq => "<<=",
            Self::Minus => "-",
            Self::MinusEq => "-=",
            Self::MinusMinus => "--",
            Self::OpenBrace => "{",
            Self::OpenBracket => "[",
            Self::OpenParen => "(",
            Self::Percent => "%",
            Self::PercentEq => "%=",
            Self::Pipe => "|",
            Self::PipeEq => "|=",
            Self::PipePipe => "||",
            Self::Plus => "+",
            Self::PlusEq => "+=",
            Self::PlusPlus => "++",
            Self::Question => "?",
            Self::QuestionQuestion => "??",
            Self::Semi => ";",
            Self::Slash => "/",
            Self::SlashEq => "/=",
            Self::Star => "*",
            Self::StarEq => "*=",
            Self::StarStar => "**",
            Self::StarStarEq => "**=",
            Self::Tilde => "~",
        }
    }
}

impl FromStr for Punctuator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::all()
            .iter()
            .find(|value| value.as_str() == s)
            .copied()
            .ok_or(())
    }
}

impl fmt::Display for Punctuator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
