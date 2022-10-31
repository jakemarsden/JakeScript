use crate::simple_enumeration;

simple_enumeration!(pub Punctuator {
    Amp => "&",
    AmpAmp => "&&",
    AmpEq => "&=",
    Bang => "!",
    BangEq => "!=",
    BangEqEq => "!==",
    Caret => "^",
    CaretEq => "^=",
    CloseBrace => "}",
    CloseBracket => "]",
    CloseParen => ")",
    Colon => ":",
    Comma => ",",
    Dot => ".",
    DotDotDot => "...",
    Eq => "=",
    EqEq => "==",
    EqEqEq => "===",
    EqGt => "=>",
    Gt => ">",
    GtEq => ">=",
    GtGt => ">>",
    GtGtEq => ">>=",
    GtGtGt => ">>>",
    GtGtGtEq => ">>>=",
    Lt => "<",
    LtEq => "<=",
    LtLt => "<<",
    LtLtEq => "<<=",
    Minus => "-",
    MinusEq => "-=",
    MinusMinus => "--",
    OpenBrace => "{",
    OpenBracket => "[",
    OpenParen => "(",
    Percent => "%",
    PercentEq => "%=",
    Pipe => "|",
    PipeEq => "|=",
    PipePipe => "||",
    Plus => "+",
    PlusEq => "+=",
    PlusPlus => "++",
    Question => "?",
    QuestionQuestion => "??",
    Semi => ";",
    Slash => "/",
    SlashEq => "/=",
    Star => "*",
    StarEq => "*=",
    StarStar => "**",
    StarStarEq => "**=",
    Tilde => "~",
});

impl Punctuator {
    /// Unlike for [`Self::all()`], **order is important**. For multiple
    /// punctuators which start with the same substring, the longest needs
    /// to come first. This is relied on by the `Lexer`.
    ///
    /// TODO: Sort at compile-time and return a `&'static [Self]`, presumably by
    /// writing a proc  macro.
    pub fn all_in_lexical_order() -> Vec<Self> {
        let mut values = Self::all().to_vec();
        values.sort_by_key(|value| usize::MAX - value.as_str().len());
        values
    }
}
