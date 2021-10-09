use crate::err::LoxError;
use crate::token::{Token, TokenType};
use crate::expr::{Expr, BinaryExpr, NumberExpr, UnaryExpr};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn is_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn consume(&mut self,
               token_type: TokenType, message: &str) -> Result<Token, LoxError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(LoxError::new("consume: token not found".to_string()))
        }
    }

    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn type_match(&mut self, types: Vec<TokenType>) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn advance(&mut self) -> Token {
        if !self.is_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn check(&self, t: TokenType) -> bool {
        if self.is_end() {
            return false;
        }
        self.peek().token_type == t
    }

    pub fn parse(&mut self) -> Result<Expr, LoxError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;
        while self.type_match(vec![TokenType::BangEqual,TokenType::EqualEqual]) {
            let oper = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                oper: oper,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;
        while self.type_match(vec![TokenType::Greater, TokenType::GreaterEqual,
                                   TokenType::Less, TokenType::LessEqual,]) {
            let oper = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                oper: oper,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;
        while self.type_match(vec![TokenType::Minus, TokenType::Plus]) {
            let oper = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                oper: oper,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;
        while self.type_match(vec![TokenType::Slash, TokenType::Star]) {
            let oper = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                oper: oper,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.type_match(vec![TokenType::Bang, TokenType::Minus]) {
            let oper = self.previous();
            let right = self.unary()?;
            Ok(Expr::Unary(UnaryExpr {
                oper,
                right: Box::new(right),
            }))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.type_match(vec![TokenType::False]) {
            return Ok(Expr::Boolean(false));
        } else if self.type_match(vec![TokenType::True]) {
            return Ok(Expr::Boolean(true));
        } else if self.type_match(vec![TokenType::Number]) {
            let num_str = self.previous().lexeme;
            let num = num_str.parse::<f64>().unwrap();
            return Ok(Expr::Number(NumberExpr{value: num}));
        } else if self.type_match(vec![TokenType::StrLit]) {
            let str_lit = self.previous().lexeme;
            return Ok(Expr::String(String::from(str_lit)));
        } else if self.type_match(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            return Ok(expr);
        }
            
        Err(LoxError::new("Error".to_string()))
    }
}

