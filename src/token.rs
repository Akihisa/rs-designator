use std::fmt;
use std::fmt::Write;

pub(crate) const WHITESPACE: char = ' ';
pub(crate) const COMMA: char = ',';
pub(crate) const CLOSE_PAREN: char = ')';
pub(crate) const OPEN_PAREN: char = '(';
pub(crate) const RANGE: char = '~';
pub(crate) const IDENTIFIER: char = 'i';
pub(crate) const SKIP: char = '>';

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Whitespace,
    Comma,
    CloseParen,
    OpenParen,
    Range(char),
    Identifier(String),
}

pub fn get_token_symbol(token: &Token) -> char {
    match token {
        Token::Whitespace => WHITESPACE,
        Token::Comma => COMMA,
        Token::CloseParen => CLOSE_PAREN,
        Token::OpenParen => OPEN_PAREN,
        Token::Range(_) => RANGE,
        Token::Identifier(_) => IDENTIFIER,
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Whitespace => f.write_char(WHITESPACE)?,
            Token::Comma => f.write_char(COMMA)?,
            Token::CloseParen => f.write_char(CLOSE_PAREN)?,
            Token::OpenParen => f.write_char(OPEN_PAREN)?,
            Token::Range(c) => f.write_char(*c)?,
            Token::Identifier(ident) => f.write_str(ident)?,
        }

        Ok(())
    }
}

#[derive(PartialEq, Debug)]
pub struct TokenWithSymbol {
    symbol: char,
    token: Token,
}

impl TokenWithSymbol {
    pub fn new(token: Token) -> Self {
        Self {
            symbol: get_token_symbol(&token),
            token,
        }
    }

    pub fn symbol(&self) -> char {
        self.symbol
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn is_whitespace(&self) -> bool {
        self.symbol.is_whitespace()
    }

    pub fn is_comma(&self) -> bool {
        self.symbol == COMMA
    }

    pub fn is_close_paren(&self) -> bool {
        self.symbol == CLOSE_PAREN
    }

    pub fn is_open_paren(&self) -> bool {
        self.symbol == OPEN_PAREN
    }

    pub fn is_range(&self) -> bool {
        self.symbol == RANGE
    }

    pub fn is_identifier(&self) -> bool {
        self.symbol.to_ascii_lowercase() == IDENTIFIER
    }

    pub fn convert_symbol_to_whitespace(&mut self) {
        self.symbol = WHITESPACE;
    }

    pub fn convert_symbol_to_comma(&mut self) {
        self.symbol = COMMA;
    }

    pub fn convert_symbol_to_identifier(&mut self) {
        // 大文字に変換されている場合も考慮して、自身が識別子でないときにのみ実行する
        if !self.is_identifier() {
            self.symbol = IDENTIFIER;
        }
    }

    pub fn change_symbol(&mut self, symbol: char) -> Result<(), &'static str> {
        match symbol.to_ascii_lowercase() {
            COMMA => (),
            CLOSE_PAREN => (),
            OPEN_PAREN => (),
            RANGE => (),
            IDENTIFIER => (),
            c if c.is_whitespace() => (),
            _ => return Err("invalid token symbol"),
        }

        self.symbol = symbol;

        Ok(())
    }

    pub fn parenthesize(&mut self) {
        self.symbol.make_ascii_uppercase();
    }

    pub fn transform(&mut self) {
        if !self
            .symbol
            .eq_ignore_ascii_case(&get_token_symbol(&self.token))
        {
            match self.symbol.to_ascii_lowercase() {
                WHITESPACE => self.token = Token::Whitespace,
                COMMA => self.token = Token::Comma,
                CLOSE_PAREN => self.token = Token::CloseParen,
                OPEN_PAREN => self.token = Token::OpenParen,
                RANGE => self.token = Token::Range(RANGE),
                IDENTIFIER => self.token = Token::Identifier(self.token.to_string()),
                _ => unreachable!(),
            }
        }
    }

    pub fn merge_token(&mut self, other: &Token) -> Result<(), &'static str> {
        let Token::Identifier(ident) = &mut self.token else {
            return Err("only identifiers can be merged");
        };

        ident.push_str(other.to_string().as_str());

        Ok(())
    }
}
