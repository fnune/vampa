use std::collections::HashMap;

use inkwell::{basic_block::BasicBlock, context::Context};

use crate::definitions::*;

/// Builds a valid compiler with a block in which to write unit tests and
/// verifies the resulting function value. A failure to verify the function
/// value will result in a failing test. This is desired.
///
/// If the test function does not mutate the target block by adding a
/// terminator, this test will add a dummy terminator for convenience.
pub fn with_compiler<F: FnOnce(Compiler, BasicBlock) -> ()>(test: F) {
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

    test(compiler, target_block);

    // Append a terminator instruction for convenience.
    // The user can overwrite it, and then this won't run.
    if target_block.get_terminator().is_none() {
        builder.position_at_end(target_block);
        builder.build_return(Some(&context.i32_type().const_int(42, false)));
    };

    assert!(function_value.verify(true));
}
