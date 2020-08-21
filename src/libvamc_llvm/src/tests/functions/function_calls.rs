use insta::assert_snapshot;
use vamc_parser::definitions::ast::*;

use crate::tests::util::*;

#[test]
fn typed_function_declaration() {
    let function_declaration = FunctionDeclaration {
        name: Box::new("return_one".to_owned()),
        parameters: vec![Box::new(Parameter {
            name: Box::new("x".to_owned()),
            typ: Box::new(Typ {
                kind: TypKind::Int(IntType::I32),
            }),
        })],
        return_typ: Box::new(Typ {
            kind: TypKind::Int(IntType::I32),
        }),
        body: Box::new(Expression {
            kind: ExpressionKind::Literal(Literal {
                kind: LiteralKind::Int(1, LiteralIntType::Unsuffixed),
            }),
        }),
    };

    with_compiler(|compiler, _| {
        compiler
            .compile_function_declaration(function_declaration)
            .unwrap();

        compiler
            .compile_function_call("return_one", vec![Box::new(Expression {
                kind: ExpressionKind::Literal(Literal {
                    kind: LiteralKind::Int(10, LiteralIntType::Unsuffixed),
                }),
            })])
            .unwrap();

        assert_snapshot!(compiler.module.print_to_string().to_string_lossy())
    });
}
