use std::collections::HashMap;

use inkwell::{basic_block::BasicBlock, context::Context, values::FunctionValue};

use crate::definitions::*;

/// Builds a valid compiler with a block in which to write unit tests and
/// verifies the resulting function value. A failure to verify the function
/// value will result in a failing test. This is desired.
pub fn with_compiler<F: FnOnce(Compiler, BasicBlock, FunctionValue) -> ()>(test: F) {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();

    let mut compiler = Compiler {
        context: &context,
        builder: &builder,
        module: &module,
        function_value: None,
        variables: HashMap::new(),
    };

    let function_type = compiler.context.i32_type().fn_type(&[], false);
    let function_value = compiler
        .module
        .add_function("test_function", function_type, None);

    compiler.function_value = Some(&function_value);

    let target_block = compiler
        .context
        .append_basic_block(function_value, "test_block");

    test(compiler, target_block, function_value);
}
