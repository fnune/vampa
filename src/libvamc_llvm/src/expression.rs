use crate::definitions::{Compiler, CompilerResult};

use inkwell::values::BasicValueEnum;
use vamc_parser::definitions::ast::{Expression, ExpressionKind};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn compile_expression(&self, expression: Expression) -> CompilerResult<BasicValueEnum> {
        match expression.kind {
            ExpressionKind::Literal(literal) => self.compile_literal(literal),
            ExpressionKind::FunctionCall(function_name, function_parameters) => {
                self.compile_function_call(function_name.as_str(), function_parameters)
            },
            ExpressionKind::Block(block) => self.compile_block(*block),
            _ => unimplemented!(),
        }
    }
}
