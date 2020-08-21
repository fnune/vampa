use insta::assert_snapshot;
use vamc_parser::definitions::ast::*;

use crate::tests::util::*;

#[test]
fn simple_variable_declaration() {
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

    with_compiler(|compiler, target_block| {
        compiler
            .compile_variable_declaration(target_block, variable_declaration)
            .unwrap();

        assert_snapshot!(compiler.module.print_to_string().to_string_lossy())
    });
}
