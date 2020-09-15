use crate::definitions::{Compiler, CompilerResult};

use inkwell::types::{BasicType, BasicTypeEnum};
use vamc_errors::Diagnostic;
use vamc_parser::definitions::ast::{IntType, Typ, TypKind};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn compile_typ(&self, typ: &Typ) -> CompilerResult<BasicTypeEnum<'ctx>> {
        match &typ.kind {
            TypKind::Int(int_type) => match int_type {
                IntType::I32 => Ok(self.context.i32_type().as_basic_type_enum()),
            },
            _ => Err(Diagnostic::error("Only int types are supported.".into())),
        }
    }
}
