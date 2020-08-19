use vamc_lexer::{cursor::Cursor, definitions::Token};

use crate::{definitions::ast::*, *};

#[test]
fn function_call_nullary() {
    let tokens: Vec<Token> = Cursor::new("apply sum").collect();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_expression();

    assert_eq!(
        result,
        Ok(Expression {
            kind: ExpressionKind::FunctionCall(Box::new("sum".to_owned()), vec![])
        })
    )
}

#[test]
fn function_call_binary() {
    let tokens: Vec<Token> = Cursor::new("apply sum 10 15").collect();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_expression();

    assert_eq!(
        result,
        Ok(Expression {
            kind: ExpressionKind::FunctionCall(Box::new("sum".to_owned()), vec![
                Box::new(Expression {
                    kind: ExpressionKind::Literal(Literal {
                        kind: LiteralKind::Int(10, LiteralIntType::Unsuffixed)
                    })
                }),
                Box::new(Expression {
                    kind: ExpressionKind::Literal(Literal {
                        kind: LiteralKind::Int(15, LiteralIntType::Unsuffixed)
                    })
                })
            ])
        })
    )
}

#[test]
fn function_call_binary_with_braces() {
    let tokens: Vec<Token> = Cursor::new("apply sum { 10 } 15").collect();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_expression();

    assert_eq!(
        result,
        Ok(Expression {
            kind: ExpressionKind::FunctionCall(Box::new("sum".to_owned()), vec![
                Box::new(Expression {
                    kind: ExpressionKind::Block(Box::new(Block {
                        statements: vec![Box::new(Statement {
                            kind: StatementKind::Return(Box::new(Expression {
                                kind: ExpressionKind::Literal(Literal {
                                    kind: LiteralKind::Int(10, LiteralIntType::Unsuffixed)
                                })
                            }))
                        })]
                    })),
                }),
                Box::new(Expression {
                    kind: ExpressionKind::Literal(Literal {
                        kind: LiteralKind::Int(15, LiteralIntType::Unsuffixed)
                    })
                })
            ])
        })
    )
}

#[test]
fn function_call_binary_as_statement_with_semicolon() {
    let tokens: Vec<Token> = Cursor::new("let aggregate = apply sum 10 15;").collect();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_statement();

    assert_eq!(
        result,
        Ok(Statement {
            kind: StatementKind::VariableDeclaration(VariableDeclaration {
                name: Box::new("aggregate".to_owned()),
                typ: Box::new(Typ {
                    kind: TypKind::Infer
                }),
                value: Box::new(Expression {
                    kind: ExpressionKind::FunctionCall(Box::new("sum".to_owned()), vec![
                        Box::new(Expression {
                            kind: ExpressionKind::Literal(Literal {
                                kind: LiteralKind::Int(10, LiteralIntType::Unsuffixed)
                            })
                        }),
                        Box::new(Expression {
                            kind: ExpressionKind::Literal(Literal {
                                kind: LiteralKind::Int(15, LiteralIntType::Unsuffixed)
                            })
                        })
                    ])
                })
            })
        })
    )
}

#[test]
fn function_call_binary_as_statement_with_braces() {
    let tokens: Vec<Token> = Cursor::new("let aggregate = { apply sum 10 15 };").collect();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_statement();

    let block = Expression {
        kind: ExpressionKind::Block(Box::new(Block {
            statements: vec![Box::new(Statement {
                kind: StatementKind::Return(Box::new(Expression {
                    kind: ExpressionKind::FunctionCall(Box::new("sum".to_owned()), vec![
                        Box::new(Expression {
                            kind: ExpressionKind::Literal(Literal {
                                kind: LiteralKind::Int(10, LiteralIntType::Unsuffixed),
                            }),
                        }),
                        Box::new(Expression {
                            kind: ExpressionKind::Literal(Literal {
                                kind: LiteralKind::Int(15, LiteralIntType::Unsuffixed),
                            }),
                        }),
                    ]),
                })),
            })],
        })),
    };

    assert_eq!(
        result,
        Ok(Statement {
            kind: StatementKind::VariableDeclaration(VariableDeclaration {
                name: Box::new("aggregate".to_owned()),
                typ: Box::new(Typ {
                    kind: TypKind::Infer
                }),
                value: Box::new(block)
            })
        })
    )
}
