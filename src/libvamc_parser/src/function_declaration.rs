use vamc_errors::Diagnostic;
use vamc_lexer::definitions::TokenKind;

use crate::{
    definitions::{ast::*, Parser, ParserResult},
    util::*,
};

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
                        let parameters = self.parse_parameters().expect(
                            format!("Failed to parse function parameters {}.", self.token())
                                .as_str(),
                        );

                        let return_typ = if is_keyword_returning(self.token()) {
                            // Eat the `returning` keyword.
                            self.bump_until_next();
                            let return_typ = Box::new(
                                self.parse_typ().expect(
                                    format!(
                                        "Failed to parse function return type {}.",
                                        self.token()
                                    )
                                    .as_str(),
                                ),
                            );
                            self.bump_until_next();
                            return_typ
                        } else {
                            Box::new(Typ::infer())
                        };

                        match self.token().kind {
                            TokenKind::EqualitySign => {
                                // Eat the `=`.
                                self.bump_until_next();

                                let body = Box::new(
                                    self.parse_expression().expect(
                                        format!("Failed to parse function body {}.", self.token())
                                            .as_str(),
                                    ),
                                );

                                let result = self.expect_semicolon(FunctionDeclaration {
                                    name,
                                    parameters,
                                    return_typ,
                                    body,
                                });

                                // Eat the semicolon.
                                self.bump_until_next();

                                result
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
