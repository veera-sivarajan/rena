use crate::token::{Token, TokenType};

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

    fn advance(&mut self) -> char {
        self.current += 1;
        let char_vec: Vec<char> = self.source.chars().collect();
        char_vec[self.current - 1]
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self
            .source
            .get(self.start..self.current)
            .expect("Source token is empty.");
        self.tokens.push(Token::new(token_type, text.to_string(),
                                    self.line));
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
             _  => println!("Unrecognized character"),
        }
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
            
            
