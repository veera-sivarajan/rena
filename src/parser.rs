use crate::err::LoxError;
use crate::expr::{BinaryExpr, Expr, NumberExpr, UnaryExpr, VariableExpr};
use crate::stmt::{Stmt, VarStmt, PrintStmt, ExpressionStmt};
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn is_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn consume(&mut self, token_type: TokenType,
               message: &str) -> Result<Token, LoxError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(LoxError::new(message.to_string()))
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, LoxError> {
        if self.type_match(vec![TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.type_match(vec![TokenType::Print]) {
            self.print_stmt()
        } else {
            self.expression_stmt()
        }
    }

    fn print_stmt(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect semicolon.")?;
        Ok(Stmt::Print(PrintStmt { expr: Box::new(expr) } ))
    }

    fn expression_stmt(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect semicolon.")?;
        Ok(Stmt::Expression(ExpressionStmt { expr: Box::new(expr) }))
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self.consume(TokenType::Identifier,"Expect variable name.")?;
        if self.type_match(vec![TokenType::Equal]) {
            let init = self.expression()?;
            self.consume(TokenType::Semicolon, "Expect semicolon.")?;
            Ok(Stmt::Var(VarStmt { name, init: Some(Box::new(init)) }))
        } else {
            self.consume(TokenType::Semicolon, "Expect semicolon.")?;
            Ok(Stmt::Var(VarStmt { name, init: None }))
        }
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;
        while self.type_match(vec![TokenType::BangEqual,
                                   TokenType::EqualEqual]) {
            let oper = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                oper,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;
        while self.type_match(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let oper = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                oper,
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
                oper,
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
                oper,
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
            Ok(Expr::Boolean(false))
        } else if self.type_match(vec![TokenType::True]) {
            Ok(Expr::Boolean(true))
        } else if self.type_match(vec![TokenType::Number]) {
            let num_str = self.previous().lexeme;
            // let num = num_str.parse::<f64>().unwrap();
            let num = num_str.parse::<f64>().expect("Cannot convert str to f64");
            Ok(Expr::Number(NumberExpr { value: num }))
        } else if self.type_match(vec![TokenType::StrLit]) {
            let str_lit = self.previous().lexeme;
            Ok(Expr::String(str_lit))
        } else if self.type_match(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            Ok(expr)
        } else if self.type_match(vec![TokenType::Identifier]) {
            Ok(Expr::Variable(VariableExpr { name: self.previous() }))
        } else {
            Err(LoxError::new("Expect expressions.".to_string()))
        }
    }
}
