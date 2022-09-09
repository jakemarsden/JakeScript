pub use keyword::*;
pub use literal::*;
pub use location::*;
pub use punctuator::*;
pub use template::*;

use crate::str::NonEmptyString;
use std::fmt;
use symbol::*;

pub mod symbol;

mod keyword;
mod literal;
mod location;
mod punctuator;
mod template;

#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    kind: ElementKind,
    loc: SourceLocation,
}

impl Element {
    pub fn new_identifier(it: NonEmptyString, loc: SourceLocation) -> Self {
        Self::new_token(Token::Identifier(it), loc)
    }

    pub fn into_identifier(self) -> Option<NonEmptyString> {
        match self.into_token() {
            Some(Token::Identifier(it)) => Some(it),
            _ => None,
        }
    }

    pub fn identifier(&self) -> Option<&NonEmptyString> {
        match self.token() {
            Some(Token::Identifier(it)) => Some(it),
            _ => None,
        }
    }

    pub fn new_keyword(it: Keyword, loc: SourceLocation) -> Self {
        Self::new_token(Token::Keyword(it), loc)
    }

    pub fn keyword(&self) -> Option<Keyword> {
        match self.token() {
            Some(Token::Keyword(it)) => Some(*it),
            _ => None,
        }
    }

    pub fn new_literal(it: Literal, loc: SourceLocation) -> Self {
        Self::new_token(Token::Literal(it), loc)
    }

    pub fn into_literal(self) -> Option<Literal> {
        match self.into_token() {
            Some(Token::Literal(it)) => Some(it),
            _ => None,
        }
    }

    pub fn literal(&self) -> Option<&Literal> {
        match self.token() {
            Some(Token::Literal(it)) => Some(it),
            _ => None,
        }
    }

    pub fn new_punctuator(it: Punctuator, loc: SourceLocation) -> Self {
        Self::new_token(Token::Punctuator(it), loc)
    }

    pub fn punctuator(&self) -> Option<Punctuator> {
        match self.token() {
            Some(Token::Punctuator(it)) => Some(*it),
            _ => None,
        }
    }

    pub fn new_token(it: Token, loc: SourceLocation) -> Self {
        Self::new(ElementKind::Token(it), loc)
    }

    pub fn into_token(self) -> Option<Token> {
        match self.into_kind() {
            ElementKind::Token(it) => Some(it),
            _ => None,
        }
    }

    pub fn token(&self) -> Option<&Token> {
        match self.kind() {
            ElementKind::Token(ref it) => Some(it),
            _ => None,
        }
    }

    pub fn new_comment(it: Comment, loc: SourceLocation) -> Self {
        Self::new(ElementKind::Comment(it), loc)
    }

    pub fn into_comment(self) -> Option<Comment> {
        match self.into_kind() {
            ElementKind::Comment(it) => Some(it),
            _ => None,
        }
    }

    pub fn comment(&self) -> Option<&Comment> {
        match self.kind() {
            ElementKind::Comment(ref it) => Some(it),
            _ => None,
        }
    }

    pub fn new_line_terminator(it: LineTerminator, loc: SourceLocation) -> Self {
        Self::new(ElementKind::LineTerminator(it), loc)
    }

    pub fn into_line_terminator(self) -> Option<LineTerminator> {
        match self.into_kind() {
            ElementKind::LineTerminator(it) => Some(it),
            _ => None,
        }
    }

    pub fn line_terminator(&self) -> Option<&LineTerminator> {
        match self.kind() {
            ElementKind::LineTerminator(ref it) => Some(it),
            _ => None,
        }
    }

    pub fn new_whitespace(it: Whitespace, loc: SourceLocation) -> Self {
        Self::new(ElementKind::Whitespace(it), loc)
    }

    pub fn into_whitespace(self) -> Option<Whitespace> {
        match self.into_kind() {
            ElementKind::Whitespace(it) => Some(it),
            _ => None,
        }
    }

    pub fn whitespace(&self) -> Option<&Whitespace> {
        match self.kind() {
            ElementKind::Whitespace(ref it) => Some(it),
            _ => None,
        }
    }

    fn new(kind: ElementKind, loc: SourceLocation) -> Self {
        Self { kind, loc }
    }

    pub fn into_kind(self) -> ElementKind {
        self.kind
    }

    pub fn kind(&self) -> &ElementKind {
        &self.kind
    }

    pub fn source_location(&self) -> &SourceLocation {
        &self.loc
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind() {
            ElementKind::Token(it) => write!(f, "{}", it),
            ElementKind::Comment(it) => write!(f, "{}", it),
            ElementKind::LineTerminator(it) => write!(f, "{}", it),
            ElementKind::Whitespace(it) => write!(f, "{}", it),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ElementKind {
    Token(Token),
    Comment(Comment),
    LineTerminator(LineTerminator),
    Whitespace(Whitespace),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Identifier(NonEmptyString),
    Keyword(Keyword),
    Literal(Literal),
    Punctuator(Punctuator),
    Template(Template),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Identifier(it) => write!(f, "{}", it),
            Self::Keyword(it) => write!(f, "{}", it),
            Self::Literal(it) => write!(f, "{}", it),
            Self::Punctuator(it) => write!(f, "{}", it),
            Self::Template(it) => write!(f, "{}", it),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Comment {
    pub kind: CommentKind,
    pub value: String,
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            CommentKind::SingleLine => write!(f, "//{}", self.value),
            CommentKind::MultiLine => write!(f, "/*{}*/", self.value),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CommentKind {
    SingleLine,
    MultiLine,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
            Self::Crlf => (CR, Some(LF)),
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Whitespace {
    value: NonEmptyString,
}

impl fmt::Display for Whitespace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.value.as_ref())
    }
}

impl From<NonEmptyString> for Whitespace {
    fn from(s: NonEmptyString) -> Whitespace {
        Self { value: s }
    }
}
