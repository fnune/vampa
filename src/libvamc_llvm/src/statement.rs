use crate::definitions::{Compiler, CompilerResult};

use inkwell::{
    basic_block::BasicBlock,
    values::{BasicValue, BasicValueEnum},
};
use vamc_parser::definitions::ast::{Statement, StatementKind};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn compile_statement(
        &self,
        target_block: BasicBlock,
        statement: Statement,
    ) -> CompilerResult<BasicValueEnum>
    {
        match statement.kind {
            StatementKind::VariableDeclaration(variable_declaration) => {
                self.compile_variable_declaration(target_block, variable_declaration)
                    .unwrap();
                Ok(self.context.i8_type().const_zero().as_basic_value_enum())
            },
            StatementKind::FunctionDeclaration(function_declaration) => {
                self.compile_function_declaration(function_declaration)
                    .unwrap();
                Ok(self.context.i8_type().const_zero().as_basic_value_enum())
            },
            StatementKind::Return(statement_expression) => {
                let compiled_expression = self.compile_expression(*statement_expression).unwrap();
                self.builder.build_return(Some(&compiled_expression));
                Ok(compiled_expression)
            },
        }
    }
}
