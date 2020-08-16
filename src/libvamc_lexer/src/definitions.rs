/// Base of the numeric literal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
    /// No prefix.
    Decimal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
    /// "12"
    Int { base: Base },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    /// An inline comment:
    /// "# This is an inline comment"
    InlineComment,
    /// A block comment:
    /// "#[ This is a block comment #]"
    /// Nested comments are terminated if they all have balanced opening "#[" and closing "#]" signs.
    BlockComment { terminated: bool },
    /// "12"
    Literal { kind: LiteralKind },
    /// A keyword or a user-defined identifier.
    Identifier,
    /// ":"
    Colon,
    /// ";"
    Semicolon,
    /// Any sequence of whitespace characters.
    Whitespace,
    /// "="
    EqualitySign,
    /// "+"
    PlusSign,
    /// "-"
    MinusSign,
    /// "/"
    Slash,
    /// "%"
    PercentSign,
    /// "*"
    Star,
    /// "{"
    OpeningBrace,
    /// "}"
    ClosingBrace,
    /// A token not expected by the lexer.
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}
