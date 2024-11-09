mod lexer;
mod token;

#[cfg(test)]
mod tests {
    use super::token::*;

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

        // 変換結果のテスト
        let mut token = TokenWithSymbol::new(Token::Whitespace);
        assert_eq!(token.change_symbol(COMMA), Ok(()));
        assert_eq!(token.change_symbol(CLOSE_PAREN), Ok(()));
        assert_eq!(token.change_symbol(OPEN_PAREN), Ok(()));
        assert_eq!(token.change_symbol(RANGE), Ok(()));
        assert_eq!(token.change_symbol(IDENTIFIER), Ok(()));
        assert!(token.change_symbol('a').is_err());

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
