use std::fmt;

#[derive(Debug, Clone)]
pub struct CompilerError {
    pub line: usize,
    pub message: String,
    pub error_type: ErrorType,
}

#[derive(Debug, Clone)]
pub enum ErrorType {
    LexerError,
    ParserError,
    CodegenError,
    SemanticError,
}

impl CompilerError {
    pub fn new(line: usize, message: String, error_type: ErrorType) -> Self {
        CompilerError {
            line,
            message,
            error_type,
        }
    }

    pub fn lexer(line: usize, message: String) -> Self {
        Self::new(line, message, ErrorType::LexerError)
    }

    pub fn parser(line: usize, message: String) -> Self {
        Self::new(line, message, ErrorType::ParserError)
    }

    pub fn codegen(line: usize, message: String) -> Self {
        Self::new(line, message, ErrorType::CodegenError)
    }

    pub fn semantic(line: usize, message: String) -> Self {
        Self::new(line, message, ErrorType::SemanticError)
    }
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_kind = match self.error_type {
            ErrorType::LexerError => "Lexer Error",
            ErrorType::ParserError => "Parser Error",
            ErrorType::CodegenError => "Codegen Error",
            ErrorType::SemanticError => "Semantic Error",
        };
        write!(f, "{} at line {}: {}", error_kind, self.line, self.message)
    }
}

impl std::error::Error for CompilerError {}

pub type Result<T> = std::result::Result<T, CompilerError>;
