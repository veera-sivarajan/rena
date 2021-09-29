pub enum TokenType {
    LeftParen, RightParen, Dot, Minus, Plus, Slash, Star,
    Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual,
    Less, LessEqual,

    Number(f64), True, False,
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: i32,
}

impl Token {
    
