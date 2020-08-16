#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn works_in_a_basic_assignment() {
        let mut cursor = Cursor::new("let first: i32 = 20;");
        assert_eq!(cursor.advance(), Token::new(TokenKind::Identifier, "let"));
        assert_eq!(cursor.advance(), Token::new(TokenKind::Whitespace, " "));
        assert_eq!(cursor.advance(), Token::new(TokenKind::Identifier, "first"));
        assert_eq!(cursor.advance(), Token::new(TokenKind::Colon, ":"));
        assert_eq!(cursor.advance(), Token::new(TokenKind::Whitespace, " "));
        assert_eq!(cursor.advance(), Token::new(TokenKind::Identifier, "i32"));
        assert_eq!(cursor.advance(), Token::new(TokenKind::Whitespace, " "));
        assert_eq!(cursor.advance(), Token::new(TokenKind::EqualitySign, "="));
        assert_eq!(cursor.advance(), Token::new(TokenKind::Whitespace, " "));
        assert_eq!(
            cursor.advance(),
            Token::new(
                TokenKind::Literal {
                    kind: LiteralKind::Int {
                        base: Base::Decimal
                    }
                },
                "20"
            )
        );
        assert_eq!(cursor.advance(), Token::new(TokenKind::Semicolon, ";"));
    }

    #[test]
    fn works_with_an_inline_comment() {
        let mut cursor = Cursor::new("2 # The number 20");
        assert_eq!(
            cursor.advance(),
            Token::new(
                TokenKind::Literal {
                    kind: LiteralKind::Int {
                        base: Base::Decimal
                    }
                },
                "2"
            )
        );
        assert_eq!(cursor.advance(), Token::new(TokenKind::Whitespace, " "));
        assert_eq!(
            cursor.advance(),
            Token::new(TokenKind::InlineComment, " The number 20")
        );
    }

    #[test]
    fn works_with_a_terminated_depth_one_block_comment() {
        let mut cursor =
            Cursor::new("#[\n# This is a block comment\n# It spans multiple lines\n#]");
        assert_eq!(
            cursor.advance(),
            Token::new(
                TokenKind::BlockComment { terminated: true },
                "\n# This is a block comment\n# It spans multiple lines\n"
            )
        );
    }

    #[test]
    fn works_with_a_terminated_depth_two_block_comment() {
        let mut cursor = Cursor::new("#[\n# Level one\n# #[ Level two #]\n# Level one\n#]");
        assert_eq!(
            cursor.advance(),
            Token::new(
                TokenKind::BlockComment { terminated: true },
                "\n# Level one\n# #[ Level two #]\n# Level one\n"
            )
        );
    }

    #[test]
    fn works_with_an_unterminated_depth_two_block_comment() {
        let mut cursor = Cursor::new("#[\n# Level one\n# #[ Level two\n# Level one\n#]");
        assert_eq!(
            cursor.advance(),
            Token::new(
                TokenKind::BlockComment { terminated: false }, // Notice the `false` here.
                "\n# Level one\n# #[ Level two\n# Level one\n"
            )
        );
    }

    #[test]
    fn works_with_braces() {
        let mut cursor = Cursor::new("sum { + 2 2 } 5;");
        assert_eq!(cursor.advance(), Token::new(TokenKind::Identifier, "sum"));
        assert_eq!(cursor.advance(), Token::new(TokenKind::Whitespace, " "));
        assert_eq!(cursor.advance(), Token::new(TokenKind::OpeningBrace, "{"));
        assert_eq!(cursor.advance(), Token::new(TokenKind::Whitespace, " "));
        assert_eq!(cursor.advance(), Token::new(TokenKind::PlusSign, "+"));
        assert_eq!(cursor.advance(), Token::new(TokenKind::Whitespace, " "));
        assert_eq!(
            cursor.advance(),
            Token::new(
                TokenKind::Literal {
                    kind: LiteralKind::Int {
                        base: Base::Decimal
                    }
                },
                "2"
            )
        );
        assert_eq!(cursor.advance(), Token::new(TokenKind::Whitespace, " "));
        assert_eq!(
            cursor.advance(),
            Token::new(
                TokenKind::Literal {
                    kind: LiteralKind::Int {
                        base: Base::Decimal
                    }
                },
                "2"
            )
        );
        assert_eq!(cursor.advance(), Token::new(TokenKind::Whitespace, " "));
        assert_eq!(cursor.advance(), Token::new(TokenKind::ClosingBrace, "}"));
        assert_eq!(cursor.advance(), Token::new(TokenKind::Whitespace, " "));
        assert_eq!(
            cursor.advance(),
            Token::new(
                TokenKind::Literal {
                    kind: LiteralKind::Int {
                        base: Base::Decimal
                    }
                },
                "5"
            )
        );
        assert_eq!(cursor.advance(), Token::new(TokenKind::Semicolon, ";"));
    }

    #[test]
    fn works_with_whitespace() {
        let mut cursor = Cursor::new("\nsum   \n\n \r   sum # A comment\n");
        assert_eq!(cursor.advance(), Token::new(TokenKind::Whitespace, " "));
        assert_eq!(cursor.advance(), Token::new(TokenKind::Identifier, "sum"));
        assert_eq!(cursor.advance(), Token::new(TokenKind::Whitespace, " "));
        assert_eq!(cursor.advance(), Token::new(TokenKind::Identifier, "sum"));
        assert_eq!(cursor.advance(), Token::new(TokenKind::Whitespace, " "));
        assert_eq!(
            cursor.advance(),
            Token::new(TokenKind::InlineComment, " A comment")
        );
        assert_eq!(cursor.advance(), Token::new(TokenKind::Whitespace, " "));
    }
}
