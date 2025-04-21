use std::fmt;

use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("Syntax error at {location}: {message}")]
    SyntaxError {
        message: String,
        location: SourceLocation,
    },

    #[error("Semantic error at {location}: {message} (Symbol: {symbol})")]
    SemanticError {
        message: String,
        symbol: String,
        location: SourceLocation,
    },

    #[error("Type error at {location}: {message}")]
    TypeError {
        message: String,
        location: SourceLocation,
    },

    #[error("Code generation error: {message}")]
    CodeGenerationError { message: String },

    #[error("Internal error: {message}")]
    InternalError { message: String },

    #[error("Unknown compiler error")]
    UnknownError,
}

#[derive(Debug)]
pub struct SourceLocation {
    file: String,
    line: usize,
    col: usize,
}

// Implement Display for SourceLocation so it's formatted correctly in error messages
impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.col)
    }
}
