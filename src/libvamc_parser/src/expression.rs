use vamc_errors::Diagnostic;
use vamc_lexer::definitions::TokenKind;

use crate::{
    definitions::{
        ast::{Expression, ExpressionKind},
        Parser, ParserResult,
    },
    util::{is_binary_operator, is_keyword_apply},
};

impl Parser {
    pub fn parse_expression(&mut self) -> ParserResult<Expression> {
        let token = self.token();

        match token.kind {
            TokenKind::Literal(_) => {
                let literal = self.parse_literal().expect(
                    format!("Failed to parse literal expression {}.", self.token()).as_str(),
                );

                Ok(Expression {
                    kind: ExpressionKind::Literal(literal),
                })
            }
            TokenKind::OpeningBrace => {
                let block = self
                    .parse_block()
                    .expect(format!("Failed to parse block expression {}.", self.token()).as_str());

                Ok(Expression {
                    kind: ExpressionKind::Block(Box::new(block)),
                })
            }

            _ if is_binary_operator(token) => {
                let binary_operation = self
                    .parse_binary_operation()
                    .expect(format!("Failed to parse binary operation {}.", self.token()).as_str());

                Ok(Expression {
                    kind: ExpressionKind::BinaryOperation(binary_operation),
                })
            }

            _ if is_keyword_apply(token) => {
                let function_call = self
                    .parse_function_call()
                    .expect(format!("Failed to parse function call {}.", self.token()).as_str());

                Ok(function_call)
            }

            // Must go after the check for the `apply` keyword.
            TokenKind::Identifier => self.parse_variable_reference(),

            _ => Err(Diagnostic::error(format!(
                "Failed to parse expression {}.",
                self.token()
            ))),
        }
    }
}
