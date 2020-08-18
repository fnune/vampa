use vamc_lexer::definitions::*;

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
