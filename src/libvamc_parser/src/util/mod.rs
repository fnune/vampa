use vamc_lexer::definitions::*;

pub fn is_binary_operator(token: &Token) -> bool {
    matches!(
        token.kind,
        TokenKind::PlusSign
            | TokenKind::MinusSign
            | TokenKind::Slash
            | TokenKind::PercentSign
            | TokenKind::Star
    )
}

pub fn is_parameters_list_termination(token: &Token) -> bool {
    matches!(token.kind, TokenKind::ClosingBrace | TokenKind::Semicolon)
}

pub fn is_identifier_exactly(identifier: &str, token: &Token) -> bool {
    match token.kind {
        TokenKind::Identifier => token.value == identifier,
        _ => false,
    }
}

pub fn is_keyword_and(token: &Token) -> bool {
    is_identifier_exactly("and", token)
}
pub fn is_keyword_apply(token: &Token) -> bool {
    is_identifier_exactly("apply", token)
}
pub fn is_keyword_fun(token: &Token) -> bool {
    is_identifier_exactly("fun", token)
}
pub fn is_keyword_let(token: &Token) -> bool {
    is_identifier_exactly("let", token)
}
pub fn is_keyword_of(token: &Token) -> bool {
    is_identifier_exactly("of", token)
}
pub fn is_keyword_returning(token: &Token) -> bool {
    is_identifier_exactly("returning", token)
}

pub fn is_keyword(token: &Token) -> bool {
    is_keyword_and(token)
        || is_keyword_apply(token)
        || is_keyword_fun(token)
        || is_keyword_let(token)
        || is_keyword_of(token)
        || is_keyword_returning(token)
}
