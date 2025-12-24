use crate::ast::*;
use crate::error::{CompilerError, Result};
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<(Token, usize)>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<(Token, usize)>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.pos)
            .map(|(t, _)| t)
            .unwrap_or(&Token::Eof)
    }

    fn peek_line(&self) -> usize {
        self.tokens.get(self.pos).map(|(_, l)| *l).unwrap_or(0)
    }

    fn advance(&mut self) -> Token {
        let tok = self
            .tokens
            .get(self.pos)
            .map(|(t, _)| t.clone())
            .unwrap_or(Token::Eof);
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: Token) -> Result<()> {
        let line = self.peek_line();
        let tok = self.advance();
        if tok != expected {
            return Err(CompilerError::parser(
                line,
                format!("Expected {:?}, got {:?}", expected, tok),
            ));
        }
        Ok(())
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        let mut functions = Vec::new();
        let mut top_level = Vec::new();

        while *self.peek() != Token::Eof {
            if *self.peek() == Token::Function {
                functions.push(self.parse_function()?);
            } else {
                top_level.push(self.parse_statement()?);
            }
        }

        Ok(Program {
            functions,
            top_level,
        })
    }

    fn parse_function(&mut self) -> Result<Function> {
        let line = self.peek_line();
        self.expect(Token::Function)?;
        let name = match self.advance() {
            Token::Identifier(s) => s,
            t => return Err(CompilerError::parser(line, format!("Expected function name, got {:?}", t))),
        };
        self.expect(Token::LParen)?;

        let mut params = Vec::new();
        if *self.peek() != Token::RParen {
            loop {
                match self.advance() {
                    Token::Identifier(s) => params.push(s),
                    t => return Err(CompilerError::parser(self.peek_line(), format!("Expected parameter name, got {:?}", t))),
                }
                if *self.peek() == Token::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.expect(Token::RParen)?;
        self.expect(Token::LBrace)?;

        let mut body = Vec::new();
        while *self.peek() != Token::RBrace {
            body.push(self.parse_statement()?);
        }
        self.expect(Token::RBrace)?;

        Ok(Function {
            name,
            params,
            body,
            line,
        })
    }

    fn parse_statement(&mut self) -> Result<Stmt> {
        let line = self.peek_line();
        let kind = match self.peek() {
            Token::Let => {
                self.advance();
                let name = match self.advance() {
                    Token::Identifier(s) => s,
                    t => return Err(CompilerError::parser(line, format!("Expected identifier, got {:?}", t))),
                };
                self.expect(Token::Eq)?;
                let expr = self.parse_expr()?;
                self.expect(Token::Semicolon)?;
                StmtKind::Let(name, expr)
            }
            Token::Const => {
                self.advance();
                let name = match self.advance() {
                    Token::Identifier(s) => s,
                    t => return Err(CompilerError::parser(line, format!("Expected identifier, got {:?}", t))),
                };
                self.expect(Token::Eq)?;
                let expr = self.parse_expr()?;
                self.expect(Token::Semicolon)?;
                StmtKind::Const(name, expr)
            }
            Token::If => {
                self.advance();
                self.expect(Token::LParen)?;
                let cond = self.parse_expr()?;
                self.expect(Token::RParen)?;
                let then_branch = Box::new(self.parse_statement()?);
                let else_branch = if *self.peek() == Token::Else {
                    self.advance();
                    Some(Box::new(self.parse_statement()?))
                } else {
                    None
                };
                StmtKind::If(cond, then_branch, else_branch)
            }
            Token::While => {
                self.advance();
                self.expect(Token::LParen)?;
                let cond = self.parse_expr()?;
                self.expect(Token::RParen)?;
                let body = Box::new(self.parse_statement()?);
                StmtKind::While(cond, body)
            }
            Token::For => {
                self.advance();
                self.expect(Token::LParen)?;

                let init = if *self.peek() == Token::Semicolon {
                    self.advance();
                    None
                } else {
                    let init_stmt = if *self.peek() == Token::Let {
                        self.advance();
                        let name = match self.advance() {
                            Token::Identifier(s) => s,
                            t => return Err(CompilerError::parser(line, format!("Expected identifier, got {:?}", t))),
                        };
                        self.expect(Token::Eq)?;
                        let expr = self.parse_expr()?;
                        self.expect(Token::Semicolon)?;
                        Stmt {
                            kind: StmtKind::Let(name, expr),
                            line: self.peek_line(),
                        }
                    } else if *self.peek() == Token::Const {
                        self.advance();
                        let name = match self.advance() {
                            Token::Identifier(s) => s,
                            t => return Err(CompilerError::parser(line, format!("Expected identifier, got {:?}", t))),
                        };
                        self.expect(Token::Eq)?;
                        let expr = self.parse_expr()?;
                        self.expect(Token::Semicolon)?;
                        Stmt {
                            kind: StmtKind::Const(name, expr),
                            line: self.peek_line(),
                        }
                    } else if let Token::Identifier(_) = self.peek() {
                        let name = match self.advance() {
                            Token::Identifier(s) => s,
                            _ => unreachable!(),
                        };
                        self.expect(Token::Eq)?;
                        let expr = self.parse_expr()?;
                        self.expect(Token::Semicolon)?;
                        Stmt {
                            kind: StmtKind::Assign(name, expr),
                            line: self.peek_line(),
                        }
                    } else {
                        return Err(CompilerError::parser(line, format!("Unexpected token in for init: {:?}", self.peek())));
                    };
                    Some(Box::new(init_stmt))
                };

                let cond = if *self.peek() == Token::Semicolon {
                    self.advance();
                    None
                } else {
                    let expr = self.parse_expr()?;
                    self.expect(Token::Semicolon)?;
                    Some(expr)
                };

                let incr = if *self.peek() == Token::RParen {
                    None
                } else {
                    if let Token::Identifier(_) = self.peek() {
                        let name = match self.advance() {
                            Token::Identifier(s) => s,
                            _ => unreachable!(),
                        };
                        self.expect(Token::Eq)?;
                        let expr = self.parse_expr()?;
                        Some(Box::new(Stmt {
                            kind: StmtKind::Assign(name, expr),
                            line: self.peek_line(),
                        }))
                    } else {
                        let expr = self.parse_expr()?;
                        Some(Box::new(Stmt {
                            kind: StmtKind::Expr(expr),
                            line: self.peek_line(),
                        }))
                    }
                };

                self.expect(Token::RParen)?;
                let body = Box::new(self.parse_statement()?);

                StmtKind::For(init, cond, incr, body)
            }
            Token::LBrace => {
                self.advance();
                let mut stmts = Vec::new();
                while *self.peek() != Token::RBrace {
                    stmts.push(self.parse_statement()?);
                }
                self.expect(Token::RBrace)?;
                StmtKind::Block(stmts)
            }
            Token::Return => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(Token::Semicolon)?;
                StmtKind::Return(expr)
            }
            Token::Break => {
                self.advance();
                self.expect(Token::Semicolon)?;
                StmtKind::Break
            }
            Token::Continue => {
                self.advance();
                self.expect(Token::Semicolon)?;
                StmtKind::Continue
            }
            Token::Identifier(_) => {
                let name = match self.advance() {
                    Token::Identifier(s) => s,
                    _ => unreachable!(),
                };
                if *self.peek() == Token::Eq {
                    self.advance();
                    let expr = self.parse_expr()?;
                    self.expect(Token::Semicolon)?;
                    StmtKind::Assign(name, expr)
                } else {
                    self.pos -= 1;
                    let expr = self.parse_expr()?;
                    self.expect(Token::Semicolon)?;
                    StmtKind::Expr(expr)
                }
            }
            _ => {
                let expr = self.parse_expr()?;
                self.expect(Token::Semicolon)?;
                StmtKind::Expr(expr)
            }
        };
        Ok(Stmt { kind, line })
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr> {
        let mut left = self.parse_and()?;
        loop {
            if *self.peek() == Token::OrOr {
                self.advance();
                let right = self.parse_and()?;
                left = Expr::Logical(Box::new(left), LogicalOp::Or, Box::new(right));
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr> {
        let mut left = self.parse_equality()?;
        loop {
            if *self.peek() == Token::AndAnd {
                self.advance();
                let right = self.parse_equality()?;
                left = Expr::Logical(Box::new(left), LogicalOp::And, Box::new(right));
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr> {
        let mut left = self.parse_comparison()?;
        loop {
            let op = match self.peek() {
                Token::EqEq => BinOp::Eq,
                Token::BangEq => BinOp::Ne,
                _ => break,
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr> {
        let mut left = self.parse_additive()?;
        loop {
            let op = match self.peek() {
                Token::Lt => BinOp::Lt,
                Token::Gt => BinOp::Gt,
                Token::LtEq => BinOp::Le,
                Token::GtEq => BinOp::Ge,
                _ => break,
            };
            self.advance();
            let right = self.parse_additive()?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expr> {
        let mut left = self.parse_multiplicative()?;
        loop {
            let op = match self.peek() {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr> {
        let mut left = self.parse_unary()?;
        loop {
            let op = match self.peek() {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                Token::Percent => BinOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr> {
        match self.peek() {
            Token::Minus => {
                self.advance();
                Ok(Expr::Unary(UnaryOp::Neg, Box::new(self.parse_unary()?)))
            }
            Token::Bang => {
                self.advance();
                Ok(Expr::Unary(UnaryOp::Not, Box::new(self.parse_unary()?)))
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Expr> {
        let line = self.peek_line();
        match self.peek().clone() {
            Token::Number(n) => {
                self.advance();
                Ok(Expr::Number(n))
            }
            Token::Identifier(name) => {
                self.advance();
                if *self.peek() == Token::LParen {
                    self.advance();
                    let mut args = Vec::new();
                    if *self.peek() != Token::RParen {
                        loop {
                            args.push(self.parse_expr()?);
                            if *self.peek() == Token::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(Token::RParen)?;
                    Ok(Expr::Call(name, args))
                } else {
                    Ok(Expr::Identifier(name))
                }
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            t => Err(CompilerError::parser(line, format!("Unexpected token in expression: {:?}", t))),
        }
    }
}
