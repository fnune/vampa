use vamc_errors::Diagnostic;

use crate::{
    definitions::{
        Parser, ParserResult,
        ast::{Expression, ExpressionKind, Ident},
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
                kind: ExpressionKind::VariableReference(Box::new(Ident::new(
                    token.value.clone(),
                    token.span,
                ))),
            }),
        };

        // Eat the identifier token.
        self.bump_until_next();

        result
    }
}
