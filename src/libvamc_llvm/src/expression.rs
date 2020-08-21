use crate::definitions::{Compiler, CompilerResult};

use inkwell::values::AnyValueEnum;
use vamc_parser::definitions::ast::{Expression, ExpressionKind};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn compile_expression(&self, expression: Expression) -> CompilerResult<AnyValueEnum> {
        match expression.kind {
            ExpressionKind::Literal(literal) => self.compile_literal(literal),
            _ => unimplemented!(),
        }
    }
}
