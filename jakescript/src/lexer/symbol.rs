/// NULL
pub const NUL: char = '\u{0000}';
/// BACKSPACE
pub const BS: char = '\u{0008}';
/// CHARACTER TABULATION
pub const HT: char = '\u{0009}';
/// LINE FEED (LF)
pub const LF: char = '\u{000A}';
/// LINE TABULATION
pub const VT: char = '\u{000B}';
/// FORM FEED (FF)
pub const FF: char = '\u{000C}';
/// CARRIAGE RETURN (CR)
pub const CR: char = '\u{000D}';
/// SPACE
pub const SP: char = '\u{0020}';
/// NO-BREAK SPACE
pub const NBSP: char = '\u{00A0}';
/// ZERO WIDTH NON-JOINER
pub const ZWNJ: char = '\u{200C}';
/// ZERO WIDTH JOINER
pub const ZWJ: char = '\u{200D}';
/// LINE SEPARATOR
pub const LS: char = '\u{2028}';
/// PARAGRAPH SEPARATOR
pub const PS: char = '\u{2029}';
/// ZERO WIDTH NO-BREAK SPACE
pub const ZWNBSP: char = '\u{FEFF}';

pub fn is_whitespace(ch: char) -> bool {
    // FIXME: Return `true` for USP (any other code point classified in the "Space_Separator"
    //  category, which is not the same as the Unicode "White_Space" property).
    matches!(ch, HT | VT | FF | SP | NBSP | ZWNBSP)
}

pub fn is_line_terminator(ch: char) -> bool {
    matches!(ch, LF | CR | LS | PS)
}

pub fn is_identifier_start(ch: char) -> bool {
    // TODO: Handle Unicode escape sequence.
    is_unicode_start(ch) || matches!(ch, '$' | '_')
}

pub fn is_identifier_part(ch: char) -> bool {
    // TODO: Handle Unicode escape sequence.
    is_unicode_continue(ch) || matches!(ch, '$' | ZWNJ | ZWJ)
}

pub fn is_unicode_start(ch: char) -> bool {
    // FIXME: Check for characters with the Unicode "ID_Start" property.
    ch.is_ascii_alphabetic()
}

pub fn is_unicode_continue(ch: char) -> bool {
    // FIXME: Check for characters with the Unicode "ID_Continue" property.
    ch.is_ascii_alphabetic() || ch.is_ascii_digit() || ch == '_'
}

pub fn into_escaped(ch: char) -> char {
    match ch {
        '0' => NUL,
        'b' => BS,
        't' => HT,
        'n' => LF,
        'v' => VT,
        'f' => FF,
        'r' => CR,
        ch => ch,
    }
}
