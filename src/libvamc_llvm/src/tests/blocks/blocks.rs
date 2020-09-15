use insta::assert_snapshot;
use vamc_parser::definitions::ast::*;

use crate::tests::util::*;

#[test]
fn basic_block() {
    let block = Block {
        statements: vec![
            Box::new(Statement {
                kind: StatementKind::VariableDeclaration(VariableDeclaration {
                    name: Box::new("first".to_owned()),
                    typ: Box::new(Typ {
                        kind: TypKind::Int(IntType::I32),
                    }),
                    value: Box::new(Expression {
                        kind: ExpressionKind::Literal(Literal {
                            kind: LiteralKind::Int(20, LiteralIntType::Unsuffixed),
                        }),
                    }),
                }),
            }),
            Box::new(Statement {
                kind: StatementKind::Return(Box::new(Expression {
                    kind: ExpressionKind::Literal(Literal {
                        kind: LiteralKind::Int(30, LiteralIntType::Unsuffixed),
                    }),
                })),
            }),
        ],
    };

    with_compiler(|mut compiler, _| {
        compiler.compile_block(block).unwrap();

        assert_snapshot!(compiler.module.print_to_string().to_string_lossy())
    })
}
