use crate::definitions::{Compiler, CompilerResult};

use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum};
use vamc_errors::Diagnostic;
use vamc_parser::definitions::ast::Expression;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn compile_function_call(
        &mut self,
        function_name: &str,
        parameters: Vec<Box<Expression>>,
    ) -> CompilerResult<BasicValueEnum<'ctx>> {
        self.builder
            .position_at_end(self.function_value.unwrap().get_last_basic_block().unwrap());

        let function_value = self.module.get_function(function_name).unwrap_or_else(|| {
            panic!(
                "{}",
                "Failed to find a function with name `{function_name}`."
            )
        });

        let mut compiled_parameters: Vec<BasicMetadataValueEnum> =
            Vec::with_capacity(parameters.len());

        for parameter in parameters {
            let compiled_expression = self.compile_expression(*parameter)?;
            compiled_parameters.push(compiled_expression.into());
        }

        match self
            .builder
            .build_call(
                function_value,
                compiled_parameters.as_slice(),
                &format!("apply_{function_name}"),
            )
            .try_as_basic_value()
            .left()
        {
            Some(value) => Ok(value),
            None => Err(Diagnostic::error(
                "Failed to produce a valid function call.".into(),
            )),
        }
    }
}
