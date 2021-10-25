use crate::token::{Token, TokenType};
use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::err::LoxError;

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
}

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut hash_map = HashMap::new();
        hash_map.insert("true".to_owned(), TokenType::True);
        hash_map.insert("false".to_owned(), TokenType::False);
        hash_map.insert("print".to_owned(), TokenType::Print);
        hash_map.insert("var".to_owned(), TokenType::Var);
        hash_map
    };
}

fn is_alphanumeric(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
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

    fn add_token(&mut self, token_type: TokenType) -> Result<(), LoxError> {
        let text = self
            .source
            .get(self.start..self.current)
            .expect("Source token is empty.");
        self.tokens.push(Token::new(token_type, text.to_string(),
                                    self.line));
        Ok(())
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

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            '/' => self.add_token(TokenType::Slash),
            '*' => self.add_token(TokenType::Star),
            ';' => self.add_token(TokenType::Semicolon),
            '!' => {
                let new_type = if self.matches('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(new_type)
            },
            '=' => {
                let new_type = if self.matches('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(new_type)
            },
            '>' => {
                let new_type = if self.matches('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(new_type)
            },
            '<' => {
                let new_type = if self.matches('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(new_type)
            },
            ' ' | '\r' | '\t' => Ok(()), // skip whitespaces, tab and enter?
            '"' => self.scan_string(),
             _  => {
                 if c.is_digit(10) {
                     self.number()
                 } else if is_alphanumeric(c) {
                     self.identifier()
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

    fn number(&mut self) -> Result<(), LoxError> {
        while self.peek().is_digit(10) {
            self.advance();
        }

        self.add_token(TokenType::Number)
    }

    fn scan_string(&mut self) -> Result<(), LoxError> {
        while self.peek() != '"' && !self.is_end() {
            self.advance();
        }
        if !self.is_end() {
            self.advance();
        } else {
            return Err(LoxError::new(String::from("Unterminated string.")));
        }
        self.add_token(TokenType::StrLit)
    }

    fn identifier(&mut self) -> Result<(), LoxError> {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }
        let sub_string = &self.source[self.start..self.current];
        let token_type = {
            match KEYWORDS.get(sub_string) {
                None => TokenType::Identifier,
                Some(t_type) => t_type.clone(),
            }
        };
       
        self.add_token(token_type)
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LoxError> {
        while !self.is_end() {
            self.start = self.current;
            self.scan_token()?
        }
        
        self.add_token(TokenType::EOF)?;
        Ok(self.tokens.clone())
    }
}
            
            
