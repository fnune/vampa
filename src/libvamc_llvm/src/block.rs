use crate::definitions::{Compiler, CompilerResult};

use inkwell::values::BasicValueEnum;
use vamc_parser::definitions::ast::Block;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn compile_block(&mut self, block: Block) -> CompilerResult<BasicValueEnum<'ctx>> {
        let target_block = self.context.append_basic_block(
            *self
                .function_value
                .expect("Expected a function value in the compiler but found None."),
            "vampa_block",
        );

        self.builder.position_at_end(target_block);

        let compiled_statements = block
            .statements
            .into_iter()
            .map(|statement| self.compile_statement(target_block, *statement));

        Ok(compiled_statements.last().unwrap().unwrap())
    }
}
