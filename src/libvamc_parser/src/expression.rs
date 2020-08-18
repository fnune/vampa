use vamc_errors::Diagnostic;
use vamc_lexer::definitions::TokenKind;

use crate::definitions::ast::Expression;
use crate::definitions::{Parser, ParserResult};
use crate::util::*;

impl Parser {
    pub fn parse_expression(&mut self) -> ParserResult<Expression> {
        let token = self.token();

        unimplemented!();
    }
}
