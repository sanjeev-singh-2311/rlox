use crate::tokens::{token::Token, token_type::TokenType};

use crate::utility::error::show_error;

pub struct Scanner {
    source: String,
    source_iter: Vec<char>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source_iter: source.chars().collect(),
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        if self.current == self.source_iter.len() {
            return true;
        }
        let c = self.source_iter[self.current];
        c == '\0'
    }

    pub fn scan_all_tokens(&mut self) -> () {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_owned(),
            Box::<Option<i32>>::new(None),
            self.line,
        ));
        ()
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '\n' => self.line += 1,
            _ => show_error(self.line, "Unexpected character".to_owned()),
        }
        ()
    }

    fn add_token(&mut self, tok_type: TokenType) {
        self.add_token_in_vec(tok_type, Box::<Option<i32>>::new(None))
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source_iter[self.current - 1]
    }

    fn add_token_in_vec(&mut self, token_type: TokenType, literal: Box<dyn std::any::Any>) {
        let substr = self.source[self.start..self.current].to_owned();
        self.tokens
            .push(Token::new(token_type, substr, literal, self.line));
    }
}
