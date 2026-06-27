#[cfg(test)]
mod test;

pub mod cursor;
pub mod definitions;

mod util;

use crate::{cursor::Cursor, definitions::*, util::*};
use std::fmt::{Display, Formatter};
use vamc_span::Span;

impl Token {
    pub fn new<S: Into<String>>(kind: TokenKind, value: S) -> Token {
        Token {
            kind,
            value: value.into(),
            span: Span::new(0, 0),
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.kind == other.kind && self.value == other.value
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "[{:?}] `{}`", self.kind, self.value)
    }
}

impl Cursor<'_> {
    fn advance(&mut self) -> Token {
        let start = self.offset();
        let character = self.bump().unwrap();

        let mut token = match character {
            character if is_identifier_start(character) => self.token_identifier(character),
            character if is_whitespace(character) => self.token_whitespace(),
            character if is_numeric_literal(character) => self.token_numeric_literal(character),

            '#' => match self.peek() {
                '[' => self.token_block_comment(),
                _ => self.token_inline_comment(),
            },

            ':' => Token::new(TokenKind::Colon, ":"),
            ';' => Token::new(TokenKind::Semicolon, ";"),
            '=' => Token::new(TokenKind::EqualitySign, "="),
            '+' => Token::new(TokenKind::PlusSign, "+"),
            '-' => Token::new(TokenKind::MinusSign, "-"),
            '/' => Token::new(TokenKind::Slash, "/"),
            '%' => Token::new(TokenKind::PercentSign, "%"),
            '*' => Token::new(TokenKind::Star, "*"),
            '{' => Token::new(TokenKind::OpeningBrace, "{"),
            '}' => Token::new(TokenKind::ClosingBrace, "}"),
            _ => Token::new(TokenKind::Unknown, ""),
        };

        token.span = Span::new(start, self.offset());
        token
    }

    fn token_identifier(&mut self, character: char) -> Token {
        let mut value = String::new();
        value.push(character);
        value.push_str(self.bump_while(is_identifier_continuation).as_ref());
        Token::new(TokenKind::Identifier, value)
    }

    fn token_whitespace(&mut self) -> Token {
        self.bump_while(is_whitespace);
        Token::new(TokenKind::Whitespace, " ")
    }

    /// Currently only works for integers.
    fn token_numeric_literal(&mut self, character: char) -> Token {
        let kind = TokenKind::Literal(LiteralKind::Int(Base::Decimal));
        let mut value = String::new();
        value.push(character);
        value.push_str(self.bump_while(is_numeric_literal).as_ref());
        Token::new(kind, value)
    }

    fn token_block_comment(&mut self) -> Token {
        self.bump(); // Bump the `[` character.
        let mut depth: usize = 1;
        let mut value = String::new();

        while let Some(character) = self.bump() {
            match character {
                '#' => {
                    value.push('#');
                    match self.peek() {
                        '[' => {
                            depth += 1;
                            value.push('[');
                            self.bump();
                        },
                        ']' => {
                            depth -= 1;
                            value.push(']');
                            self.bump();
                            if depth == 0 {
                                break;
                            };
                        },
                        _ => {},
                    }
                },
                character => {
                    value.push(character);
                },
            }
        }

        value.pop(); // Remove the dangling `]`.
        value.pop(); // Remove the dangling `#`.

        Token::new(
            TokenKind::BlockComment {
                terminated: depth == 0,
            },
            value,
        )
    }

    fn token_inline_comment(&mut self) -> Token {
        let value = self.bump_while(|character| character != '\n');
        Token::new(TokenKind::InlineComment, value)
    }
}

impl Iterator for Cursor<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        match self.is_eof() {
            false => Some(self.advance()),
            true => None,
        }
    }
}
