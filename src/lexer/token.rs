use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    Literal(Literal),
    Punctuator(Punctuator),

    Comment(String, CommentKind),
    LineTerminator,
    Whitespace(char),
}

impl Token {
    pub fn is_significant(&self) -> bool {
        match self {
            Self::Identifier(..) | Self::Keyword(..) | Self::Literal(..) | Self::Punctuator(..) => {
                true
            }
            Self::Comment(..) | Self::LineTerminator | Self::Whitespace(..) => false,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Identifier(it) => write!(f, "Identifier<{}>", it),
            Self::Keyword(it) => write!(f, "Keyword<{}>", it),
            Self::Literal(it) => write!(f, "Literal<{}>", it),
            Self::Punctuator(it) => write!(f, "Punctuator<{}>", it),

            Self::Comment(it, kind) => write!(f, "Comment({:?})<{}>", kind, it),
            Self::LineTerminator => write!(f, "LineTerminator"),
            Self::Whitespace(it) => write!(f, "Whitespace<{}>", it),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Keyword {
    // TODO: Some of these are only _contextually_ disallowed as identifiers, and under certain
    // conditions _can_ be used as identifiers.
    As,
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
    Yield,
}

impl Keyword {
    pub(crate) const VALUES: [Self; 50] = [
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
        Self::Yield,
    ];

    pub fn is_future_reserved_word(&self, strict_mode: bool) -> bool {
        match self {
            Self::Enum => true,
            Self::Implements
            | Self::Interface
            | Self::Package
            | Self::Private
            | Self::Protected
            | Self::Public => strict_mode,
            _ => false,
        }
    }

    pub fn to_str(&self) -> &'static str {
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
            Self::Yield => "yield",
        }
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

impl FromStr for Keyword {
    type Err = BadKeywordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for value in Self::VALUES {
            if value.to_str() == s {
                return Ok(value);
            }
        }
        Err(BadKeywordError)
    }
}

// TODO: Support RegEx literals
// TODO: Support decimal numeric literals
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Literal {
    Boolean(bool),
    Null,
    Numeric(i64),
    String(String),
    Undefined,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(it) => write!(f, "{}", it),
            Self::Null => write!(f, "null"),
            Self::Numeric(it) => write!(f, "{}", it),
            Self::String(it) => write!(f, r#""{}""#, it),
            Self::Undefined => write!(f, "undefined"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

    pub fn to_str(&self) -> &'static str {
        match *self {
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
        f.write_str(self.to_str())
    }
}

impl FromStr for Punctuator {
    type Err = BadPunctuatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for value in Self::VALUES {
            if value.to_str() == s {
                return Ok(value);
            }
        }
        Err(BadPunctuatorError)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CommentKind {
    MultiLine,
    SingleLine,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BadKeywordError;

impl fmt::Display for BadKeywordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("bad keyword")
    }
}

impl std::error::Error for BadKeywordError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BadPunctuatorError;

impl fmt::Display for BadPunctuatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("bad punctuator")
    }
}

impl std::error::Error for BadPunctuatorError {}
