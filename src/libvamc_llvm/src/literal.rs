use crate::definitions::{Compiler, CompilerResult};

use inkwell::values::AnyValueEnum;

use vamc_parser::definitions::ast::{Literal, LiteralIntType, LiteralKind};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn compile_literal(&self, literal: Literal) -> CompilerResult<AnyValueEnum> {
        match literal.kind {
            LiteralKind::Int(value, typ) => match typ {
                LiteralIntType::Unsuffixed => Ok(AnyValueEnum::IntValue(
                    self.context.i32_type().const_int(value as u64, false),
                )),
            },
        }
    }
}
