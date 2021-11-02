use crate::lexer::error::{BadKeywordError, BadPunctuatorError};
use crate::lexer::{CR, LF, LS, PS};
use std::fmt;
use std::str::FromStr;

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
            Self::Token(token) => write!(f, "Token<{}>", token),
            Self::Comment(content) => write!(f, "Comment<{}>", content),
            Self::LineTerminator(content) => write!(f, "LineTerminator<{}>", content),
            Self::Whitespace(content) => write!(f, "Whitespace<{}>", content),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    Literal(Literal),
    Punctuator(Punctuator),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Identifier(it) => write!(f, "Identifier<{}>", it),
            Self::Keyword(it) => write!(f, "Keyword<{}>", it),
            Self::Literal(it) => write!(f, "Literal<{}>", it),
            Self::Punctuator(it) => write!(f, "Punctuator<{}>", it),
        }
    }
}

// TODO: Some variants should only be _contextually_ disallowed as identifiers, i.e. in certain
//  circumstances they should be allowed as identifiers.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Keyword {
    As,
    // TODO: Remove from the language once no longer needed to support unit tests.
    Assert,
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
    // TODO: Remove from the language once no longer needed to support unit tests.
    Print,
    // TODO: Remove from the language once no longer needed to support unit tests.
    PrintLn,
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

impl Keyword {
    pub(crate) const VALUES: [Self; 53] = [
        Self::As,
        Self::Assert,
        Self::Async,
        Self::Await,
        Self::Break,
        Self::Case,
        Self::Catch,
        Self::Class,
        Self::Const,
        Self::Continue,
        Self::Debugger,
        Self::Default,
        Self::Delete,
        Self::Do,
        Self::Else,
        Self::Enum,
        Self::Export,
        Self::Extends,
        Self::Finally,
        Self::For,
        Self::From,
        Self::Function,
        Self::Get,
        Self::If,
        Self::Implements,
        Self::Import,
        Self::In,
        Self::Instanceof,
        Self::Interface,
        Self::Let,
        Self::New,
        Self::Of,
        Self::Package,
        Self::Print,
        Self::PrintLn,
        Self::Private,
        Self::Protected,
        Self::Public,
        Self::Return,
        Self::Set,
        Self::Static,
        Self::Super,
        Self::Switch,
        Self::Target,
        Self::This,
        Self::Throw,
        Self::Try,
        Self::Typeof,
        Self::Var,
        Self::Void,
        Self::While,
        Self::With,
        Self::Yield,
    ];

    pub fn into_str(self) -> &'static str {
        match self {
            Self::As => "as",
            Self::Assert => "assert",
            Self::Async => "async",
            Self::Await => "await",
            Self::Break => "break",
            Self::Case => "case",
            Self::Catch => "catch",
            Self::Class => "class",
            Self::Const => "const",
            Self::Continue => "continue",
            Self::Debugger => "debugger",
            Self::Default => "default",
            Self::Delete => "delete",
            Self::Do => "do",
            Self::Else => "else",
            Self::Enum => "enum",
            Self::Export => "export",
            Self::Extends => "extends",
            Self::Finally => "finally",
            Self::For => "for",
            Self::From => "from",
            Self::Function => "function",
            Self::Get => "get",
            Self::If => "if",
            Self::Implements => "implements",
            Self::Import => "import",
            Self::Interface => "interface",
            Self::In => "in",
            Self::Instanceof => "instanceof",
            Self::Let => "let",
            Self::New => "new",
            Self::Of => "of",
            Self::Package => "package",
            Self::Print => "print",
            Self::PrintLn => "println",
            Self::Private => "private",
            Self::Protected => "protected",
            Self::Public => "public",
            Self::Return => "return",
            Self::Set => "set",
            Self::Static => "static",
            Self::Super => "super",
            Self::Switch => "switch",
            Self::Target => "target",
            Self::This => "this",
            Self::Throw => "throw",
            Self::Try => "try",
            Self::Typeof => "typeof",
            Self::Var => "var",
            Self::Void => "void",
            Self::While => "while",
            Self::With => "with",
            Self::Yield => "yield",
        }
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.into_str())
    }
}

impl FromStr for Keyword {
    type Err = BadKeywordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::VALUES
            .iter()
            .find(|value| value.into_str() == s)
            .cloned()
            .ok_or(BadKeywordError)
    }
}

// TODO: Support RegEx literals.
#[derive(Clone, PartialEq, Debug)]
pub enum Literal {
    Boolean(bool),
    Numeric(NumericLiteral),
    String(StringLiteral),
    Null,
    Undefined,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(value) => write!(f, "{}", value),
            Self::Numeric(value) => write!(f, "{}", value),
            Self::String(value) => write!(f, "{}", value),
            Self::Null => f.write_str("null"),
            Self::Undefined => f.write_str("undefined"),
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Punctuator {
    Ampersand,
    AmpersandEqual,
    Asterisk,
    AsteriskEqual,
    Bang,
    BangDoubleEqual,
    BangEqual,
    Caret,
    CaretEqual,
    CloseBrace,
    CloseBracket,
    CloseParen,
    Colon,
    Comma,
    Dot,
    DoubleAmpersand,
    DoubleAsterisk,
    DoubleAsteriskEqual,
    DoubleEqual,
    DoubleLessThan,
    DoubleLessThanEqual,
    DoubleMoreThan,
    DoubleMoreThanEqual,
    DoubleMinus,
    DoublePipe,
    DoublePlus,
    DoubleQuestion,
    Equal,
    EqualMoreThan,
    LessThan,
    LessThanEqual,
    Minus,
    MinusEqual,
    MoreThan,
    MoreThanEqual,
    OpenBrace,
    OpenBracket,
    OpenParen,
    Percent,
    PercentEqual,
    Pipe,
    PipeEqual,
    Plus,
    PlusEqual,
    Question,
    Semicolon,
    Slash,
    SlashEqual,
    Tilde,
    TripleDot,
    TripleEqual,
    TripleMoreThan,
    TripleMoreThanEqual,
}

impl Punctuator {
    pub(crate) const VALUES: [Self; 53] = Self::VALUES_IN_LEXICAL_ORDER;

    /// Unlike for [VALUES], **order is important**. For multiple punctuators which start with the
    /// same substring, the longest needs to come first. This is relied on by the [Lexer].
    pub(crate) const VALUES_IN_LEXICAL_ORDER: [Self; 53] = [
        Self::DoubleAmpersand,
        Self::AmpersandEqual,
        Self::Ampersand,
        Self::DoubleAsteriskEqual,
        Self::DoubleAsterisk,
        Self::AsteriskEqual,
        Self::Asterisk,
        Self::BangDoubleEqual,
        Self::BangEqual,
        Self::Bang,
        Self::CaretEqual,
        Self::Caret,
        Self::CloseBrace,
        Self::CloseBracket,
        Self::CloseParen,
        Self::Colon,
        Self::Comma,
        Self::TripleDot,
        Self::Dot,
        Self::TripleEqual,
        Self::DoubleEqual,
        Self::EqualMoreThan,
        Self::Equal,
        Self::DoubleLessThanEqual,
        Self::DoubleLessThan,
        Self::LessThanEqual,
        Self::LessThan,
        Self::DoubleMinus,
        Self::MinusEqual,
        Self::Minus,
        Self::TripleMoreThanEqual,
        Self::TripleMoreThan,
        Self::DoubleMoreThanEqual,
        Self::DoubleMoreThan,
        Self::MoreThanEqual,
        Self::MoreThan,
        Self::OpenBrace,
        Self::OpenBracket,
        Self::OpenParen,
        Self::PercentEqual,
        Self::Percent,
        Self::DoublePipe,
        Self::PipeEqual,
        Self::Pipe,
        Self::DoublePlus,
        Self::PlusEqual,
        Self::Plus,
        Self::DoubleQuestion,
        Self::Question,
        Self::Semicolon,
        Self::SlashEqual,
        Self::Slash,
        Self::Tilde,
    ];

    pub fn into_str(self) -> &'static str {
        match self {
            Self::Ampersand => "&",
            Self::AmpersandEqual => "&=",
            Self::Asterisk => "*",
            Self::AsteriskEqual => "*=",
            Self::Bang => "!",
            Self::BangDoubleEqual => "!==",
            Self::BangEqual => "!=",
            Self::Caret => "^",
            Self::CaretEqual => "^=",
            Self::CloseBrace => "}",
            Self::CloseBracket => "]",
            Self::CloseParen => ")",
            Self::Colon => ":",
            Self::Comma => ",",
            Self::Dot => ".",
            Self::DoubleAmpersand => "&&",
            Self::DoubleAsterisk => "**",
            Self::DoubleAsteriskEqual => "**=",
            Self::DoubleEqual => "==",
            Self::DoubleLessThan => "<<",
            Self::DoubleLessThanEqual => "<<=",
            Self::DoubleMoreThan => ">>",
            Self::DoubleMoreThanEqual => ">>=",
            Self::DoubleMinus => "--",
            Self::DoublePipe => "||",
            Self::DoublePlus => "++",
            Self::DoubleQuestion => "??",
            Self::Equal => "=",
            Self::EqualMoreThan => "=>",
            Self::LessThan => "<",
            Self::LessThanEqual => "<=",
            Self::Minus => "-",
            Self::MinusEqual => "-=",
            Self::MoreThan => ">",
            Self::MoreThanEqual => ">=",
            Self::OpenBrace => "{",
            Self::OpenBracket => "[",
            Self::OpenParen => "(",
            Self::Percent => "%",
            Self::PercentEqual => "%=",
            Self::Pipe => "|",
            Self::PipeEqual => "|=",
            Self::Plus => "+",
            Self::PlusEqual => "+=",
            Self::Question => "?",
            Self::Semicolon => ";",
            Self::Slash => "/",
            Self::SlashEqual => "/=",
            Self::Tilde => "~",
            Self::TripleDot => "...",
            Self::TripleEqual => "===",
            Self::TripleMoreThan => ">>>",
            Self::TripleMoreThanEqual => ">>>=",
        }
    }
}

impl fmt::Display for Punctuator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.into_str())
    }
}

impl FromStr for Punctuator {
    type Err = BadPunctuatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::VALUES
            .iter()
            .find(|value| value.into_str() == s)
            .cloned()
            .ok_or(BadPunctuatorError)
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
            Self::SingleLine(content) => write!(f, "SingleLine<{}>", content),
            Self::MultiLine(content) => write!(f, "MultiLine<{}>", content),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum LineTerminator {
    /// Carriage return
    Cr,
    /// Line feed
    Lf,
    /// Carriage return + line feed
    Crlf,
    /// Line separator
    Ls,
    /// Paragraph separator
    Ps,
}

impl LineTerminator {
    pub fn into_chars(self: LineTerminator) -> (char, Option<char>) {
        match self {
            Self::Cr => (CR, None),
            Self::Lf => (LF, None),
            Self::Crlf => (CR, Some(super::LF)),
            Self::Ls => (LS, None),
            Self::Ps => (PS, None),
        }
    }
}

impl fmt::Display for LineTerminator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Cr => "CR",
            Self::Lf => "LF",
            Self::Crlf => "CRLF",
            Self::Ls => "LS",
            Self::Ps => "PS",
        })
    }
}
