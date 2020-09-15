use crate::definitions::{Compiler, CompilerResult};

use inkwell::values::BasicValueEnum;
use vamc_errors::Diagnostic;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn compile_variable_reference(
        &mut self,
        variable_name: &str,
    ) -> CompilerResult<BasicValueEnum<'ctx>>
    {
        match self.variables.get(variable_name) {
            Some(pointer_value) => Ok(self.builder.build_load(*pointer_value, variable_name)),
            None => Err(Diagnostic::error("Reference to undefined variable.".into())),
        }
    }
}
