use vamc_errors::Diagnostic;

use crate::definitions::ast::Typ;
use crate::definitions::{Parser, ParserResult};

impl Parser {
    pub fn parse_typ(&mut self) -> ParserResult<Typ> {
        let token = self.token();

        match token.kind {
            _ => Err(Diagnostic::error("Expected a type.".into())),
        }
    }
}
