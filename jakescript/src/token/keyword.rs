use std::fmt;
use std::str::FromStr;

// TODO: Some variants should only be _contextually_ disallowed as identifiers, i.e. in certain
//  circumstances they should be allowed as identifiers.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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
    InstanceOf,
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
    TypeOf,
    Var,
    Void,
    While,
    With,
    Yield,
}

impl Keyword {
    const ALL: &'static [Self] = &[
        Self::As,
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
        Self::InstanceOf,
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
        Self::TypeOf,
        Self::Var,
        Self::Void,
        Self::While,
        Self::With,
        Self::Yield,
    ];

    pub fn all() -> &'static [Self] {
        Self::ALL
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::As => "as",
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
            Self::In => "in",
            Self::InstanceOf => "instanceof",
            Self::Interface => "interface",
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
            Self::TypeOf => "typeof",
            Self::Var => "var",
            Self::Void => "void",
            Self::While => "while",
            Self::With => "with",
            Self::Yield => "yield",
        }
    }
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::all()
            .iter()
            .find(|value| value.as_str() == s)
            .copied()
            .ok_or(())
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
