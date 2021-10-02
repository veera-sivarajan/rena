use crate::token;

pub struct Scanner {
    source: String,
    tokens: Vec<token::Token>,
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

    fn advance(&mut self) -> char {
        self.current += 1;
        let char_vec: Vec<char> = self.source.chars().collect();
        char_vec[self.current - 1]
    }

    fn add_token(&mut self, token_type: token::TokenType) {
        let text = self
            .source
            .get(self.start..self.current)
            .expect("Source token is empty.");
        self.tokens.push(token::Token::new(token_type, text.to_string(),
                                           self.line));
    }

    pub fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(token::TokenType::LeftParen),
            ')' => self.add_token(token::TokenType::RightParen),
            '.' => self.add_token(token::TokenType::Dot),
            '-' => self.add_token(token::TokenType::Minus),
            '+' => self.add_token(token::TokenType::Plus),
            '*' => self.add_token(token::TokenType::Star),
            _   => println!("Unrecognized character"),
        }
    }
}
            
            
