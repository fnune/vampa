use std::collections::HashMap;

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    values::{FunctionValue, PointerValue},
};
use vamc_errors::Diagnostic;

pub type CompilerResult<T> = Result<T, Diagnostic>;

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    pub function_value: Option<&'a FunctionValue<'a>>,

    pub variables: HashMap<String, PointerValue<'ctx>>,
}
