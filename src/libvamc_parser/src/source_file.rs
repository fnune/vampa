use crate::definitions::{ast::SourceFile, Parser, ParserResult};

impl Parser {
    pub fn parse_source_file(&mut self, file_name: &str) -> ParserResult<SourceFile> {
        self.bump_while_whitespace();

        let statements = self.parse_block_statements();

        Ok(SourceFile {
            file_name: file_name.into(),
            statements,
        })
    }
}
