use crate::err::LoxError;
use crate::expr::{
    AssignExpr, BinaryExpr, CallExpr, Expr, GroupExpr, NumberExpr,
    UnaryExpr, VariableExpr,
};
use crate::stmt::{
<<<<<<< HEAD
    BlockStmt, ExpressionStmt, IfStmt, PrintStmt, Stmt, VarStmt,
    WhileStmt, FunStmt, ReturnStmt, MVarStmt,
=======
    BlockStmt, ExpressionStmt, FunStmt, IfStmt, PrintStmt, ReturnStmt,
    Stmt, VarStmt, WhileStmt,
>>>>>>> closure
};
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

    fn consume(
        &mut self,
        token_type: TokenType,
        message: &str,
    ) -> Result<Token, LoxError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            error!(message)
        }
    }

    fn previous(&self) -> Token {
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
        } else if matches!(self, TokenType::MVar) {
            self.mvar()
        } else if matches!(self, TokenType::Fun) {
            self.function()
        } else {
            self.statement()
        }
    }

    // add(1, 2, 3)
    // mvar a, b, c = 1, 2, 3;
    fn mvar(&mut self) -> Result<Stmt, LoxError> {
        let mut names = vec![];
        while !self.check(TokenType::Equal) {
            names.push(self.consume(TokenType::Identifier, "Expect var name.")?);
            if !self.check(TokenType::Equal) {
                self.consume(TokenType::Comma, "Expect comma between names.")?;
            }
        }
        self.consume(TokenType::Equal, "Expect `=`")?;
        let mut values = vec![];
        while !self.check(TokenType::Semicolon) {
            values.push(self.expression()?);
            if !self.check(TokenType::Semicolon) {
                self.consume(TokenType::Comma, "Expect comma between values.")?;
            }
        }
        self.consume(TokenType::Semicolon, "Expect semicolon.")?;
        Ok(Stmt::MVar(MVarStmt { names, values }))
    }

    fn function(&mut self) -> Result<Stmt, LoxError> {
        let name =
            self.consume(TokenType::Identifier, "Expect function name.")?;
        self.consume(
            TokenType::LeftParen,
            "Expect '(' after function name.",
        )?;
        // parse all parameters
        let mut params: Vec<Token> = vec![];
        if !self.check(TokenType::RightParen) {
            params.push(self.consume(
                TokenType::Identifier,
                "Expect parameter name.",
            )?);
            while matches!(self, TokenType::Comma) {
                if params.len() < 255 {
                    params.push(self.consume(
                        TokenType::Identifier,
                        "Expect parameter name.",
                    )?);
                } else {
                    return error!("Can't have more than 255 parameters.");
                }
            }
        }
        self.consume(
            TokenType::RightParen,
            "Expect ')' after parameters.",
        )?;

        self.consume(
            TokenType::LeftBrace,
            "Expect '{' before function body",
        )?;
        let body = self.block_stmt()?;
        Ok(Stmt::Function(FunStmt { name, params, body }))
    }

    // var a = 5;
    // var b;
    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name =
            self.consume(TokenType::Identifier, "Expect variable name.")?;
        if matches!(self, TokenType::Equal) {
            let init = self.expression()?;
            self.consume(TokenType::Semicolon, "Expect semicolon.")?;
            Ok(Stmt::Var(VarStmt {
                name,
                init: Some(init),
            }))
        }

        else {
            self.consume(TokenType::Semicolon, "Expect semicolon.")?;
            Ok(Stmt::Var(VarStmt { name, init: None }))
        }
    }

    // entry point for parsing statements
    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if matches!(self, TokenType::Print) {
            self.print_stmt()
        } else if matches!(self, TokenType::LeftBrace) {
            let statements = self.block_stmt()?;
            Ok(Stmt::Block(BlockStmt { statements }))
        } else if matches!(self, TokenType::If) {
            self.if_stmt()
        } else if matches!(self, TokenType::While) {
            self.while_stmt()
        } else if matches!(self, TokenType::For) {
            self.for_stmt()
        } else if matches!(self, TokenType::Return) {
            self.return_stmt()
        } else {
            self.expression_stmt()
        }
    }

    fn return_stmt(&mut self) -> Result<Stmt, LoxError> {
        let keyword = self.previous();
        let mut value = None;
        if !self.check(TokenType::Semicolon) {
            value = Some(self.expression()?);
        }
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after return value.",
        )?;
        Ok(Stmt::Return(ReturnStmt { keyword, value }))
    }

    fn for_stmt(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let init;
        if matches!(self, TokenType::Semicolon) {
            init = None;
        } else if matches!(self, TokenType::Var) {
            init = Some(self.var_declaration()?);
        } else {
            init = Some(self.expression_stmt()?);
        }

        let mut condition = None;
        if !self.check(TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after loop condition.",
        )?;

        let mut increment = None;
        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(
            TokenType::RightParen,
            "Expect ')' after for clauses.",
        )?;
        let mut body = self.statement()?;
        if let Some(increment_expression) = increment {
            let increment_stmt = Stmt::Expression(ExpressionStmt {
                expr: increment_expression,
            });
            body = Stmt::Block(BlockStmt {
                statements: vec![body, increment_stmt],
            });
        }
        body = Stmt::While(WhileStmt {
            condition: condition.unwrap_or(Expr::Boolean(true)),
            body: Box::new(body),
        });
        if let Some(init_statement) = init {
            body = Stmt::Block(BlockStmt {
                statements: vec![init_statement, body],
            });
        }
        Ok(body)
    }

    fn while_stmt(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "Expect ')' after condition.",
        )?;
        let body = self.statement()?;
        Ok(Stmt::While(WhileStmt {
            condition,
            body: Box::new(body),
        }))
    }

    
// if (1 > 4) {
//    ...
// } else {
//     ...
// }

    fn if_stmt(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "Expect ')' after condition.",
        )?;
        let then_branch = self.statement()?;
        let mut else_branch = None;
        if matches!(self, TokenType::Else) {
            else_branch = Some(Box::new(self.statement()?));
        }
        let if_node = IfStmt {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        };
        Ok(Stmt::If(if_node))
    }

    fn block_stmt(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_end() {
            statements.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn print_stmt(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect semicolon.")?;
        Ok(Stmt::Print(PrintStmt { expr }))
    }

    fn expression_stmt(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect semicolon.")?;
        Ok(Stmt::Expression(ExpressionStmt { expr }))
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        // println!("assignment()");
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
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.primary()?;
        loop {
            if matches!(self, TokenType::LeftParen) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, LoxError> {
        let mut args: Vec<Expr> = vec![];
        if !self.check(TokenType::RightParen) {
            args.push(self.expression()?); // parse first argument
            while matches!(self, TokenType::Comma) {
                if args.len() >= 255 {
                    return error!("Can't have more than 255 arguments.");
                };
                args.push(self.expression()?);
            }
        }
        let paren = self.consume(
            TokenType::RightParen,
            "Expect ')' after arguments.",
        )?;
        Ok(Expr::Call(CallExpr {
            callee: Box::new(callee),
            paren,
            args,
        }))
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
            let num =
                num_str.parse::<f64>().expect("Cannot convert str to f64");
            Ok(Expr::Number(NumberExpr { value: num }))
        } else if matches!(self, TokenType::StrLit) {
            let str_lit = self.previous().lexeme;
            Ok(Expr::String(str_lit))
        } else if matches!(self, TokenType::LeftParen) {
            let expr = self.expression()?;
            self.consume(
                TokenType::RightParen,
                "Expect ')' after expression",
            )?;
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
