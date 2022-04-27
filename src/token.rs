#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Dot,
    Minus,
    Plus,
    Slash,
    Star,
    Semicolon,
    Eof,
    LeftBrace,
    RightBrace,
    Comma,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Number,
    True,
    False,
    Unknown,
    Identifier,
    StrLit,
    Print,
    Var,
    Nil,
    If,
    Else,
    While,
    For,
    Fun,
    Return,
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
            TokenType::Number => "Number".to_string(),
            TokenType::True => String::from("True"),
            TokenType::False => String::from("False"),
            TokenType::Eof => String::from("EOF"),
            TokenType::Unknown => String::from("Unknown"),
            TokenType::Identifier => String::from("Identifier"),
            TokenType::StrLit => "String".to_string(),
            TokenType::Print => String::from("Print"),
            TokenType::Var => String::from("Var"),
            TokenType::LeftBrace => String::from("Leftbrace"),
            TokenType::RightBrace => String::from("Rightbrace"),
            TokenType::Nil => String::from("Nil"),
            TokenType::If => String::from("If"),
            TokenType::Else => String::from("Else"),
            TokenType::While => String::from("While"),
            TokenType::For => String::from("For"),
            TokenType::Comma => String::from("Comma"),
            TokenType::Fun => String::from("Fun"),
            TokenType::Return => String::from("Return"),
        }
    }
}
