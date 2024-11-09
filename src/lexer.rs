use super::token;
use crate::token::{Token, TokenWithSymbol};
use std::iter::Peekable;

pub struct Lexer {
    iter: Peekable<std::vec::IntoIter<char>>,
}

fn get_char_token(c: char) -> Option<TokenWithSymbol> {
    if c.is_whitespace() {
        return Some(TokenWithSymbol::new(Token::Whitespace));
    }

    // 範囲記号は '~' が基本だが、'-', '～' も許容する
    match c {
        token::COMMA => Some(TokenWithSymbol::new(Token::Comma)),
        token::CLOSE_PAREN => Some(TokenWithSymbol::new(Token::CloseParen)),
        token::OPEN_PAREN => Some(TokenWithSymbol::new(Token::OpenParen)),
        token::RANGE | '-' | '～' => Some(TokenWithSymbol::new(Token::Range(c))),
        _ => None,
    }
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Self {
            iter: input
                .trim()
                .chars()
                .collect::<Vec<char>>()
                .into_iter()
                .peekable(),
        }
    }

    pub fn token(&mut self) -> Option<TokenWithSymbol> {
        let mut ident = String::new();

        while let Some(c) = self.iter.next() {
            let tok = get_char_token(c);
            if tok.is_some() {
                return tok;
            }

            // トークンでないものは識別子の一部
            ident.push(c);

            // 次の文字がトークンの場合は識別子トークンを返す
            // 終端は確実にトークンとなるようにホワイトスペースを返す
            let c = *self.iter.peek().unwrap_or(&token::WHITESPACE);
            if get_char_token(c).is_some() {
                return Some(TokenWithSymbol::new(Token::Identifier(ident)));
            }
        }

        None
    }
}
