use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    Literal(Literal),
    Symbol(Symbol),

    Invalid(LexError),
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Keyword {
    Let,
    While,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Let => "let",
            Self::While => "while",
        })
    }
}

impl FromStr for Keyword {
    type Err = BadKeywordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "let" => Self::Let,
            "while" => Self::While,
            _ => return Err(BadKeywordError),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Literal {
    Character(char),
    Integer(u64),
    String(String),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Character(it) => write!(f, r#"'{}'"#, it),
            Self::Integer(it) => write!(f, r#"{}"#, it),
            Self::String(it) => write!(f, r#""{}""#, it),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Symbol {
    CloseBrace,
    CloseBracket,
    CloseParen,
    Equal,
    LessThan,
    MoreThan,
    OpenBrace,
    OpenBracket,
    OpenParen,
    Plus,
    Semicolon,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::CloseBrace => "}",
            Self::CloseBracket => "]",
            Self::CloseParen => ")",
            Self::Equal => "=",
            Self::LessThan => "<",
            Self::MoreThan => ">",
            Self::OpenBrace => "{",
            Self::OpenBracket => "[",
            Self::OpenParen => "(",
            Self::Plus => "+",
            Self::Semicolon => ";",
        })
    }
}

impl FromStr for Symbol {
    type Err = BadSymbolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "}" => Self::CloseBrace,
            "]" => Self::CloseBracket,
            ")" => Self::CloseParen,
            "=" => Self::Equal,
            "<" => Self::LessThan,
            ">" => Self::MoreThan,
            "{" => Self::OpenBrace,
            "[" => Self::OpenBracket,
            "(" => Self::OpenParen,
            "+" => Self::Plus,
            ";" => Self::Semicolon,
            _ => return Err(BadSymbolError),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LexError {
    BadNumericLiteral(ParseIntError),
    NotSingleCharacter,
    UnclosedCharacterLiteral,
    UnclosedStringLiteral,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::BadNumericLiteral(..) => "bad numeric literal",
            Self::NotSingleCharacter => "not a single character",
            Self::UnclosedCharacterLiteral => "unclosed character literal",
            Self::UnclosedStringLiteral => "unclosed string literal",
        })
    }
}

impl std::error::Error for LexError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BadNumericLiteral(source) => Some(source),
            Self::NotSingleCharacter
            | Self::UnclosedCharacterLiteral
            | Self::UnclosedStringLiteral => None,
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
