use vamc_lexer::{cursor::Cursor, definitions::Token};

use crate::{definitions::ast::*, *};

#[test]
fn typed_function_declaration() {
    let tokens: Vec<Token> = Cursor::new("fun return_one of x: i32 returning i32 = 1;").collect();
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
                    kind: ExpressionKind::Literal(Literal {
                        kind: LiteralKind::Int(1, LiteralIntType::Unsuffixed)
                    })
                })
            })
        })
    )
}

#[test]
fn typed_nullary_function_declaration() {
    let tokens: Vec<Token> = Cursor::new("fun return_one returning i32 = 1;").collect();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_statement();

    assert_eq!(
        result,
        Ok(Statement {
            kind: StatementKind::FunctionDeclaration(FunctionDeclaration {
                name: Box::new("return_one".to_owned()),
                parameters: vec![],
                return_typ: Box::new(Typ {
                    kind: TypKind::Int(IntType::I32)
                }),
                body: Box::new(Expression {
                    kind: ExpressionKind::Literal(Literal {
                        kind: LiteralKind::Int(1, LiteralIntType::Unsuffixed)
                    })
                })
            })
        })
    )
}

#[test]
fn typed_binary_function_declaration() {
    let tokens: Vec<Token> =
        Cursor::new("fun return_one of a: i32 and b: i32 returning i32 = 1;").collect();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_statement();

    assert_eq!(
        result,
        Ok(Statement {
            kind: StatementKind::FunctionDeclaration(FunctionDeclaration {
                name: Box::new("return_one".to_owned()),
                parameters: vec![
                    Box::new(Parameter {
                        name: Box::new("a".to_owned()),
                        typ: Box::new(Typ {
                            kind: TypKind::Int(IntType::I32)
                        })
                    }),
                    Box::new(Parameter {
                        name: Box::new("b".to_owned()),
                        typ: Box::new(Typ {
                            kind: TypKind::Int(IntType::I32)
                        })
                    })
                ],
                return_typ: Box::new(Typ {
                    kind: TypKind::Int(IntType::I32)
                }),
                body: Box::new(Expression {
                    kind: ExpressionKind::Literal(Literal {
                        kind: LiteralKind::Int(1, LiteralIntType::Unsuffixed)
                    })
                })
            })
        })
    )
}

#[test]
fn typed_ternary_function_declaration() {
    let tokens: Vec<Token> =
        Cursor::new("fun return_one of a: i32 and b: i32 and c: i32 returning i32 = 1;").collect();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_statement();

    assert_eq!(
        result,
        Ok(Statement {
            kind: StatementKind::FunctionDeclaration(FunctionDeclaration {
                name: Box::new("return_one".to_owned()),
                parameters: vec![
                    Box::new(Parameter {
                        name: Box::new("a".to_owned()),
                        typ: Box::new(Typ {
                            kind: TypKind::Int(IntType::I32)
                        })
                    }),
                    Box::new(Parameter {
                        name: Box::new("b".to_owned()),
                        typ: Box::new(Typ {
                            kind: TypKind::Int(IntType::I32)
                        })
                    }),
                    Box::new(Parameter {
                        name: Box::new("c".to_owned()),
                        typ: Box::new(Typ {
                            kind: TypKind::Int(IntType::I32)
                        })
                    })
                ],
                return_typ: Box::new(Typ {
                    kind: TypKind::Int(IntType::I32)
                }),
                body: Box::new(Expression {
                    kind: ExpressionKind::Literal(Literal {
                        kind: LiteralKind::Int(1, LiteralIntType::Unsuffixed)
                    })
                })
            })
        })
    )
}
