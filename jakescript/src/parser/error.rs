use crate::ast::Script;
use crate::lexer;
use crate::token::{Element, Keyword, Punctuator};
use ansi_term::Style;
use std::borrow::Cow;
use std::fmt;

pub type Result<T = Script> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(ErrorKind);

impl Error {
    pub fn lexical(source: lexer::Error) -> Self {
        Self(ErrorKind::Lexical(source))
    }

    pub fn unexpected(expected: impl Into<Expected>, actual: impl Into<Actual>) -> Self {
        Self(ErrorKind::Parser(expected.into(), actual.into()))
    }

    pub fn unexpected_token(expected: impl Into<Expected>, actual: Element) -> Self {
        Self(ErrorKind::Parser(expected.into(), Actual::Element(actual)))
    }

    pub fn unexpected_eoi(expected: impl Into<Expected>) -> Self {
        Self(ErrorKind::Parser(expected.into(), Actual::EndOfInput))
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.kind().source()
    }
}

impl From<lexer::Error> for Error {
    fn from(source: lexer::Error) -> Self {
        Self::lexical(source)
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Lexical(lexer::Error),
    Parser(Expected, Actual),
}

impl ErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ErrorKind::Lexical(source) => Some(source),
            ErrorKind::Parser(..) => None,
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lexical(source) => write!(f, "lexical error: {}", source),
            Self::Parser(expected, Actual::Element(actual)) => write!(
                f,
                "expected {} at {} but was {}",
                expected,
                actual.source_location(),
                actual
            ),
            Self::Parser(expected, Actual::EndOfInput) => {
                write!(f, "expected {} but reached end of input", expected)
            }
        }
    }
}

#[derive(Debug)]
pub enum Actual {
    Element(Element),
    EndOfInput,
}

impl From<Element> for Actual {
    fn from(actual: Element) -> Self {
        Self::Element(actual)
    }
}

impl From<Option<Element>> for Actual {
    fn from(actual: Option<Element>) -> Self {
        match actual {
            Some(actual) => Self::Element(actual),
            None => Self::EndOfInput,
        }
    }
}

impl fmt::Display for Actual {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Element(actual) => write!(f, "{}", highlight(actual.to_string())),
            Self::EndOfInput => write!(f, "{}", emphasis("end of input")),
        }
    }
}

#[derive(Debug)]
pub enum Expected {
    AnyExpression,
    AnyStatement,

    Identifier(&'static str),
    Keyword(Keyword),
    Literal,
    Punctuator(Punctuator),

    Or2(Box<[Expected; 2]>),
    Or3(Box<[Expected; 3]>),
    Or4(Box<[Expected; 4]>),
}

impl From<Keyword> for Expected {
    fn from(expected: Keyword) -> Self {
        Self::Keyword(expected)
    }
}

impl From<Punctuator> for Expected {
    fn from(expected: Punctuator) -> Self {
        Self::Punctuator(expected)
    }
}

macro_rules! expected_from_n {
    ($variant:ident($($argument:ident: $generic_param:ident, )*)) => {
        impl<$($generic_param, )*> From<($($generic_param, )*)> for Expected
        where
            $($generic_param: Into<Expected>, )*
        {
            fn from(($($argument, )*): ($($generic_param, )*)) -> Self {
                Self::$variant(Box::new([$($argument.into(), )*]))
            }
        }
    };
}
expected_from_n!(Or2(a: A, b: B,));
expected_from_n!(Or3(a: A, b: B, c: C,));
expected_from_n!(Or4(a: A, b: B, c: C, d: D,));

impl fmt::Display for Expected {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AnyExpression => f.write_str("any expression"),
            Self::AnyStatement => f.write_str("any statement"),

            Self::Identifier(placeholder) => write!(f, "{} identifier", emphasis(*placeholder)),
            Self::Keyword(expected) => write!(f, "{}", highlight(expected.as_str())),
            Self::Literal => f.write_str("literal expression"),
            Self::Punctuator(expected) => write!(f, "{}", highlight(expected.as_str())),

            Self::Or2(box [a, b]) => write!(f, "{a} or {b}"),
            Self::Or3(box [a, b, c]) => write!(f, "{a} or {b} or {c}"),
            Self::Or4(box [a, b, c, d]) => write!(f, "{a} or {b} or {c} or {d}"),
        }
    }
}

fn emphasis<'a>(input: impl Into<Cow<'a, str>>) -> ansi_term::ANSIGenericString<'a, str> {
    Style::new().italic().paint(input)
}

fn highlight<'a>(input: impl Into<Cow<'a, str>>) -> ansi_term::ANSIGenericString<'a, str> {
    Style::new().bold().paint(input)
}
