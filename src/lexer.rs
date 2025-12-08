#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i32),
    Identifier(String),

    // Keywords
    Let,
    Const,
    Function,
    If,
    Else,
    While,
    Return,

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

    fn skip_block_comment(&mut self) {
        self.advance(); // consume '/'
        self.advance(); // consume '*'
        loop {
            if self.peek() == '\0' {
                panic!("Unterminated block comment");
            }
            if self.peek() == '*' && self.input.get(self.pos + 1) == Some(&'/') {
                self.advance(); // consume '*'
                self.advance(); // consume '/'
                break;
            }
            self.advance();
        }
    }

    fn read_number(&mut self) -> i32 {
        let mut n = 0i32;
        while self.peek().is_ascii_digit() {
            n = n * 10 + (self.advance() as i32 - '0' as i32);
        }
        n
    }

    fn read_identifier(&mut self) -> String {
        let mut s = String::new();
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            s.push(self.advance());
        }
        s
    }

    pub fn next_token(&mut self) -> (Token, usize) {
        self.skip_whitespace();

        // Single-line comment
        if self.peek() == '/' && self.input.get(self.pos + 1) == Some(&'/') {
            self.skip_line_comment();
            return self.next_token();
        }

        // Multi-line comment
        if self.peek() == '/' && self.input.get(self.pos + 1) == Some(&'*') {
            self.skip_block_comment();
            return self.next_token();
        }

        let line = self.line;
        let c = self.peek();

        if c == '\0' {
            return (Token::Eof, line);
        }

        if c.is_ascii_digit() {
            return (Token::Number(self.read_number()), line);
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
                "return" => Token::Return,
                _ => Token::Identifier(ident),
            };
            return (tok, line);
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
            _ => panic!("Unexpected character: {}", c),
        };
        (tok, line)
    }

    pub fn tokenize(&mut self) -> Vec<(Token, usize)> {
        let mut tokens = Vec::new();
        loop {
            let (tok, line) = self.next_token();
            let is_eof = tok == Token::Eof;
            tokens.push((tok, line));
            if is_eof {
                break;
            }
        }
        tokens
    }
}
