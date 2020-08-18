#[cfg(test)]
mod test {
    use vamc_lexer::cursor::Cursor;
    use vamc_lexer::definitions::{Base, LiteralKind, Token};

    use crate::*;

    #[test]
    fn works_with_a_basic_assignment() {
        let tokens: Vec<Token> = Cursor::new("let first: i32 = 20;").collect();
        let mut parser = Parser::new(tokens);
        let result = parser.parse_statement();

        println!("{:?}", result);
    }
}
