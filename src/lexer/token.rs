use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    Literal(Literal),
    Symbol(Symbol),

    Invalid(LexicalError),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Identifier(it) => write!(f, "Identifier<{}>", it),
            Self::Keyword(it) => write!(f, "Keyword<{}>", it),
            Self::Literal(it) => write!(f, "Literal<{}>", it),
            Self::Symbol(it) => write!(f, "Symbol<{}>", it),

            Self::Invalid(it) => write!(f, "Invalid<{}>", it),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Keyword {
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
    Function,
    If,
    Import,
    In,
    Instanceof,
    // TODO: `let` is not in the spec as a _ReservedWord_?
    Let,
    New,
    Return,
    Super,
    Switch,
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
    pub(crate) const VALUES: [Self; 35] = [
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
        Self::Function,
        Self::If,
        Self::Import,
        Self::In,
        Self::Instanceof,
        Self::Let,
        Self::New,
        Self::Return,
        Self::Super,
        Self::Switch,
        Self::This,
        Self::Throw,
        Self::Try,
        Self::Typeof,
        Self::Var,
        Self::Void,
        Self::While,
        Self::Yield,
    ];

    pub fn to_str(&self) -> &'static str {
        match self {
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
            Self::Function => "function",
            Self::If => "if",
            Self::Import => "import",
            Self::In => "in",
            Self::Instanceof => "instanceof",
            Self::Let => "let",
            Self::New => "new",
            Self::Return => "return",
            Self::Super => "super",
            Self::Switch => "switch",
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
        for keyword in Self::VALUES {
            if keyword.to_str() == s {
                return Ok(keyword);
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
    Numeric(u64),
    String(String),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(it) => write!(f, "{}", it),
            Self::Null => write!(f, "null"),
            Self::Numeric(it) => write!(f, "{}", it),
            Self::String(it) => write!(f, r#""{}""#, it),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Symbol {
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

impl Symbol {
    pub(crate) const VALUES: [Self; 53] = [
        Self::Ampersand,
        Self::AmpersandEqual,
        Self::Asterisk,
        Self::AsteriskEqual,
        Self::Bang,
        Self::BangDoubleEqual,
        Self::BangEqual,
        Self::Caret,
        Self::CaretEqual,
        Self::CloseBrace,
        Self::CloseBracket,
        Self::CloseParen,
        Self::Colon,
        Self::Comma,
        Self::Dot,
        Self::DoubleAmpersand,
        Self::DoubleAsterisk,
        Self::DoubleAsteriskEqual,
        Self::DoubleEqual,
        Self::DoubleLessThan,
        Self::DoubleLessThanEqual,
        Self::DoubleMoreThan,
        Self::DoubleMoreThanEqual,
        Self::DoubleMinus,
        Self::DoublePipe,
        Self::DoublePlus,
        Self::DoubleQuestion,
        Self::Equal,
        Self::EqualMoreThan,
        Self::LessThan,
        Self::LessThanEqual,
        Self::Minus,
        Self::MinusEqual,
        Self::MoreThan,
        Self::MoreThanEqual,
        Self::OpenBrace,
        Self::OpenBracket,
        Self::OpenParen,
        Self::Percent,
        Self::PercentEqual,
        Self::Pipe,
        Self::PipeEqual,
        Self::Plus,
        Self::PlusEqual,
        Self::Question,
        Self::Semicolon,
        Self::Slash,
        Self::SlashEqual,
        Self::Tilde,
        Self::TripleDot,
        Self::TripleEqual,
        Self::TripleMoreThan,
        Self::TripleMoreThanEqual,
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

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

impl FromStr for Symbol {
    type Err = BadSymbolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for symbol in Self::VALUES {
            if symbol.to_str() == s {
                return Ok(symbol);
            }
        }
        Err(BadSymbolError)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LexicalError {
    BadNumericLiteral(ParseIntError),
    UnclosedStringLiteral,
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::BadNumericLiteral(..) => "bad numeric literal",
            Self::UnclosedStringLiteral => "unclosed string literal",
        })
    }
}

impl std::error::Error for LexicalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BadNumericLiteral(source) => Some(source),
            Self::UnclosedStringLiteral => None,
        }
    }
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
pub struct BadSymbolError;

impl fmt::Display for BadSymbolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("bad symbol")
    }
}

impl std::error::Error for BadSymbolError {}
