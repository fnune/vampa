use vamc_errors::Diagnostic;
use vamc_lexer::definitions::TokenKind;

use crate::definitions::{
    ast::{Block, Statement},
    Parser, ParserResult,
};

impl Parser {
    pub fn parse_block(&mut self) -> ParserResult<Block> {
        let token = self.token();

        match token.kind {
            TokenKind::OpeningBrace => {
                // Eat the opening brace.
                self.bump_until_next();

                let statements = self.parse_block_statements();
                let result = self.expect_closing_brace(Block { statements });

                // Eat the closing brace.
                self.bump_until_next();

                result
            }
            _ => Err(Diagnostic::error(format!(
                "Failed to parse statement {}.",
                self.token()
            ))),
        }
    }

    pub fn parse_block_statements(&mut self) -> Vec<Box<Statement>> {
        let mut statements: Vec<Box<Statement>> = Vec::new();

        while self.token().kind != TokenKind::ClosingBrace && !self.is_done() {
            statements.push(Box::new(self.parse_statement().expect(
                format!("Failed to parse statement in block {}.", self.token()).as_str(),
            )));
        }

        statements
    }
}
