use std::collections::HashMap;

use inkwell::{builder::Builder, context::Context, module::Module, values::PointerValue};

use vamc_errors::Diagnostic;

pub type CompilerResult<T> = Result<T, Diagnostic>;

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,

    variables: HashMap<String, PointerValue<'ctx>>,
}
