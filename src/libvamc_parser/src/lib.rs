#[cfg(test)]
mod tests;

pub mod definitions;

mod util;

mod binary_operation;
mod block;
mod expression;
mod function_call;
mod function_declaration;
mod literal;
mod parameters;
mod source_file;
mod statement;
mod typ;
mod variable_declaration;
mod variable_reference;

use vamc_errors::Diagnostic;
use vamc_lexer::definitions::{Token, TokenKind};

use crate::definitions::*;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let mut tokens = tokens.into_iter().peekable();
        let current = tokens
            .peek()
            .unwrap_or(&Token::new(TokenKind::EOF, ""))
            .clone();

        Parser { tokens, current }
    }

    pub fn is_done(&mut self) -> bool { self.tokens.peek() == None }

    pub fn bump(&mut self) {
        if !self.is_done() {
            self.tokens.next();
            self.current = self
                .tokens
                .peek()
                .unwrap_or(&Token::new(TokenKind::EOF, ""))
                .clone()
        };
    }

    pub fn token(&self) -> &Token { &self.current }

    pub fn bump_while<F>(&mut self, predicate: F)
    where F: Fn(&Token) -> bool {
        while !self.is_done() && predicate(self.tokens.peek().unwrap()) {
            self.bump();
        }
    }

    pub fn bump_while_whitespace(&mut self) {
        self.bump_while(|token| token.kind == TokenKind::Whitespace);
    }

    pub fn bump_until_next(&mut self) -> &Token {
        self.bump();
        self.bump_while_whitespace();
        self.token()
    }

    pub fn expect_token<T, S: Into<String>>(
        &mut self,
        token_kind: TokenKind,
        ok: T,
        err: S,
    ) -> ParserResult<T>
    {
        self.bump_while_whitespace();
        let token = self.token();
        match token.kind {
            kind if kind == token_kind => Ok(ok),
            _ => Err(Diagnostic::error(err.into())),
        }
    }

    pub fn expect_semicolon<T>(&mut self, ok: T) -> ParserResult<T> {
        self.expect_token(TokenKind::Semicolon, ok, "Expected `;`.")
    }

    pub fn expect_closing_brace<T>(&mut self, ok: T) -> ParserResult<T> {
        self.expect_token(TokenKind::ClosingBrace, ok, "Expected `}`.")
    }
}
