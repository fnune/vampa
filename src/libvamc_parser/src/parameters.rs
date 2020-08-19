use vamc_errors::Diagnostic;
use vamc_lexer::definitions::TokenKind;

use crate::definitions::ast::{Parameter, Parameters};
use crate::definitions::{Parser, ParserResult};
use crate::util::{is_keyword_and, is_keyword_of, is_keyword_returning};

impl Parser {
    pub fn parse_parameter(&mut self) -> ParserResult<Parameter> {
        let token = self.token();

        match token.kind {
            TokenKind::Identifier => {
                let name = token.value.clone();

                let token = self.bump_until_next();
                match token.kind {
                    TokenKind::Colon => {
                        self.bump_until_next();
                        let typ = self.parse_typ().expect("Failed to parse parameter type.");

                        self.bump_until_next();
                        Ok(Parameter {
                            name: Box::new(name),
                            typ: Box::new(typ),
                        })
                    }
                    _ => Err(Diagnostic::error("Expected `:`.".into())),
                }
            }
            _ => Err(Diagnostic::error(
                "Failed to parse function parameter.".into(),
            )),
        }
    }

    pub fn parse_parameters(&mut self) -> ParserResult<Parameters> {
        let token = self.token();

        match token {
            _ if is_keyword_of(token) => {
                self.bump_until_next();

                let mut parameters: Parameters = Vec::new();
                let mut result: ParserResult<Parameters> = Err(Diagnostic::error(
                    "Unexpected error parsing function parameters.".into(),
                ));

                while self.is_parameter_like() {
                    if parameters.len() >= 1 {
                        if is_keyword_and(self.token()) {
                            // Consume the `and` keyword.
                            self.bump_until_next();
                        } else {
                            result = Err(Diagnostic::error(
                                "Expected `and` keyword between function parameters.".into(),
                            ));
                            break;
                        }
                    }

                    parameters.push(Box::new(
                        self.parse_parameter().expect("Failed to parse parameter."),
                    ));
                    result = Ok(Vec::default());
                }

                match result {
                    Err(err) => Err(err),
                    Ok(_) => Ok(parameters),
                }
            }
            _ => Err(Diagnostic::error(
                "Expected `of` keyword to start a list of parameters.".into(),
            )),
        }
    }

    fn is_parameter_like(&self) -> bool {
        let token = self.token();

        match token.kind {
            TokenKind::Identifier => !is_keyword_returning(token),
            _ => false,
        }
    }
}
