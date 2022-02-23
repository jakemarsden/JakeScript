use crate::lexer::{CR, LF, LS, PS};
use crate::str::NonEmptyString;
use enumerate::{Enumerate, EnumerateStr};
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum Element {
    Token(Token),
    Comment(Comment),
    LineTerminator(LineTerminator),
    Whitespace(char),
}

impl Element {
    pub fn token(self) -> Option<Token> {
        match self {
            Self::Token(token) => Some(token),
            Self::Comment(..) | Self::LineTerminator(..) | Self::Whitespace(..) => None,
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Token(token) => write!(f, "{}", token),
            Self::Comment(content) => write!(f, "{}", content),
            Self::LineTerminator(content) => write!(f, "{}", content),
            Self::Whitespace(content) => write!(f, "{}", content),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Identifier(NonEmptyString),
    Keyword(Keyword),
    Literal(Literal),
    Punctuator(Punctuator),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Identifier(it) => write!(f, "{}", it),
            Self::Keyword(it) => write!(f, "{}", it),
            Self::Literal(it) => write!(f, "{}", it),
            Self::Punctuator(it) => write!(f, "{}", it),
        }
    }
}

// TODO: Some variants should only be _contextually_ disallowed as identifiers, i.e. in certain
//  circumstances they should be allowed as identifiers.
#[derive(Enumerate, EnumerateStr, Copy, Clone, Eq, PartialEq, Debug)]
#[enumerate_str(rename_all = "lowercase")]
pub enum Keyword {
    As,
    Async,
    Await,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Enum,
    Export,
    Extends,
    Finally,
    For,
    From,
    Function,
    Get,
    If,
    Implements,
    Import,
    In,
    Instanceof,
    Interface,
    Let,
    New,
    Of,
    Package,
    Private,
    Protected,
    Public,
    Return,
    Set,
    Static,
    Super,
    Switch,
    Target,
    This,
    Throw,
    Try,
    Typeof,
    Var,
    Void,
    While,
    With,
    Yield,
}

// TODO: Support RegEx literals.
#[derive(Clone, PartialEq, Debug)]
pub enum Literal {
    Boolean(bool),
    Numeric(NumericLiteral),
    String(StringLiteral),
    RegEx(RegExLiteral),
    Null,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(value) => write!(f, "{}", value),
            Self::Numeric(value) => write!(f, "{}", value),
            Self::String(value) => write!(f, "{}", value),
            Self::RegEx(value) => write!(f, "{}", value),
            Self::Null => f.write_str("null"),
        }
    }
}

/// Numeric literal tokens are **always unsigned** (but can be made negative at runtime with the
/// negation unary operator).
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum NumericLiteral {
    BinInt(u64),
    OctInt(u64),
    DecInt(u64),
    HexInt(u64),
    Decimal(f64),
}

impl fmt::Display for NumericLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BinInt(value) => write!(f, "{:#b}", value),
            Self::OctInt(value) => write!(f, "{:#o}", value),
            Self::DecInt(value) => write!(f, "{}", value),
            Self::HexInt(value) => write!(f, "{:#x}", value),
            Self::Decimal(value) => write!(f, "{}", value),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum StringLiteral {
    SingleQuoted(String),
    DoubleQuoted(String),
}

impl fmt::Display for StringLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SingleQuoted(value) => write!(f, r#"'{}'"#, value),
            Self::DoubleQuoted(value) => write!(f, r#""{}""#, value),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct RegExLiteral {
    pub content: NonEmptyString,
    pub flags: Vec<char>,
}

impl fmt::Display for RegExLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "/{}/{}",
            self.content,
            self.flags.iter().collect::<String>()
        )
    }
}

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

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Comment {
    SingleLine(String),
    MultiLine(String),
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SingleLine(content) => write!(f, "//{}", content),
            Self::MultiLine(content) => write!(f, "/*{}*/", content),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum LineTerminator {
    /// Carriage return + line feed
    Crlf,
    /// Carriage return
    Cr,
    /// Line feed
    Lf,
    /// Line separator
    Ls,
    /// Paragraph separator
    Ps,
}

impl LineTerminator {
    pub fn into_chars(self: LineTerminator) -> (char, Option<char>) {
        match self {
            Self::Crlf => (CR, Some(super::LF)),
            Self::Cr => (CR, None),
            Self::Lf => (LF, None),
            Self::Ls => (LS, None),
            Self::Ps => (PS, None),
        }
    }
}

impl fmt::Display for LineTerminator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Write;
        match self.into_chars() {
            (ch0, Some(ch1)) => write!(f, "{}{}", ch0, ch1),
            (ch0, None) => f.write_char(ch0),
        }
    }
}
