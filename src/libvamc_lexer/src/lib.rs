mod cursor;
mod definitions;
mod util;

use crate::cursor::Cursor;
use crate::definitions::*;
use crate::util::*;

impl Token {
  pub fn new<S: Into<String>>(kind: TokenKind, value: S) -> Token {
    Token { kind, value: value.into() }
  }
}

impl Cursor<'_> {
  fn advance(&mut self) -> Token {
    let character = self.bump().unwrap();

    match character {
      character if is_identifier_start(character) => self.token_identifier(),
      character if is_whitespace(character) => self.token_whitespace(),
      character if is_numeric_literal(character) => self.token_numeric_literal(),

      '#' => match self.peek() {
        '[' => self.token_block_comment(),
        _ => self.token_inline_comment(),
      }

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
      _ => Token::new(TokenKind::Unknown, "")
    }
  }

  fn token_identifier(&mut self) -> Token {
    let value = self.bump_while(is_identifier_continuation);
    Token::new(TokenKind::Identifier, value)
  }

  fn token_whitespace(&mut self) -> Token {
    self.bump_while(is_whitespace);
    Token::new(TokenKind::Whitespace, " ")
  }

  /// Currently only works for integers.
  fn token_numeric_literal(&mut self) -> Token {
    let kind = TokenKind::Literal { kind: LiteralKind::Int { base: Base::Decimal } };
    let value = self.bump_while(is_numeric_literal);
    Token::new(kind, value)
  }

  fn token_block_comment(&self) -> Token {
    unimplemented!()
  }

  fn token_inline_comment(&self) -> Token {
    unimplemented!()
  }
}
