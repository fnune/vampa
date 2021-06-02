use vamc_errors::Diagnostic;

use crate::{
    definitions::{
        ast::{Expression, ExpressionKind},
        Parser, ParserResult,
    },
    util::is_keyword,
};

impl Parser {
    pub fn parse_variable_reference(&mut self) -> ParserResult<Expression> {
        let token = self.token();
        let token_is_keyword = is_keyword(token);

        let result = match token {
            _ if token_is_keyword => Err(Diagnostic::error(format!(
                "Failed to parse variable reference: {} is a reserved keyword.",
                self.token()
            ))),
            token => Ok(Expression {
                kind: ExpressionKind::VariableReference(Box::new(token.value.clone())),
            }),
        };

        // Eat the identifier token.
        self.bump_until_next();

        result
    }
}
