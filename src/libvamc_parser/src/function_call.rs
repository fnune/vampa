use vamc_errors::Diagnostic;
use vamc_lexer::definitions::TokenKind;

use crate::{
    definitions::{
        ast::{Expression, ExpressionKind},
        Parser, ParserResult,
    },
    util::{is_keyword_apply, is_parameters_list_termination},
};

impl Parser {
    pub fn parse_function_call(&mut self) -> ParserResult<Expression> {
        let token = self.token();

        match token.kind {
            _ if is_keyword_apply(token) => {
                let token = self.bump_until_next();

                match token.kind {
                    TokenKind::Identifier => {
                        let name = Box::new(token.value.clone());
                        let mut parameters: Vec<Box<Expression>> = Vec::default();

                        // Eat the identifier.
                        self.bump_until_next();

                        while !is_parameters_list_termination(self.token()) && !self.is_done() {
                            let parameter = self
                                .parse_expression()
                                .expect("Failed to parse parameter expression in function call.");

                            parameters.push(Box::new(parameter));

                            self.bump_while_whitespace();
                        }

                        Ok(Expression {
                            kind: ExpressionKind::FunctionCall(name, parameters),
                        })
                    },
                    _ => Err(Diagnostic::error(
                        "Expected identifier after `apply` keyword.".into(),
                    )),
                }
            },
            _ => Err(Diagnostic::error("Failed `apply` keyword.".into())),
        }
    }
}
