use std::collections::HashMap;

use inkwell::OptimizationLevel;
use inkwell::context::Context;
use inkwell::targets::{InitializationConfig, Target};
use vamc_lexer::{cursor::Cursor, definitions::Token};
use vamc_parser::definitions::Parser;

use crate::definitions::Compiler;

fn run(source: &str) -> i32 {
    Target::initialize_native(&InitializationConfig::default())
        .expect("Failed to initialize the native target.");

    let tokens: Vec<Token> = Cursor::new(source).collect();
    let mut parser = Parser::new(tokens);
    let source_file_ast = parser
        .parse_source_file("behavior")
        .expect("Failed to parse source file.");

    let context = Context::create();
    let module = context.create_module("behavior");
    let builder = context.create_builder();

    let mut compiler = Compiler {
        context: &context,
        builder: &builder,
        module: &module,
        function_value: None,
        variables: HashMap::new(),
    };

    let function_type = compiler.context.i32_type().fn_type(&[], false);
    let function_value = compiler.module.add_function("main", function_type, None);
    compiler.function_value = Some(&function_value);

    compiler.compile_source_file(source_file_ast);

    let engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .expect("Failed to create a JIT execution engine.");

    unsafe {
        let main = engine
            .get_function::<unsafe extern "C" fn() -> i32>("main")
            .expect("Failed to find `main` in the compiled module.");
        main.call()
    }
}

#[test]
fn function_returning_a_literal() {
    assert_eq!(run("fun three returning i32 = 3;\napply three"), 3);
}

#[test]
fn distinguishes_function_values() {
    assert_eq!(run("fun answer returning i32 = 42;\napply answer"), 42);
}

#[test]
fn resolves_the_applied_function_among_several() {
    let source = "fun one returning i32 = 1;\nfun two returning i32 = 2;\napply two";
    assert_eq!(run(source), 2);
}
