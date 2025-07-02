#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    String,
    Number,
    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

// Clone trait to copy the values out of the HashMap, dont really think I need more than these so I
// just bound teh default match to Identifier.. but we'll see
impl Clone for TokenType {
    fn clone(&self) -> Self {
        match self {
            TokenType::And => TokenType::And,
            TokenType::Class => TokenType::Class,
            TokenType::Else => TokenType::Else,
            TokenType::False => TokenType::False,
            TokenType::Fun => TokenType::Fun,
            TokenType::For => TokenType::For,
            TokenType::If => TokenType::If,
            TokenType::Nil => TokenType::Nil,
            TokenType::Or => TokenType::Or,
            TokenType::Print => TokenType::Print,
            TokenType::Return => TokenType::Return,
            TokenType::Super => TokenType::Super,
            TokenType::This => TokenType::This,
            TokenType::True => TokenType::True,
            TokenType::Var => TokenType::Var,
            TokenType::While => TokenType::While,
            TokenType::Eof => TokenType::Eof,
            _ => TokenType::Identifier,
        }
    }
}
