use enumerate::{Enumerate, EnumerateStr};

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
