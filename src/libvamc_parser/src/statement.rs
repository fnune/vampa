use vamc_errors::Diagnostic;

use crate::definitions::ast::{Statement, StatementKind};
use crate::definitions::{Parser, ParserResult};
use crate::util::{is_keyword_fun, is_keyword_let};

impl Parser {
    pub fn parse_statement(&mut self) -> ParserResult<Statement> {
        let token = self.token();

        match token {
            token if is_keyword_let(token) => {
                let declaration = self
                    .parse_variable_declaration()
                    .expect("Failed to parse variable declaration.");

                Ok(Statement {
                    kind: StatementKind::VariableDeclaration(declaration),
                })
            }
            token if is_keyword_fun(token) => {
                unimplemented!();
            }
            _ => Err(Diagnostic::error("Failed to parse statement.".into())),
        }
    }
}
