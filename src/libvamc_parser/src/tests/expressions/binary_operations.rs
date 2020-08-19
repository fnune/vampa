use vamc_lexer::{cursor::Cursor, definitions::Token};

use crate::{definitions::ast::*, *};

#[test]
fn simple_addition() {
    let tokens: Vec<Token> = Cursor::new("+ 2 3").collect();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_expression();

    assert_eq!(
        result,
        Ok(Expression {
            kind: ExpressionKind::BinaryOperation(BinaryOperation {
                kind: BinaryOperationKind::Addition,
                left: Box::new(Expression {
                    kind: ExpressionKind::Literal(Literal {
                        kind: LiteralKind::Int(2, LiteralIntType::Unsuffixed)
                    })
                }),
                right: Box::new(Expression {
                    kind: ExpressionKind::Literal(Literal {
                        kind: LiteralKind::Int(3, LiteralIntType::Unsuffixed)
                    })
                })
            })
        })
    )
}

#[test]
fn nested_addition() {
    let tokens: Vec<Token> = Cursor::new("+ + 2 3 4").collect();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_expression();

    assert_eq!(
        result,
        Ok(Expression {
            kind: ExpressionKind::BinaryOperation(BinaryOperation {
                kind: BinaryOperationKind::Addition,
                left: Box::new(Expression {
                    kind: ExpressionKind::BinaryOperation(BinaryOperation {
                        kind: BinaryOperationKind::Addition,
                        left: Box::new(Expression {
                            kind: ExpressionKind::Literal(Literal {
                                kind: LiteralKind::Int(2, LiteralIntType::Unsuffixed)
                            })
                        }),
                        right: Box::new(Expression {
                            kind: ExpressionKind::Literal(Literal {
                                kind: LiteralKind::Int(3, LiteralIntType::Unsuffixed)
                            })
                        })
                    })
                }),
                right: Box::new(Expression {
                    kind: ExpressionKind::Literal(Literal {
                        kind: LiteralKind::Int(4, LiteralIntType::Unsuffixed)
                    })
                })
            })
        })
    )
}
