use crate::tokens::token_type::TokenType;

pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

pub enum Operator {
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Minus,
    Plus,
    Slash,
    Star,
}

impl TryInto<TokenType> for Operator {
    type Error = ();
    fn try_into(self) -> Result<TokenType, Self::Error> {
        match self {
            Operator::BangEqual => Ok(TokenType::BangEqual),
            Operator::Equal => Ok(TokenType::Equal),
            Operator::EqualEqual => Ok(TokenType::EqualEqual),
            Operator::Greater => Ok(TokenType::Greater),
            Operator::GreaterEqual => Ok(TokenType::GreaterEqual),
            Operator::Less => Ok(TokenType::Less),
            Operator::LessEqual => Ok(TokenType::LessEqual),
            Operator::Minus => Ok(TokenType::Minus),
            Operator::Plus => Ok(TokenType::Plus),
            Operator::Slash => Ok(TokenType::Slash),
            Operator::Star => Ok(TokenType::Star),
            _ => Err(()),
        }
    }
}

pub enum Expression {
    // Binary -> left operator right
    Binary(Box<Expression>, Operator, Box<Expression>),
    // Literal -> NUMBER | STRING | "true" | "false" | "nil"
    Literal(Literal),
}
