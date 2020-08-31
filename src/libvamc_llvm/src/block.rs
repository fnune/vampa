use crate::definitions::{Compiler, CompilerResult};

use inkwell::basic_block::BasicBlock;
use vamc_parser::definitions::ast::Block;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn compile_block(&self, block: Block) -> CompilerResult<BasicBlock> {
        let target_block = self.context.append_basic_block(
            *self
                .function_value
                .expect("Expected a function value in the compiler but found None."),
            "vampa_block",
        );

        block.statements.into_iter().for_each(|statement| {
            println!("Compiling statement: {:?}", statement);
            self.compile_statement(target_block, *statement);
        });

        Ok(target_block)
    }
}
