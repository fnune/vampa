use vamc_errors::{Diagnostic, DiagnosticLevel};
use vamc_lexer::{cursor::Cursor, definitions::Token};

use crate::{definitions::ast::*, *};

#[test]
fn variable_reference() {
    let tokens: Vec<Token> = Cursor::new("variable_name").collect();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_expression();

    assert_eq!(
        result,
        Ok(Expression {
            kind: ExpressionKind::VariableReference(Box::new("variable_name".to_owned()))
        })
    )
}

#[test]
fn variable_reference_but_is_keyword() {
    let tokens: Vec<Token> = Cursor::new("and").collect();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_expression();

    assert_eq!(
        result,
        Err(Diagnostic {
            level: DiagnosticLevel::Error,
            message:
                "Failed to parse variable reference: [Identifier] `and` is a reserved keyword."
                    .into(),
        })
    )
}
