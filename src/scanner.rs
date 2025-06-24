use crate::tokens::{token::Token, token_type::TokenType};

use crate::utility::error::show_error;

use std::collections::HashMap;

pub struct Scanner {
    source: String,
    source_iter: Vec<char>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
    keyword_map: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let keyword_map = HashMap::from([
            ("and".to_owned(), TokenType::AND),
            ("class".to_owned(), TokenType::CLASS),
            ("else".to_owned(), TokenType::ELSE),
            ("false".to_owned(), TokenType::FALSE),
            ("for".to_owned(), TokenType::FOR),
            ("fun".to_owned(), TokenType::FUN),
            ("if".to_owned(), TokenType::IF),
            ("nil".to_owned(), TokenType::NIL),
            ("or".to_owned(), TokenType::OR),
            ("print".to_owned(), TokenType::PRINT),
            ("return".to_owned(), TokenType::RETURN),
            ("super".to_owned(), TokenType::SUPER),
            ("this".to_owned(), TokenType::THIS),
            ("true".to_owned(), TokenType::TRUE),
            ("var".to_owned(), TokenType::VAR),
            ("while".to_owned(), TokenType::WHILE),
        ]);

        Scanner {
            source_iter: source.chars().collect(),
            source,
            keyword_map,
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

    fn char_is_alnum(&self, c: char) -> bool {
        self.char_is_alpha(c) || self.char_is_digit(c)
    }

    fn char_is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn char_is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn check_curr(&mut self, expected: char) -> bool {
        if self.lookahead_1() == expected {
            self.advance();
            return true;
        }
        false
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

    fn lex_identifier(&mut self) {
        while self.char_is_alnum(self.lookahead_1()) {
            self.advance();
        }

        let text = self.source[self.start..self.current].to_owned();
        let text_type = self
            .keyword_map
            .get(&text)
            .unwrap_or(&TokenType::IDENTIFIER);
        self.add_token(text_type.clone(), None);
    }

    fn lex_number(&mut self) {
        while self.char_is_digit(self.lookahead_1()) {
            self.advance();
        }
        if self.lookahead_1() == '.' && self.char_is_digit(self.lookahead_2()) {
            self.advance();
            while self.char_is_digit(self.lookahead_1()) {
                self.advance();
            }
        }
        let literal = self.source[self.start..self.current]
            .to_owned()
            .parse::<f32>();
        match literal {
            Ok(_) => self.add_token(TokenType::NUMBER, Some(Box::new(literal))),
            Err(_) => show_error(self.line, "Invalid number literal somehow".to_owned()),
        }
    }

    fn lex_string(&mut self) {
        while !self.is_at_end() && self.lookahead_1() != '"' {
            if self.lookahead_1() == '\n' {
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

    fn lookahead_1(&self) -> char {
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
                    while !self.is_at_end() && self.lookahead_1() != '\n' {
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
                } else if self.char_is_alpha(c) {
                    self.lex_identifier();
                } else {
                    show_error(self.line, "Unexpected character".to_owned())
                }
            }
        }
        ()
    }
}
