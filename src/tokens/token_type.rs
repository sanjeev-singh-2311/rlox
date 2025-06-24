#![allow(non_camel_case_types)]

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,
    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF,
}

// Clone trait to copy the values out of the HashMap, dont really think I need more than these so I
// just bound teh default match to Identifier.. but we'll see
impl Clone for TokenType {
    fn clone(&self) -> Self {
        match self {
            TokenType::AND => TokenType::AND,
            TokenType::CLASS => TokenType::CLASS,
            TokenType::ELSE => TokenType::ELSE,
            TokenType::FALSE => TokenType::FALSE,
            TokenType::FUN => TokenType::FUN,
            TokenType::FOR => TokenType::FOR,
            TokenType::IF => TokenType::IF,
            TokenType::NIL => TokenType::NIL,
            TokenType::OR => TokenType::OR,
            TokenType::PRINT => TokenType::PRINT,
            TokenType::RETURN => TokenType::RETURN,
            TokenType::SUPER => TokenType::SUPER,
            TokenType::THIS => TokenType::THIS,
            TokenType::TRUE => TokenType::TRUE,
            TokenType::VAR => TokenType::VAR,
            TokenType::WHILE => TokenType::WHILE,
            TokenType::EOF => TokenType::EOF,
            _ => TokenType::IDENTIFIER,
        }
    }
}
