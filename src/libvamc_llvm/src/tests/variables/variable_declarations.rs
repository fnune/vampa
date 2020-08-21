use std::collections::HashMap;

use inkwell::context::Context;
use insta::assert_snapshot;
use vamc_parser::definitions::ast::*;

use crate::definitions::*;

#[test]
fn simple_variable_declaration() {
    let context = Context::create();
    let module = context.create_module("repl");
    let builder = context.create_builder();

    let compiler = Compiler {
        context: &context,
        builder: &builder,
        module: &module,
        variables: HashMap::new(),
    };

    let function_type = compiler.context.i32_type().fn_type(&[], false);
    let function_value = compiler.module.add_function("test", function_type, None);

    let target_block = compiler
        .context
        .append_basic_block(function_value, "target");

    let variable_declaration = VariableDeclaration {
        name: Box::new("first".to_owned()),
        typ: Box::new(Typ {
            kind: TypKind::Int(IntType::I32),
        }),
        value: Box::new(Expression {
            kind: ExpressionKind::Literal(Literal {
                kind: LiteralKind::Int(20, LiteralIntType::Unsuffixed),
            }),
        }),
    };

    compiler
        .compile_variable_declaration(target_block, variable_declaration)
        .unwrap();

    assert_snapshot!(function_value.print_to_string().to_string_lossy())
}
