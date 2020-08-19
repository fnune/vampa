use vamc_lexer::definitions::*;

pub fn is_binary_operator(token: &Token) -> bool {
    match token.kind {
        TokenKind::PlusSign => true,
        TokenKind::MinusSign => true,
        TokenKind::Slash => true,
        TokenKind::PercentSign => true,
        TokenKind::Star => true,
        _ => false,
    }
}

pub fn is_keyword(token: &Token) -> bool {
    is_keyword_let(token)
        || is_keyword_fun(token)
        || is_keyword_of(token)
        || is_keyword_returning(token)
        || is_keyword_and(token)
}

pub fn is_keyword_let(token: &Token) -> bool {
    match token.kind {
        TokenKind::Identifier => token.value == "let",
        _ => false,
    }
}

pub fn is_keyword_fun(token: &Token) -> bool {
    match token.kind {
        TokenKind::Identifier => token.value == "fun",
        _ => false,
    }
}

pub fn is_keyword_of(token: &Token) -> bool {
    match token.kind {
        TokenKind::Identifier => token.value == "of",
        _ => false,
    }
}

pub fn is_keyword_returning(token: &Token) -> bool {
    match token.kind {
        TokenKind::Identifier => token.value == "returning",
        _ => false,
    }
}

pub fn is_keyword_and(token: &Token) -> bool {
    match token.kind {
        TokenKind::Identifier => token.value == "and",
        _ => false,
    }
}
