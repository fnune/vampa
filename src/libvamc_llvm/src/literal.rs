use crate::definitions::{Compiler, CompilerResult};

use inkwell::values::BasicValueEnum;
use vamc_parser::definitions::ast::{Literal, LiteralIntType, LiteralKind};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn compile_literal(&self, literal: Literal) -> CompilerResult<BasicValueEnum<'ctx>> {
        match literal.kind {
            LiteralKind::Int(value, typ) => match typ {
                LiteralIntType::Unsuffixed => Ok(BasicValueEnum::IntValue(
                    self.context.i32_type().const_int(value as u64, false),
                )),
            },
        }
    }
}
