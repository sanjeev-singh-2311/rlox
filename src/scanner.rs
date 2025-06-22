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

    fn add_token(&mut self, tok_type: TokenType, literal: Option<Box<dyn std::any::Any>>) {
        match literal {
            Some(lit) => self.add_token_in_vec(tok_type, lit),
            None => self.add_token_in_vec(tok_type, Box::<Option<i8>>::new(None)),
        }
    }

    fn add_token_in_vec(&mut self, token_type: TokenType, literal: Box<dyn std::any::Any>) {
        let substr = self.source[self.start..self.current].to_owned();
        self.tokens
            .push(Token::new(token_type, substr, literal, self.line));
    }

    fn advance(&mut self) -> char {
        let local_current = self.current;
        self.current += 1;
        self.source_iter[local_current]
    }

    fn char_is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn check_curr(&mut self, expected: char) -> bool {
        if self.lookahead() == expected {
            self.advance();
            return true;
        }
        false
    }

    fn lex_number(&mut self) {
        while self.char_is_digit(self.lookahead()) {
            self.advance();
        }
        if self.lookahead() == '.' && self.char_is_digit(self.lookahead_2()) {
            self.advance();
            while self.char_is_digit(self.lookahead()) {
                self.advance();
            }
        }
        let literal = self.source[self.start..self.current]
            .to_owned()
            .parse::<f32>();
        match literal {
            Ok(n) => self.add_token(TokenType::NUMBER, Some(Box::new(literal))),
            Err(_) => show_error(self.line, "Invalid number literal somehow".to_owned()),
        }
    }

    fn lex_string(&mut self) {
        while !self.is_at_end() && self.lookahead() != '"' {
            if self.lookahead() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            show_error(self.line, "Unterminated string".to_owned());
        }
        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_owned();
        self.add_token(TokenType::STRING, Some(Box::new(value)));
    }

    fn lookahead(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source_iter[self.current]
    }

    fn lookahead_2(&self) -> char {
        if self.current + 1 >= self.source_iter.len() {
            return '\0';
        }
        self.source_iter[self.current + 1]
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
            '<' => {
                if self.check_curr('=') {
                    self.add_token(TokenType::LESS_EQUAL, None);
                } else {
                    self.add_token(TokenType::LESS, None);
                }
            }
            '>' => {
                if self.check_curr('=') {
                    self.add_token(TokenType::GREATER_EQUAL, None);
                } else {
                    self.add_token(TokenType::GREATER, None);
                }
            }
            '=' => {
                if self.check_curr('=') {
                    self.add_token(TokenType::EQUAL_EQUAL, None);
                } else {
                    self.add_token(TokenType::EQUAL, None);
                }
            }
            '!' => {
                if self.check_curr('=') {
                    self.add_token(TokenType::BANG_EQUAL, None);
                } else {
                    self.add_token(TokenType::BANG, None);
                }
            }

            '"' => self.lex_string(),

            ' ' | '\t' | '\r' => (), // Early Return
            '\n' => self.line += 1,
            _ => {
                if self.char_is_digit(c) {
                    self.lex_number()
                } else {
                    show_error(self.line, "Unexpected character".to_owned())
                }
            }
        }
        ()
    }
}
