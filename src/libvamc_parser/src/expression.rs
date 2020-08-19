use vamc_errors::Diagnostic;
use vamc_lexer::definitions::TokenKind;

use crate::{
    definitions::{
        ast::{Expression, ExpressionKind},
        Parser, ParserResult,
    },
    util::is_binary_operator,
};

impl Parser {
    pub fn parse_expression(&mut self) -> ParserResult<Expression> {
        let token = self.token();

        match token.kind {
            TokenKind::Literal(_) => {
                let literal = self
                    .parse_literal()
                    .expect("Failed to parse literal expression.");

                Ok(Expression {
                    kind: ExpressionKind::Literal(literal),
                })
            },
            TokenKind::OpeningBrace => {
                let block = self
                    .parse_block()
                    .expect("Failed to parse block expression.");

                Ok(Expression {
                    kind: ExpressionKind::Block(Box::new(block)),
                })
            },
            TokenKind::Identifier => self.parse_variable_reference(),

            _ if is_binary_operator(token) => {
                let binary_operation = self
                    .parse_binary_operation()
                    .expect("Failed to parse binary operation.");

                Ok(Expression {
                    kind: ExpressionKind::BinaryOperation(binary_operation),
                })
            },

            _ => Err(Diagnostic::error("Failed to parse expression.".into())),
        }
    }
}
