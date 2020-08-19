pub fn is_identifier_start(character: char) -> bool {
    unicode_xid::UnicodeXID::is_xid_start(character)
}

pub fn is_identifier_continuation(character: char) -> bool {
    unicode_xid::UnicodeXID::is_xid_continue(character)
}

// https://github.com/rust-lang/rust/blob/20c50444650f90f266ab2a46afd5089b4c01a28c/src/librustc_lexer/src/lib.rs#L222
pub fn is_whitespace(character: char) -> bool {
    match character {
    // Usual ASCII suspects
    | '\u{0009}' // \t
    | '\u{000A}' // \n
    | '\u{000B}' // vertical tab
    | '\u{000C}' // form feed
    | '\u{000D}' // \r
    | '\u{0020}' // space

    // NEXT LINE from latin1
    | '\u{0085}'

    // Bidi markers
    | '\u{200E}' // LEFT-TO-RIGHT MARK
    | '\u{200F}' // RIGHT-TO-LEFT MARK

    // Dedicated whitespace characters from Unicode
    | '\u{2028}' // LINE SEPARATOR
    | '\u{2029}' // PARAGRAPH SEPARATOR
    => true,
    _ => false,
  }
}

pub fn is_numeric_literal(character: char) -> bool { ('0'..='9').contains(&character) }
