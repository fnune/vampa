#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn works_with_a_basic_assignment() {
        let tokens: Vec<Token> = Cursor::new("let first: i32 = 20;").collect();

        assert_eq!(
            tokens,
            vec![
                Token::new(TokenKind::Identifier, "let"),
                Token::new(TokenKind::Whitespace, " "),
                Token::new(TokenKind::Identifier, "first"),
                Token::new(TokenKind::Colon, ":"),
                Token::new(TokenKind::Whitespace, " "),
                Token::new(TokenKind::Identifier, "i32"),
                Token::new(TokenKind::Whitespace, " "),
                Token::new(TokenKind::EqualitySign, "="),
                Token::new(TokenKind::Whitespace, " "),
                Token::new(TokenKind::Literal(LiteralKind::Int(Base::Decimal)), "20"),
                Token::new(TokenKind::Semicolon, ";"),
            ]
        )
    }

    #[test]
    fn works_with_an_inline_comment() {
        let tokens: Vec<Token> = Cursor::new("2 # The number 20").collect();

        assert_eq!(
            tokens,
            vec![
                Token::new(TokenKind::Literal(LiteralKind::Int(Base::Decimal)), "2"),
                Token::new(TokenKind::Whitespace, " "),
                Token::new(TokenKind::InlineComment, " The number 20"),
            ]
        )
    }

    #[test]
    fn works_with_a_terminated_depth_one_block_comment() {
        let tokens: Vec<Token> =
            Cursor::new("#[\n# This is a block comment\n# It spans multiple lines\n#]").collect();
        assert_eq!(
            tokens,
            vec![Token::new(
                TokenKind::BlockComment { terminated: true },
                "\n# This is a block comment\n# It spans multiple lines\n"
            )]
        );
    }

    #[test]
    fn works_with_a_terminated_depth_two_block_comment() {
        let tokens: Vec<Token> =
            Cursor::new("#[\n# Level one\n# #[ Level two #]\n# Level one\n#]").collect();
        assert_eq!(
            tokens,
            vec![Token::new(
                TokenKind::BlockComment { terminated: true },
                "\n# Level one\n# #[ Level two #]\n# Level one\n"
            )]
        );
    }

    #[test]
    fn works_with_an_unterminated_depth_two_block_comment() {
        let tokens: Vec<Token> =
            Cursor::new("#[\n# Level one\n# #[ Level two\n# Level one\n#]").collect();
        assert_eq!(
            tokens,
            vec![Token::new(
                TokenKind::BlockComment { terminated: false }, // Notice the `false` here.
                "\n# Level one\n# #[ Level two\n# Level one\n"
            )]
        );
    }

    #[test]
    fn works_with_braces() {
        let tokens: Vec<Token> = Cursor::new("sum { + 2 2 } 5;").collect();
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenKind::Identifier, "sum"),
                Token::new(TokenKind::Whitespace, " "),
                Token::new(TokenKind::OpeningBrace, "{"),
                Token::new(TokenKind::Whitespace, " "),
                Token::new(TokenKind::PlusSign, "+"),
                Token::new(TokenKind::Whitespace, " "),
                Token::new(TokenKind::Literal(LiteralKind::Int(Base::Decimal)), "2"),
                Token::new(TokenKind::Whitespace, " "),
                Token::new(TokenKind::Literal(LiteralKind::Int(Base::Decimal)), "2"),
                Token::new(TokenKind::Whitespace, " "),
                Token::new(TokenKind::ClosingBrace, "}"),
                Token::new(TokenKind::Whitespace, " "),
                Token::new(TokenKind::Literal(LiteralKind::Int(Base::Decimal)), "5"),
                Token::new(TokenKind::Semicolon, ";"),
            ]
        )
    }

    #[test]
    fn works_with_whitespace() {
        let tokens: Vec<Token> = Cursor::new("\nsum   \n\n \r   sum # A comment\n").collect();
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenKind::Whitespace, " "),
                Token::new(TokenKind::Identifier, "sum"),
                Token::new(TokenKind::Whitespace, " "),
                Token::new(TokenKind::Identifier, "sum"),
                Token::new(TokenKind::Whitespace, " "),
                Token::new(TokenKind::InlineComment, " A comment"),
                Token::new(TokenKind::Whitespace, " "),
            ]
        )
    }
}
