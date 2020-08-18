use vamc_lexer::definitions::*;

pub fn is_keyword_let(token: &Token) -> bool {
    match token.kind {
        TokenKind::Identifier => token.value == "let",
        _ => false,
    }
}
