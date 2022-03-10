use enumerate::{Enumerate, EnumerateStr};

#[derive(Enumerate, EnumerateStr, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Punctuator {
    #[enumerate_str(rename = "&")]
    Ampersand,
    #[enumerate_str(rename = "&=")]
    AmpersandEqual,
    #[enumerate_str(rename = "*")]
    Asterisk,
    #[enumerate_str(rename = "*=")]
    AsteriskEqual,
    #[enumerate_str(rename = "!")]
    Bang,
    #[enumerate_str(rename = "!==")]
    BangDoubleEqual,
    #[enumerate_str(rename = "!=")]
    BangEqual,
    #[enumerate_str(rename = "^")]
    Caret,
    #[enumerate_str(rename = "^=")]
    CaretEqual,
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
    #[enumerate_str(rename = "&&")]
    DoubleAmpersand,
    #[enumerate_str(rename = "**")]
    DoubleAsterisk,
    #[enumerate_str(rename = "**=")]
    DoubleAsteriskEqual,
    #[enumerate_str(rename = "==")]
    DoubleEqual,
    #[enumerate_str(rename = "<<")]
    DoubleLessThan,
    #[enumerate_str(rename = "<<=")]
    DoubleLessThanEqual,
    #[enumerate_str(rename = ">>")]
    DoubleMoreThan,
    #[enumerate_str(rename = ">>=")]
    DoubleMoreThanEqual,
    #[enumerate_str(rename = "--")]
    DoubleMinus,
    #[enumerate_str(rename = "||")]
    DoublePipe,
    #[enumerate_str(rename = "++")]
    DoublePlus,
    #[enumerate_str(rename = "??")]
    DoubleQuestion,
    #[enumerate_str(rename = "=")]
    Equal,
    #[enumerate_str(rename = "=>")]
    EqualMoreThan,
    #[enumerate_str(rename = "<")]
    LessThan,
    #[enumerate_str(rename = "<=")]
    LessThanEqual,
    #[enumerate_str(rename = "-")]
    Minus,
    #[enumerate_str(rename = "-=")]
    MinusEqual,
    #[enumerate_str(rename = ">")]
    MoreThan,
    #[enumerate_str(rename = ">=")]
    MoreThanEqual,
    #[enumerate_str(rename = "{")]
    OpenBrace,
    #[enumerate_str(rename = "[")]
    OpenBracket,
    #[enumerate_str(rename = "(")]
    OpenParen,
    #[enumerate_str(rename = "%")]
    Percent,
    #[enumerate_str(rename = "%=")]
    PercentEqual,
    #[enumerate_str(rename = "|")]
    Pipe,
    #[enumerate_str(rename = "|=")]
    PipeEqual,
    #[enumerate_str(rename = "+")]
    Plus,
    #[enumerate_str(rename = "+=")]
    PlusEqual,
    #[enumerate_str(rename = "?")]
    Question,
    #[enumerate_str(rename = ";")]
    Semicolon,
    #[enumerate_str(rename = "/")]
    Slash,
    #[enumerate_str(rename = "/=")]
    SlashEqual,
    #[enumerate_str(rename = "~")]
    Tilde,
    #[enumerate_str(rename = "...")]
    TripleDot,
    #[enumerate_str(rename = "===")]
    TripleEqual,
    #[enumerate_str(rename = ">>>")]
    TripleMoreThan,
    #[enumerate_str(rename = ">>>=")]
    TripleMoreThanEqual,
}

impl Punctuator {
    /// Unlike for [`Self::VALUES`], **order is important**. For multiple punctuators which start
    /// with the same substring, the longest needs to come first. This is relied on by the `Lexer`.
    pub(crate) fn enumerate_in_lexical_order() -> &'static [Self] {
        const VALUES_IN_LEXICAL_ORDER: &[Punctuator] = &[
            Punctuator::DoubleAmpersand,
            Punctuator::AmpersandEqual,
            Punctuator::Ampersand,
            Punctuator::DoubleAsteriskEqual,
            Punctuator::DoubleAsterisk,
            Punctuator::AsteriskEqual,
            Punctuator::Asterisk,
            Punctuator::BangDoubleEqual,
            Punctuator::BangEqual,
            Punctuator::Bang,
            Punctuator::CaretEqual,
            Punctuator::Caret,
            Punctuator::CloseBrace,
            Punctuator::CloseBracket,
            Punctuator::CloseParen,
            Punctuator::Colon,
            Punctuator::Comma,
            Punctuator::TripleDot,
            Punctuator::Dot,
            Punctuator::TripleEqual,
            Punctuator::DoubleEqual,
            Punctuator::EqualMoreThan,
            Punctuator::Equal,
            Punctuator::DoubleLessThanEqual,
            Punctuator::DoubleLessThan,
            Punctuator::LessThanEqual,
            Punctuator::LessThan,
            Punctuator::DoubleMinus,
            Punctuator::MinusEqual,
            Punctuator::Minus,
            Punctuator::TripleMoreThanEqual,
            Punctuator::TripleMoreThan,
            Punctuator::DoubleMoreThanEqual,
            Punctuator::DoubleMoreThan,
            Punctuator::MoreThanEqual,
            Punctuator::MoreThan,
            Punctuator::OpenBrace,
            Punctuator::OpenBracket,
            Punctuator::OpenParen,
            Punctuator::PercentEqual,
            Punctuator::Percent,
            Punctuator::DoublePipe,
            Punctuator::PipeEqual,
            Punctuator::Pipe,
            Punctuator::DoublePlus,
            Punctuator::PlusEqual,
            Punctuator::Plus,
            Punctuator::DoubleQuestion,
            Punctuator::Question,
            Punctuator::Semicolon,
            Punctuator::SlashEqual,
            Punctuator::Slash,
            Punctuator::Tilde,
        ];
        VALUES_IN_LEXICAL_ORDER
    }
}
