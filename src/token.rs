use super::token_type::TokenType;
use std::fmt::Display;

#[derive(Debug)]
pub struct Token {
    tok_type: TokenType,
    lexeme: String,
    literal: Object,
    line: u32,
}

impl Token {
    pub fn new(tok_type: TokenType, lexeme: String, literal: Object, line: u32) -> Token {
        Token {
            tok_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {})", self.tok_type, self.lexeme)
    }
}
