use crate::definitions::{Compiler, CompilerResult};

use inkwell::{
    types::{BasicMetadataTypeEnum, BasicTypeEnum},
    values::{BasicValue, FunctionValue},
};
use vamc_errors::Diagnostic;
use vamc_parser::definitions::ast::{FunctionDeclaration, IntType, Parameters, Typ, TypKind};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn compile_function_declaration(
        &mut self,
        function_declaration: FunctionDeclaration,
    ) -> CompilerResult<FunctionValue> {
        let function_name = function_declaration.name.as_str();

        let function_body_expression = self
            .compile_expression(*function_declaration.body)
            .expect("Failed to compile function body expression.");

        let function = self
            .compile_function_signature(
                function_declaration.parameters,
                *function_declaration.return_typ,
                function_name,
            )
            .expect("Failed to compile function signature.");

        let function_return_type = function.get_type().get_return_type();

        let function_body_block = self.context.append_basic_block(function, function_name);

        let function_body_expression =
            match function_return_type.expect("Functions must all return something.") {
                BasicTypeEnum::IntType(_) => function_body_expression.into_int_value(),
                _ => panic!(
                    "{:?}",
                    Diagnostic::error("Only int types are supported.".into())
                ),
            };

        self.builder.position_at_end(function_body_block);
        self.builder.build_return(Some(&function_body_expression));

        Ok(function)
    }

    pub fn compile_function_signature(
        &self,
        function_parameters: Parameters,
        function_return_typ: Typ,
        function_name: &str,
    ) -> CompilerResult<FunctionValue> {
        let function_return_type = match function_return_typ.kind {
            TypKind::Int(int_type) => match int_type {
                IntType::I32 => self.context.i32_type(),
            },
            _ => panic!(
                "{:?}",
                Diagnostic::error("Only int types are supported.".into())
            ),
        };

        let (parameter_names, parameter_types): (Vec<Box<String>>, Vec<BasicMetadataTypeEnum>) =
            function_parameters
                .into_iter()
                .map(|parameter| {
                    let compiled_type: BasicMetadataTypeEnum = self
                        .compile_typ(&parameter.typ)
                        .expect("Failed to compile type in function parameters declaration.")
                        .into();
                    (parameter.name, compiled_type)
                })
                .unzip();

        let parameter_types_slice = parameter_types.as_slice();

        let function_signature = function_return_type.fn_type(parameter_types_slice, false);

        let function_value = self
            .module
            .add_function(function_name, function_signature, None);

        for (index, parameter) in function_value.get_param_iter().enumerate() {
            parameter
                .as_basic_value_enum()
                .set_name(parameter_names[index].as_str());
        }

        Ok(function_value)
    }
}
