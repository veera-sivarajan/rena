use crate::err::LoxError;
use crate::expr::{AssignExpr, BinaryExpr, Expr,
                  GroupExpr, NumberExpr, UnaryExpr, VariableExpr};
use crate::stmt::{BlockStmt, ExpressionStmt, PrintStmt, Stmt, VarStmt};
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

macro_rules! matches {
    ( $parser:ident, $( $x:expr ),* ) => {
        {
            if $( $parser.check($x) ) || * {
                $parser.advance();
                true
            } else {
                false
            }
        }
    };
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
        if matches!(self, TokenType::Var) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;
        if matches!(self, TokenType::Equal) {
            let init = self.expression()?;
            self.consume(TokenType::Semicolon, "Expect semicolon.")?;
            Ok(Stmt::Var(VarStmt {
                name,
                init: Some(Box::new(init)),
            }))
        } else {
            self.consume(TokenType::Semicolon, "Expect semicolon.")?;
            Ok(Stmt::Var(VarStmt { name, init: None }))
        }
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if matches!(self, TokenType::Print) {
            self.print_stmt()
        } else if matches!(self, TokenType::LeftBrace) {
            self.block_stmt()
        } else {
            self.expression_stmt()
        }
    }

    fn block_stmt(&mut self) -> Result<Stmt, LoxError> {
        let mut statements = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_end() {
            statements.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(Stmt::Block(BlockStmt { statements }))
    }

    fn print_stmt(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect semicolon.")?;
        Ok(Stmt::Print(PrintStmt {
            expr: Box::new(expr),
        }))
    }

    fn expression_stmt(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect semicolon.")?;
        Ok(Stmt::Expression(ExpressionStmt {
            expr: Box::new(expr),
        }))
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.equality()?;
        if matches!(self, TokenType::Equal) {
            let _equals = self.previous();
            let value = self.assignment()?;
            match expr {
                Expr::Variable(expr) => Ok(Expr::Assign(AssignExpr {
                    name: expr.name,
                    value: Box::new(value),
                })),
                _ => error!("Invalid assignment target."),
            }
        } else {
            Ok(expr)
        }
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;
        while matches!(self, TokenType::BangEqual, TokenType::EqualEqual) {
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
        while matches!(
            self,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual
        ) {
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
        while matches!(self, TokenType::Minus, TokenType::Plus) {
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
        while matches!(self, TokenType::Slash, TokenType::Star) {
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
        if matches!(self, TokenType::Bang, TokenType::Minus) {
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
        if matches!(self, TokenType::Nil) {
            Ok(Expr::Nil)
        } else if matches!(self, TokenType::False) {
            Ok(Expr::Boolean(false))
        } else if matches!(self, TokenType::True) {
            Ok(Expr::Boolean(true))
        } else if matches!(self, TokenType::Number) {
            let num_str = self.previous().lexeme;
            let num = num_str.parse::<f64>().expect("Cannot convert str to f64");
            Ok(Expr::Number(NumberExpr { value: num }))
        } else if matches!(self, TokenType::StrLit) {
            let str_lit = self.previous().lexeme;
            Ok(Expr::String(str_lit))
        } else if matches!(self, TokenType::LeftParen) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            Ok(Expr::Group(GroupExpr {
                expr: Box::new(expr),
            }))
        } else if matches!(self, TokenType::Identifier) {
            Ok(Expr::Variable(VariableExpr {
                name: self.previous(),
            }))
        } else {
            error!("Expect expressions.")
        }
    }
}
