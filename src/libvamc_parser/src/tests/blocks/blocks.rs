use vamc_lexer::{cursor::Cursor, definitions::Token};

use crate::{definitions::ast::*, *};

#[test]
fn block_with_two_statements_one_variable_declaration() {
    let tokens: Vec<Token> = Cursor::new("{ let first: i32 = 20; 30 }").collect();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_expression();

    let block_ast = Block {
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

    assert_eq!(
        result,
        Ok(Expression {
            kind: ExpressionKind::Block(Box::new(block_ast))
        })
    )
}

#[test]
fn block_with_two_statements_one_function_declaration() {
    let tokens: Vec<Token> = Cursor::new("{ fun return_one = 1; 30 }").collect();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_expression();

    let block_ast = Block {
        statements: vec![
            Box::new(Statement {
                kind: StatementKind::FunctionDeclaration(FunctionDeclaration {
                    name: Box::new("return_one".to_owned()),
                    parameters: vec![],
                    return_typ: Box::new(Typ {
                        kind: TypKind::Infer,
                    }),
                    body: Box::new(Expression {
                        kind: ExpressionKind::Literal(Literal {
                            kind: LiteralKind::Int(1, LiteralIntType::Unsuffixed),
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

    assert_eq!(
        result,
        Ok(Expression {
            kind: ExpressionKind::Block(Box::new(block_ast))
        })
    )
}
