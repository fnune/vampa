use vamc_errors::Diagnostic;
use vamc_lexer::definitions::TokenKind;

use crate::definitions::ast::{Block, Statement};
use crate::definitions::{Parser, ParserResult};

impl Parser {
    // TODO: implement erroring out if the last statement isn't a return expression.
    pub fn parse_block(&mut self) -> ParserResult<Block> {
        let token = self.token();

        match token.kind {
            TokenKind::OpeningBrace => {
                self.bump_until_next();

                let mut statements: Vec<Box<Statement>> = Vec::new();

                while self.token().kind != TokenKind::ClosingBrace {
                    if let Ok(statement) = self.parse_statement() {
                        statements.push(Box::new(statement));
                    }
                }

                let result = self.expect_closing_brace(Block { statements });

                // Eat the closing brace.
                self.bump_until_next();

                result
            }
            _ => Err(Diagnostic::error("Failed to parse statement.".into())),
        }
    }
}
