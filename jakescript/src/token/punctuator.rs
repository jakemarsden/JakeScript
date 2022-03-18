use enumerate::{Enumerate, EnumerateStr};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Enumerate, EnumerateStr)]
pub enum Punctuator {
    #[enumerate_str(rename = "&")]
    Amp,
    #[enumerate_str(rename = "&&")]
    AmpAmp,
    #[enumerate_str(rename = "&=")]
    AmpEq,
    #[enumerate_str(rename = "!")]
    Bang,
    #[enumerate_str(rename = "!=")]
    BangEq,
    #[enumerate_str(rename = "!==")]
    BangEqEq,
    #[enumerate_str(rename = "^")]
    Caret,
    #[enumerate_str(rename = "^=")]
    CaretEq,
    #[enumerate_str(rename = "}")]
    CloseBrace,
    #[enumerate_str(rename = "]")]
    CloseBracket,
    #[enumerate_str(rename = ")")]
    CloseParen,
    #[enumerate_str(rename = ":")]
    Colon,
    #[enumerate_str(rename = ",")]
    Comma,
    #[enumerate_str(rename = ".")]
    Dot,
    #[enumerate_str(rename = "...")]
    DotDotDot,
    #[enumerate_str(rename = "=")]
    Eq,
    #[enumerate_str(rename = "==")]
    EqEq,
    #[enumerate_str(rename = "===")]
    EqEqEq,
    #[enumerate_str(rename = "=>")]
    EqGt,
    #[enumerate_str(rename = ">")]
    Gt,
    #[enumerate_str(rename = ">=")]
    GtEq,
    #[enumerate_str(rename = ">>")]
    GtGt,
    #[enumerate_str(rename = ">>=")]
    GtGtEq,
    #[enumerate_str(rename = ">>>")]
    GtGtGt,
    #[enumerate_str(rename = ">>>=")]
    GtGtGtEq,
    #[enumerate_str(rename = "<")]
    Lt,
    #[enumerate_str(rename = "<=")]
    LtEq,
    #[enumerate_str(rename = "<<")]
    LtLt,
    #[enumerate_str(rename = "<<=")]
    LtLtEq,
    #[enumerate_str(rename = "-")]
    Minus,
    #[enumerate_str(rename = "-=")]
    MinusEq,
    #[enumerate_str(rename = "--")]
    MinusMinus,
    #[enumerate_str(rename = "{")]
    OpenBrace,
    #[enumerate_str(rename = "[")]
    OpenBracket,
    #[enumerate_str(rename = "(")]
    OpenParen,
    #[enumerate_str(rename = "%")]
    Percent,
    #[enumerate_str(rename = "%=")]
    PercentEq,
    #[enumerate_str(rename = "|")]
    Pipe,
    #[enumerate_str(rename = "|=")]
    PipeEq,
    #[enumerate_str(rename = "||")]
    PipePipe,
    #[enumerate_str(rename = "+")]
    Plus,
    #[enumerate_str(rename = "+=")]
    PlusEq,
    #[enumerate_str(rename = "++")]
    PlusPlus,
    #[enumerate_str(rename = "?")]
    Question,
    #[enumerate_str(rename = "??")]
    QuestionQuestion,
    #[enumerate_str(rename = ";")]
    Semi,
    #[enumerate_str(rename = "/")]
    Slash,
    #[enumerate_str(rename = "/=")]
    SlashEq,
    #[enumerate_str(rename = "*")]
    Star,
    #[enumerate_str(rename = "*=")]
    StarEq,
    #[enumerate_str(rename = "**")]
    StarStar,
    #[enumerate_str(rename = "**=")]
    StarStarEq,
    #[enumerate_str(rename = "~")]
    Tilde,
}

impl Punctuator {
    /// Unlike for [`Self::VALUES`], **order is important**. For multiple punctuators which start
    /// with the same substring, the longest needs to come first. This is relied on by the `Lexer`.
    pub(crate) fn enumerate_in_lexical_order() -> &'static [Self] {
        const VALUES_IN_LEXICAL_ORDER: &[Punctuator] = &[
            Punctuator::AmpAmp,
            Punctuator::AmpEq,
            Punctuator::Amp,
            Punctuator::BangEqEq,
            Punctuator::BangEq,
            Punctuator::Bang,
            Punctuator::CaretEq,
            Punctuator::Caret,
            Punctuator::CloseBrace,
            Punctuator::CloseBracket,
            Punctuator::CloseParen,
            Punctuator::Colon,
            Punctuator::Comma,
            Punctuator::DotDotDot,
            Punctuator::Dot,
            Punctuator::EqEqEq,
            Punctuator::EqEq,
            Punctuator::EqGt,
            Punctuator::Eq,
            Punctuator::GtGtEq,
            Punctuator::GtEq,
            Punctuator::GtGtGtEq,
            Punctuator::GtGtGt,
            Punctuator::GtGt,
            Punctuator::Gt,
            Punctuator::LtLtEq,
            Punctuator::LtEq,
            Punctuator::LtLt,
            Punctuator::Lt,
            Punctuator::MinusEq,
            Punctuator::MinusMinus,
            Punctuator::Minus,
            Punctuator::OpenBrace,
            Punctuator::OpenBracket,
            Punctuator::OpenParen,
            Punctuator::PercentEq,
            Punctuator::Percent,
            Punctuator::PipeEq,
            Punctuator::PipePipe,
            Punctuator::Pipe,
            Punctuator::PlusEq,
            Punctuator::PlusPlus,
            Punctuator::Plus,
            Punctuator::QuestionQuestion,
            Punctuator::Question,
            Punctuator::Semi,
            Punctuator::SlashEq,
            Punctuator::Slash,
            Punctuator::StarStarEq,
            Punctuator::StarEq,
            Punctuator::StarStar,
            Punctuator::Star,
            Punctuator::Tilde,
        ];
        VALUES_IN_LEXICAL_ORDER
    }
}
