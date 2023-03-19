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
    Until,
<<<<<<< HEAD
    MVar,
=======
>>>>>>> closure
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, line: i32) -> Token {
        Token {
            token_type,
            lexeme: lexeme.to_owned(),
            line,
        }
    }
}
