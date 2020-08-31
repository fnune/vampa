use crate::definitions::Compiler;

use inkwell::basic_block::BasicBlock;
use vamc_parser::definitions::ast::{Statement, StatementKind};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Does not return anything. It's here for the side effects.
    pub fn compile_statement(&self, target_block: BasicBlock, statement: Statement) {
        match statement.kind {
            StatementKind::VariableDeclaration(variable_declaration) => {
                self.compile_variable_declaration(target_block, variable_declaration)
                    .unwrap();
            },
            StatementKind::FunctionDeclaration(function_declaration) => {
                self.compile_function_declaration(function_declaration)
                    .unwrap();
            },
            StatementKind::Return(statement_expression) => {
                self.builder.build_return(Some(
                    &self.compile_expression(*statement_expression).unwrap(),
                ));
            },
        }
    }
}
