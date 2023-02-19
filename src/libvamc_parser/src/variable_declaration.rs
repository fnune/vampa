use vamc_errors::Diagnostic;
use vamc_lexer::definitions::TokenKind;

use crate::{
    definitions::{ast::*, Parser, ParserResult},
    util::*,
};

impl Parser {
    pub fn parse_variable_declaration(&mut self) -> ParserResult<VariableDeclaration> {
        let token = self.token();

        match token.kind {
            _ if is_keyword_let(token) => {
                let token = self.bump_until_next();
                match token.kind {
                    TokenKind::Identifier => {
                        let name = token.value.clone();

                        let token = self.bump_until_next();
                        match token.kind {
                            // A variable whose type must be inferred.
                            TokenKind::EqualitySign => {
                                self.bump_until_next();

                                let value = self.parse_expression().unwrap_or_else(|_| {
                                    panic!("Failed to parse expression {}.", self.token())
                                });

                                let result = self.expect_semicolon(VariableDeclaration {
                                    name: Box::new(name),
                                    typ: Box::new(Typ::infer()),
                                    value: Box::new(value),
                                });

                                // Eat the semicolon.
                                self.bump_until_next();

                                result
                            },
                            // A type for this variable.
                            TokenKind::Colon => {
                                self.bump_until_next();

                                let typ = self.parse_typ().unwrap_or_else(|_| {
                                    panic!("Failed to parse type {}.", self.token())
                                });

                                let token = self.bump_until_next();
                                match token.kind {
                                    TokenKind::EqualitySign => {
                                        self.bump_until_next();

                                        let value = self.parse_expression().unwrap_or_else(|_| {
                                            panic!("Failed to parse expression {}.", self.token())
                                        });

                                        let result = self.expect_semicolon(VariableDeclaration {
                                            name: Box::new(name),
                                            typ: Box::new(typ),
                                            value: Box::new(value),
                                        });

                                        // Eat the semicolon.
                                        self.bump_until_next();

                                        result
                                    },
                                    _ => Err(Diagnostic::error(
                                        "Expected `=` after variable declaration type hint.".into(),
                                    )),
                                }
                            },
                            _ => Err(Diagnostic::error(
                                "Expected `=` or `:` after variable declaration identifier.".into(),
                            )),
                        }
                    },
                    _ => Err(Diagnostic::error(
                        "Expected identifier after `let` keyword.".into(),
                    )),
                }
            },
            _ => Err(Diagnostic::error("Expected `let` keyword.".into())),
        }
    }
}
