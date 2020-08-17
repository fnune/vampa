use vamc_errors::Diagnostic;
use vamc_lexer::definitions::{Base, LiteralKind as TokenLiteralKind, TokenKind};

use crate::definitions::ast::{Literal, LiteralIntType, LiteralKind};
use crate::definitions::{Parser, ParserResult};

impl Parser {
    fn parse_literal(&mut self) -> ParserResult<Literal> {
        let token = self.token();

        match token.kind {
            TokenKind::Literal(TokenLiteralKind::Int(Base::Decimal)) => {
                if let Ok(value) = token.value.parse() {
                    Ok(Literal {
                        kind: LiteralKind::Int(value, LiteralIntType::Unsuffixed),
                    })
                } else {
                    Err(Diagnostic::error(
                        "Failed to parse LiteralIntType::Unsuffixed from token.".into(),
                    ))
                }
            }
            _ => Err(Diagnostic::error("Failed to parse literal.".into())),
        }
    }
}
