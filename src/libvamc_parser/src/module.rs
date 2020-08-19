use crate::definitions::{ast::Module, Parser, ParserResult};

impl Parser {
    pub fn parse_module(&mut self, file_name: &str) -> ParserResult<Module> {
        self.bump_while_whitespace();

        let statements = self.parse_block_statements();

        Ok(Module {
            file_name: file_name.into(),
            statements,
        })
    }
}
