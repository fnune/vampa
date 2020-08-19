use vamc_errors::Diagnostic;
use vamc_lexer::definitions::TokenKind;

use crate::definitions::ast::*;
use crate::definitions::{Parser, ParserResult};
use crate::util::*;

impl Parser {
    pub fn parse_function_declaration(&mut self) -> ParserResult<FunctionDeclaration> {
        let token = self.token();

        match token.kind {
            _ if is_keyword_fun(token) => {
                let token = self.bump_until_next();
                match token.kind {
                    TokenKind::Identifier => {
                        let name = Box::new(token.value.clone());

                        self.bump_until_next();
                        let parameters = self
                            .parse_parameters()
                            .expect("Failed to parse function parameters.");

                        let return_typ = if is_keyword_returning(self.token()) {
                            Box::new(
                                self.parse_typ()
                                    .expect("Failed to parse function return type."),
                            )
                        } else {
                            Box::new(Typ::infer())
                        };

                        self.bump_until_next();

                        match self.token().kind {
                            TokenKind::EqualitySign => {
                                // Eat the `=`.
                                self.bump_until_next();

                                let body = Box::new(
                                    self.parse_expression()
                                        .expect("Failed to parse function body."),
                                );

                                self.expect_semicolon(FunctionDeclaration {
                                    name,
                                    parameters,
                                    return_typ,
                                    body,
                                })
                            }
                            _ => Err(Diagnostic::error(
                                "Expected `=` after function signature declaration.".into(),
                            )),
                        }
                    }
                    _ => Err(Diagnostic::error(
                        "Expected identifier after `fun` keyword.".into(),
                    )),
                }
            }
            _ => Err(Diagnostic::error("Expected `fun` keyword.".into())),
        }
    }
}
