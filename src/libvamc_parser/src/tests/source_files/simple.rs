use vamc_lexer::{cursor::Cursor, definitions::Token};

use crate::{definitions::ast::*, *};

#[test]
fn simple_module() {
    let tokens: Vec<Token> = Cursor::new("let first: i32 = 20; 30").collect();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_source_file("program.vam");

    assert_eq!(
        result,
        Ok(SourceFile {
            file_name: "program.vam".into(),
            statements: vec![
                Box::new(Statement {
                    kind: StatementKind::VariableDeclaration(VariableDeclaration {
                        name: Box::new("first".into()),
                        typ: Box::new(Typ {
                            kind: TypKind::Int(IntType::I32)
                        }),
                        value: Box::new(Expression {
                            kind: ExpressionKind::Literal(Literal {
                                kind: LiteralKind::Int(20, LiteralIntType::Unsuffixed)
                            })
                        })
                    })
                }),
                Box::new(Statement {
                    kind: StatementKind::Return(Box::new(Expression {
                        kind: ExpressionKind::Literal(Literal {
                            kind: LiteralKind::Int(30, LiteralIntType::Unsuffixed),
                        }),
                    })),
                }),
            ]
        })
    );
}

#[test]
fn simple_module_with_variable_reference() {
    let tokens: Vec<Token> = Cursor::new("first").collect();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_source_file("program.vam");

    assert_eq!(
        result,
        Ok(SourceFile {
            file_name: "program.vam".into(),
            statements: vec![Box::new(Statement {
                kind: StatementKind::Return(Box::new(Expression {
                    kind: ExpressionKind::VariableReference(Box::new("first".to_string())),
                })),
            }),]
        })
    );
}
