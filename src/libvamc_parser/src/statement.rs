use crate::{
    definitions::{
        ast::{Statement, StatementKind},
        Parser, ParserResult,
    },
    util::{is_keyword_fun, is_keyword_let},
};

impl Parser {
    pub fn parse_statement(&mut self) -> ParserResult<Statement> {
        let token = self.token();

        match token {
            token if is_keyword_let(token) => {
                let variable_declaration = self.parse_variable_declaration().expect(
                    format!(
                        "Failed to parse variable declaration statement {}.",
                        self.token()
                    )
                    .as_str(),
                );

                Ok(Statement {
                    kind: StatementKind::VariableDeclaration(variable_declaration),
                })
            }
            token if is_keyword_fun(token) => {
                let function_declaration = self.parse_function_declaration().expect(
                    format!(
                        "Failed to parse function declaration statement {}.",
                        self.token()
                    )
                    .as_str(),
                );

                Ok(Statement {
                    kind: StatementKind::FunctionDeclaration(function_declaration),
                })
            }
            _ => {
                let statement_expression = self.parse_expression().expect(
                    format!(
                        "Failed to parse return statement expression {}.",
                        self.token()
                    )
                    .as_str(),
                );

                Ok(Statement {
                    kind: StatementKind::Return(Box::new(statement_expression)),
                })
            }
        }
    }
}
