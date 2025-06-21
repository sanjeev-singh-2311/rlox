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
        // DOUBT: should the program error out on a NULL character, read and tokenize it, or stop
        // reading and ignore things after it???
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
            '(' => self.add_token(TokenType::LEFT_PAREN, None),
            ')' => self.add_token(TokenType::RIGHT_PAREN, None),
            '{' => self.add_token(TokenType::LEFT_BRACE, None),
            '}' => self.add_token(TokenType::RIGHT_BRACE, None),
            ',' => self.add_token(TokenType::COMMA, None),
            '.' => self.add_token(TokenType::DOT, None),
            '-' => self.add_token(TokenType::MINUS, None),
            '+' => self.add_token(TokenType::PLUS, None),
            ';' => self.add_token(TokenType::SEMICOLON, None),
            '*' => self.add_token(TokenType::STAR, None),
            '/' => {
                if self.check_curr('/') {
                    while !self.is_at_end() && self.lookahead() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH, None)
                }
            }
            '"' => self.lex_string(),

            ' ' | '\t' | '\r' => (), // Early Return
            '\n' => self.line += 1,
            _ => show_error(self.line, "Unexpected character".to_owned()),
        }
        ()
    }

    fn add_token(&mut self, tok_type: TokenType, literal: Option<Box<dyn std::any::Any>>) {
        match literal {
            Some(lit) => self.add_token_in_vec(tok_type, lit),
            None => self.add_token_in_vec(tok_type, Box::<Option<i8>>::new(None)),
        }
    }

    fn advance(&mut self) -> char {
        let local_current = self.current;
        self.current += 1;
        self.source_iter[local_current]
    }

    fn lookahead(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source_iter[self.current]
    }

    fn add_token_in_vec(&mut self, token_type: TokenType, literal: Box<dyn std::any::Any>) {
        let substr = self.source[self.start..self.current].to_owned();
        self.tokens
            .push(Token::new(token_type, substr, literal, self.line));
    }

    fn check_curr(&mut self, expected: char) -> bool {
        if self.lookahead() == expected {
            self.advance();
            return true;
        }
        false
    }
}
