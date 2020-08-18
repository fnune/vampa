use vamc_errors::Diagnostic;
use vamc_lexer::definitions::{LiteralKind, TokenKind};

use crate::definitions::ast::{Expression, ExpressionKind};
use crate::definitions::{Parser, ParserResult};

impl Parser {
    pub fn parse_expression(&mut self) -> ParserResult<Expression> {
        let token = self.token();

        match token.kind {
            TokenKind::Literal(LiteralKind::Unknown) => {
                let literal = self
                    .parse_literal()
                    .expect("Failed to parse literal expression.");

                Ok(Expression {
                    kind: ExpressionKind::Literal(literal),
                })
            }
            _ => Err(Diagnostic::error("Failed to parse expression.".into())),
        }
    }
}
