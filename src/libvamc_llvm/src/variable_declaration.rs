use crate::definitions::{Compiler, CompilerResult};

use inkwell::{basic_block::BasicBlock, values::BasicValueEnum};

use vamc_errors::Diagnostic;
use vamc_parser::definitions::ast::{IntType, TypKind, VariableDeclaration};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// We return the assigned value, but the important part is the side effect
    /// of allocating the newly-declared variable into the target block.
    pub fn compile_variable_declaration(
        &self,
        target_block: BasicBlock,
        variable_declaration: VariableDeclaration,
    ) -> CompilerResult<BasicValueEnum>
    {
        let name = variable_declaration.name;

        match target_block.get_first_instruction() {
            Some(instruction) => self.builder.position_before(&instruction),
            None => self.builder.position_at_end(target_block),
        }

        if let Ok(value) = self.compile_expression(*variable_declaration.value) {
            match variable_declaration.typ.kind {
                TypKind::Int(int_type) => match int_type {
                    IntType::I32 => {
                        let allocation = self
                            .builder
                            .build_alloca(self.context.i32_type(), name.as_str());

                        self.builder.build_store(allocation, value.into_int_value());

                        Ok(value)
                    },
                },
                TypKind::Infer => Err(Diagnostic::error(
                    "Type inference isn't supported yet.".into(),
                )),
            }
        } else {
            Err(Diagnostic::error("Failed to compile expression.".into()))
        }
    }
}
