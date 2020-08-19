use vamc_lexer::{cursor::Cursor, definitions::Token};

use crate::{definitions::ast::*, *};

#[test]
fn function_declaration_with_block_body() {
    let tokens: Vec<Token> =
        Cursor::new("fun return_one of x: i32 returning i32 = { 1 };").collect();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_statement();

    assert_eq!(
        result,
        Ok(Statement {
            kind: StatementKind::FunctionDeclaration(FunctionDeclaration {
                name: Box::new("return_one".to_owned()),
                parameters: vec![Box::new(Parameter {
                    name: Box::new("x".to_owned()),
                    typ: Box::new(Typ {
                        kind: TypKind::Int(IntType::I32)
                    })
                })],
                return_typ: Box::new(Typ {
                    kind: TypKind::Int(IntType::I32)
                }),
                body: Box::new(Expression {
                    kind: ExpressionKind::Block(Box::new(Block {
                        statements: vec![Box::new(Statement {
                            kind: StatementKind::Return(Box::new(Expression {
                                kind: ExpressionKind::Literal(Literal {
                                    kind: LiteralKind::Int(1, LiteralIntType::Unsuffixed)
                                })
                            }))
                        })]
                    }))
                })
            })
        })
    )
}
