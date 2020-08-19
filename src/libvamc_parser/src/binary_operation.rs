use vamc_errors::Diagnostic;
use vamc_lexer::definitions::TokenKind;

use crate::definitions::{
    ast::{BinaryOperation, BinaryOperationKind},
    Parser, ParserResult,
};

impl Parser {
    pub fn parse_binary_operation(&mut self) -> ParserResult<BinaryOperation> {
        let token = self.token();

        match token.kind {
            TokenKind::PlusSign => {
                // Consume the operator.
                self.bump_until_next();

                let left = self
                    .parse_expression()
                    .expect("Failed to parse left side of binary operation.");

                // Get to the next expression.
                self.bump_while_whitespace();

                let right = self
                    .parse_expression()
                    .expect("Failed to parse right side of binary operation.");

                Ok(BinaryOperation {
                    kind: BinaryOperationKind::Addition,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            },
            _ => Err(Diagnostic::error(
                "Failed to parse binary operation.".into(),
            )),
        }
    }
}
