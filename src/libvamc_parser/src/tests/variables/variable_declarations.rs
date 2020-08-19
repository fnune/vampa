use vamc_lexer::{cursor::Cursor, definitions::Token};

use crate::{definitions::ast::*, *};

#[test]
fn typed_variable_declaration() {
    let tokens: Vec<Token> = Cursor::new("let first: i32 = 20;").collect();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_statement();

    assert_eq!(
        result,
        Ok(Statement {
            kind: StatementKind::VariableDeclaration(VariableDeclaration {
                name: Box::new("first".to_owned()),
                typ: Box::new(Typ {
                    kind: TypKind::Int(IntType::I32)
                }),
                value: Box::new(Expression {
                    kind: ExpressionKind::Literal(Literal {
                        kind: LiteralKind::Int(20, LiteralIntType::Unsuffixed)
                    })
                })
            })
        })
    )
}

#[test]
fn inferred_variable_declaration() {
    let tokens: Vec<Token> = Cursor::new("let first = 20;").collect();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_statement();

    assert_eq!(
        result,
        Ok(Statement {
            kind: StatementKind::VariableDeclaration(VariableDeclaration {
                name: Box::new("first".to_owned()),
                typ: Box::new(Typ {
                    kind: TypKind::Infer
                }),
                value: Box::new(Expression {
                    kind: ExpressionKind::Literal(Literal {
                        kind: LiteralKind::Int(20, LiteralIntType::Unsuffixed)
                    })
                })
            })
        })
    )
}
