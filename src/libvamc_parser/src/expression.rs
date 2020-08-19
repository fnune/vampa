use vamc_errors::Diagnostic;
use vamc_lexer::definitions::{LiteralKind, TokenKind};

use crate::definitions::ast::{Expression, ExpressionKind};
use crate::definitions::{Parser, ParserResult};

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
            }
            TokenKind::OpeningBrace => {
                let block = self
                    .parse_block()
                    .expect("Failed to parse block expression.");

                Ok(Expression {
                    kind: ExpressionKind::Block(Box::new(block)),
                })
            }
            _ => Err(Diagnostic::error("Failed to parse expression.".into())),
        }
    }
}
