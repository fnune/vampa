use vamc_errors::Diagnostic;
use vamc_lexer::definitions::TokenKind;

use crate::{
    definitions::{
        ast::{BinaryOperation, BinaryOperationKind},
        Parser, ParserResult,
    },
    util::is_binary_operator,
};

impl Parser {
    pub fn parse_binary_operation(&mut self) -> ParserResult<BinaryOperation> {
        let token = self.token();

        match token.kind {
            token_kind if is_binary_operator(token) => {
                // Consume the operator.
                self.bump_until_next();

                let left = self.parse_expression().expect(
                    format!(
                        "Failed to parse left side of binary operation {}.",
                        self.token()
                    )
                    .as_str(),
                );

                // Get to the next expression.
                self.bump_while_whitespace();

                let right = self.parse_expression().expect(
                    format!(
                        "Failed to parse right side of binary operation {}.",
                        self.token()
                    )
                    .as_str(),
                );

                Ok(BinaryOperation {
                    kind: BinaryOperationKind::from(token_kind),
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
            _ => Err(Diagnostic::error(format!(
                "Failed to parse binary operation {}.",
                self.token()
            ))),
        }
    }
}

impl From<TokenKind> for BinaryOperationKind {
    fn from(token_kind: TokenKind) -> BinaryOperationKind {
        match token_kind {
            TokenKind::PlusSign => BinaryOperationKind::Addition,
            _ => unimplemented!(),
        }
    }
}
