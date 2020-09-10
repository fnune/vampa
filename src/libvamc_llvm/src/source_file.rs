use crate::definitions::{Compiler, CompilerResult};

use inkwell::values::BasicValueEnum;
use vamc_parser::definitions::ast::SourceFile;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn compile_source_file(&self, source_file: SourceFile) -> CompilerResult<BasicValueEnum> {
        let target_block = self.context.append_basic_block(
            *self
                .function_value
                .expect("Expected a function value in the compiler but found None."),
            &source_file.file_name,
        );

        let compiled_statements = source_file
            .statements
            .into_iter()
            .map(|statement| self.compile_statement(target_block, *statement));

        Ok(compiled_statements.last().unwrap().unwrap())
    }
}
