use crate::error::{CompilerError, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i32),
    NumberF32(f32),
    Identifier(String),

    // Keywords
    Let,
    Const,
    Function,
    If,
    Else,
    While,
    For,
    Return,
    Break,
    Continue,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Bang,
    EqEq,
    BangEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Eq,

    // Logical
    AndAnd,
    OrOr,

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,

    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
}
impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
        }
    }

    fn peek(&self) -> char {
        self.input.get(self.pos).copied().unwrap_or('\0')
    }

    fn advance(&mut self) -> char {
        let c = self.peek();
        self.pos += 1;
        if c == '\n' {
            self.line += 1;
        }
        c
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_whitespace() {
            self.advance();
        }
    }

    fn skip_line_comment(&mut self) {
        while self.peek() != '\n' && self.peek() != '\0' {
            self.advance();
        }
    }

    fn skip_block_comment(&mut self) -> Result<()> {
        self.advance(); // consume '/'
        self.advance(); // consume '*'
        loop {
            if self.peek() == '\0' {
                return Err(CompilerError::lexer(
                    self.line,
                    "Unterminated block comment".to_string(),
                ));
            }
            if self.peek() == '*' && self.input.get(self.pos + 1) == Some(&'/') {
                self.advance(); // consume '*'
                self.advance(); // consume '/'
                break;
            }
            self.advance();
        }
        Ok(())
    }

    fn read_number(&mut self) -> Result<(Token, usize)> {
        let start_line = self.line;
        let mut num_str = String::new();
        let mut is_float = false;

        // Read integer part
        while self.peek().is_ascii_digit() {
            num_str.push(self.advance());
        }

        // Check for decimal point
        if self.peek() == '.' {
            // Check what's after the dot to decide if we should consume it
            let next_char = self.input.get(self.pos + 1).copied();

            // Consume the dot if it's part of a float literal:
            // - 3.14 (followed by digit)
            // - 3.e5 (followed by exponent marker)
            // - 3.; or 3.) (trailing dot - followed by non-alphanumeric)
            // Don't consume if followed by alphabetic (except e/E), as that would be like "3.foo"
            let is_float_dot = match next_char {
                Some(ch) if ch.is_ascii_digit() => true,              // 3.14
                Some('e') | Some('E') => true,                        // 3.e5
                Some(ch) if ch == '_' || ch.is_alphabetic() => false, // Don't consume, could be "3.foo" error
                _ => true, // Trailing dot (3.; or 3.) or 3. at EOF)
            };

            if is_float_dot {
                is_float = true;
                num_str.push(self.advance()); // consume '.'

                // Read fractional part (if any)
                while self.peek().is_ascii_digit() {
                    num_str.push(self.advance());
                }
            }
        }

        // Check for exponent
        if self.peek() == 'e' || self.peek() == 'E' {
            is_float = true;
            num_str.push(self.advance()); // consume 'e'/'E'

            // Optional sign
            if self.peek() == '+' || self.peek() == '-' {
                num_str.push(self.advance());
            }

            // Exponent digits
            if !self.peek().is_ascii_digit() {
                return Err(CompilerError::lexer(
                    start_line,
                    "Invalid number: expected digit after exponent".to_string(),
                ));
            }
            while self.peek().is_ascii_digit() {
                num_str.push(self.advance());
            }
        }

        // Parse the string
        if is_float {
            match num_str.parse::<f32>() {
                Ok(f) => Ok((Token::NumberF32(f), start_line)),
                Err(_) => Err(CompilerError::lexer(
                    start_line,
                    format!("Invalid float literal: {}", num_str),
                )),
            }
        } else {
            match num_str.parse::<i32>() {
                Ok(n) => Ok((Token::Number(n), start_line)),
                Err(_) => Err(CompilerError::lexer(
                    start_line,
                    format!("Invalid integer literal: {}", num_str),
                )),
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut s = String::new();
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            s.push(self.advance());
        }
        s
    }

    pub fn next_token(&mut self) -> Result<(Token, usize)> {
        self.skip_whitespace();

        // Single-line comment
        if self.peek() == '/' && self.input.get(self.pos + 1) == Some(&'/') {
            self.skip_line_comment();
            return self.next_token();
        }

        // Multi-line comment
        if self.peek() == '/' && self.input.get(self.pos + 1) == Some(&'*') {
            self.skip_block_comment()?;
            return self.next_token();
        }

        let line = self.line;
        let c = self.peek();

        if c == '\0' {
            return Ok((Token::Eof, line));
        }

        // Handle .5 style floats
        if c == '.'
            && self
                .input
                .get(self.pos + 1)
                .map_or(false, |ch| ch.is_ascii_digit())
        {
            return self.read_number();
        }

        if c.is_ascii_digit() {
            return self.read_number();
        }

        if c.is_alphabetic() || c == '_' {
            let ident = self.read_identifier();
            let tok = match ident.as_str() {
                "let" => Token::Let,
                "const" => Token::Const,
                "function" => Token::Function,
                "if" => Token::If,
                "else" => Token::Else,
                "while" => Token::While,
                "for" => Token::For,
                "return" => Token::Return,
                "break" => Token::Break,
                "continue" => Token::Continue,
                _ => Token::Identifier(ident),
            };
            return Ok((tok, line));
        }

        self.advance();
        let tok = match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '%' => Token::Percent,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    Token::BangEq
                } else {
                    Token::Bang
                }
            }
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    Token::EqEq
                } else {
                    Token::Eq
                }
            }
            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    Token::LtEq
                } else {
                    Token::Lt
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    Token::GtEq
                } else {
                    Token::Gt
                }
            }
            '&' => {
                if self.peek() == '&' {
                    self.advance();
                    Token::AndAnd
                } else {
                    return Err(CompilerError::lexer(
                        line,
                        format!("Unexpected character: {}", c),
                    ));
                }
            }
            '|' => {
                if self.peek() == '|' {
                    self.advance();
                    Token::OrOr
                } else {
                    return Err(CompilerError::lexer(
                        line,
                        format!("Unexpected character: {}", c),
                    ));
                }
            }
            _ => {
                return Err(CompilerError::lexer(
                    line,
                    format!("Unexpected character: {}", c),
                ))
            }
        };
        Ok((tok, line))
    }
    pub fn tokenize(&mut self) -> Result<Vec<(Token, usize)>> {
        let mut tokens = Vec::new();
        loop {
            let (tok, line) = self.next_token()?;
            let is_eof = tok == Token::Eof;
            tokens.push((tok, line));
            if is_eof {
                break;
            }
        }
        Ok(tokens)
    }
}
