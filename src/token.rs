#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {
    LeftParen, RightParen, Dot, Minus, Plus, Slash, Star, Semicolon, EOF,

    Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Less, LessEqual,

    Number, True, False, Unknown, Identifier, StrLit,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: i32) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

impl TokenType {
    pub fn _to_string(&self) -> String {
        match self {
            TokenType::LeftParen => String::from("LeftParen"),
            TokenType::RightParen => String::from("RightParen"),
            TokenType::Dot => String::from("Dot"),
            TokenType::Minus => String::from("Minus"),
            TokenType::Plus => String::from("Plus"),
            TokenType::Slash => String::from("Slash"),
            TokenType::Star => String::from("Star"),
            TokenType::Semicolon => String::from("Semicolon"),
            TokenType::Bang => String::from("Bang"),
            TokenType::BangEqual => String::from("BangEqual"),
            TokenType::Equal => String::from("Equal"),
            TokenType::EqualEqual => String::from("EqualEqual"),
            TokenType::Greater => String::from("Greater"),
            TokenType::GreaterEqual => String::from("GreaterEqual"),
            TokenType::Less => String::from("Less"),
            TokenType::LessEqual => String::from("LessEqual"),
            TokenType::Number => format!("Number"),
            TokenType::True => String::from("True"),
            TokenType::False => String::from("False"),
            TokenType::EOF => String::from("EOF"),
            TokenType::Unknown => String::from("Unknown"),
            TokenType::Identifier => String::from("Identifier"),
            TokenType::StrLit => format!("String"),
        }
    }
}
