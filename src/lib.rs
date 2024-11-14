pub mod builder;
mod designator;
mod lexer;
pub mod parser;
mod token;

#[cfg(test)]
mod tests {
    use super::builder;
    use super::lexer::*;
    use super::token::*;
    use crate::parser::Parser;

    #[test]
    fn test_builder() {
        let designators = vec![
            "R5".to_string(),
            "R1".to_string(),
            "(R2)".to_string(),
            "R3".to_string(),
            "(R1)".to_string(),
            "R6".to_string(),
            "R8".to_string(),
            "R2".to_string(),
            "(R3)".to_string(),
        ];

        let s = builder::build(designators);
        assert_eq!(
            r#"R1~3,5,6,8,
(R1~3)"#
                .to_string(),
            s
        );
    }

    #[test]
    fn test_parser() {
        // let input = r"R1-3,5,6,(C3)";
        // let mut parser = Parser::new(input);
        // let designators = parser.parse();
        // assert_eq!(designators.len(), 6);
        // let mut iter = designators.into_iter();
        // assert_eq!(iter.next(), Some("R1".to_string()));
        // assert_eq!(iter.next(), Some("R2".to_string()));
        // assert_eq!(iter.next(), Some("R3".to_string()));
        // assert_eq!(iter.next(), Some("R5".to_string()));
        // assert_eq!(iter.next(), Some("R6".to_string()));
        // assert_eq!(iter.next(), Some("(C3)".to_string()));
        // assert_eq!(iter.next(), None);

        let input = r"(IC1)";
        let mut parser = Parser::new(input);
        let designators = parser.parse();
        assert_eq!(designators.len(), 1);
    }

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("R1,R2 (R3),R4-9)");
        assert_eq!(
            lexer.token(),
            Some(TokenWithSymbol::new(Token::Identifier("R1".to_string())))
        );
        assert_eq!(lexer.token(), Some(TokenWithSymbol::new(Token::Comma)));
        assert_eq!(
            lexer.token(),
            Some(TokenWithSymbol::new(Token::Identifier("R2".to_string())))
        );
        assert_eq!(lexer.token(), Some(TokenWithSymbol::new(Token::Whitespace)));
        assert_eq!(lexer.token(), Some(TokenWithSymbol::new(Token::OpenParen)));
        assert_eq!(
            lexer.token(),
            Some(TokenWithSymbol::new(Token::Identifier("R3".to_string())))
        );
        assert_eq!(lexer.token(), Some(TokenWithSymbol::new(Token::CloseParen)));
        assert_eq!(lexer.token(), Some(TokenWithSymbol::new(Token::Comma)));
        assert_eq!(
            lexer.token(),
            Some(TokenWithSymbol::new(Token::Identifier("R4".to_string())))
        );
        assert_eq!(lexer.token(), Some(TokenWithSymbol::new(Token::Range('-'))));
        assert_eq!(
            lexer.token(),
            Some(TokenWithSymbol::new(Token::Identifier("9".to_string())))
        );
    }

    #[test]
    fn test_token() {
        // トークンのシンボルが正しいかのテスト
        assert_eq!(TokenWithSymbol::new(Token::Whitespace).symbol(), WHITESPACE);
        assert_eq!(TokenWithSymbol::new(Token::Comma).symbol(), COMMA);
        assert_eq!(
            TokenWithSymbol::new(Token::CloseParen).symbol(),
            CLOSE_PAREN
        );
        assert_eq!(TokenWithSymbol::new(Token::OpenParen).symbol(), OPEN_PAREN);
        assert_eq!(TokenWithSymbol::new(Token::Range('-')).symbol(), RANGE);
        assert_eq!(
            TokenWithSymbol::new(Token::Identifier("abc".to_string())).symbol(),
            IDENTIFIER
        );

        // 括弧で囲む
        let mut token = TokenWithSymbol::new(Token::Identifier("abc".to_string()));
        token.parenthesize();
        assert_eq!(token.symbol(), IDENTIFIER.to_ascii_uppercase());

        // 連結する
        assert_eq!(
            token.merge_token(&Token::Identifier("def".to_string())),
            Ok(())
        );
        assert_eq!(token.token().to_string(), "abcdef".to_string());
    }
}
