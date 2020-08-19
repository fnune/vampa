use vamc_errors::Diagnostic;
use vamc_lexer::definitions::TokenKind;

use crate::definitions::{
    ast::{IntType, Typ, TypKind},
    Parser, ParserResult,
};

impl Parser {
    pub fn parse_typ(&mut self) -> ParserResult<Typ> {
        let token = self.token();

        match token.kind {
            TokenKind::Identifier => match token.value.as_str() {
                "i32" => Ok(Typ {
                    kind: TypKind::Int(IntType::I32),
                }),
                _ => Err(Diagnostic::error("Unknown type.".into())),
            },
            _ => Err(Diagnostic::error("Expected a type.".into())),
        }
    }
}
