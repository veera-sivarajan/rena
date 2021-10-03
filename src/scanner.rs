use crate::token::{Token, TokenType};
use std::str::FromStr;

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self
            .source
            .get(self.start..self.current)
            .expect("Source token is empty.");
        self.tokens.push(Token::new(token_type, text.to_string(),
                                    self.line));
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_end() ||
            self.source.chars().nth(self.current).unwrap() != expected {
                false
            } else {
                self.current += 1;
                true
            }
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            '/' => self.add_token(TokenType::Slash),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let new_type = if self.matches('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(new_type);
            },
            '=' => {
                let new_type = if self.matches('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(new_type);
            },
            '>' => {
                let new_type = if self.matches('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(new_type);
            },
            '<' => {
                let new_type = if self.matches('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(new_type);
            },
            ' ' | '\r' | '\t' => {},
            _  => {
                if c.is_digit(10) {
                    self.number()
                } else {
                    self.add_token(TokenType::Unknown)
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        let current_char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;

        current_char
    }

    fn peek(&self) -> char {
        if self.is_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        let sub_string = &self.source[self.start..self.current];
        let num = f64::from_str(sub_string).unwrap();
        self.add_token(TokenType::Number(num));
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.add_token(TokenType::EOF);
        self.tokens.clone()
    }

}
            
            
