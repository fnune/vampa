mod definitions;
mod util;

mod expression;
mod literal;
mod statement;
mod typ;
mod variable_declaration;

use vamc_errors::Diagnostic;
use vamc_lexer::definitions::{Token, TokenKind};

use crate::definitions::*;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, index: 0 }
    }

    pub fn is_done(&self) -> bool {
        self.index == self.tokens.len() - 1
    }

    pub fn token(&self) -> &Token {
        &self.tokens[self.index]
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index + 1)
    }

    pub fn bump(&mut self) -> &Token {
        self.index += 1;
        &self.tokens[self.index]
    }

    pub fn bump_while<F>(&mut self, predicate: F)
    where
        F: Fn(&Token) -> bool,
    {
        while !self.is_done() && predicate(self.peek().unwrap()) {
            self.bump();
        }
    }

    pub fn bump_while_whitespace(&mut self) {
        self.bump_while(|token| token.kind == TokenKind::Whitespace);
    }

    pub fn bump_until_next(&mut self) -> &Token {
        self.bump();
        self.bump_while_whitespace();
        &self.tokens[self.index]
    }

    pub fn expect_semicolon<T>(&mut self, ok: T) -> ParserResult<T> {
        let token = self.bump_until_next();
        match token.kind {
            TokenKind::Semicolon => Ok(ok),
            _ => Err(Diagnostic::error("Expected `;`.".into())),
        }
    }
}
