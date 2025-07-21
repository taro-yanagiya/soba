//! Error types for the Soba programming language

use std::fmt;

/// Main error type for Soba operations
#[derive(Debug, Clone, PartialEq)]
pub enum SobaError {
    LexError(LexError),
    ParseError(ParseError),
    EvalError(EvalError),
}

/// Lexing errors
#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
    InvalidNumber(String),
    UnexpectedCharacter(char),
    UnterminatedString,
}

/// Parsing errors
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken(String),
    UnexpectedEof,
    MismatchedParentheses,
    InvalidExpression,
}

/// Evaluation errors
#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    DivisionByZero,
    Overflow,
    TypeError(String),
    StackOverflow,
}

impl fmt::Display for SobaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SobaError::LexError(e) => write!(f, "Lexing error: {}", e),
            SobaError::ParseError(e) => write!(f, "Parse error: {}", e),
            SobaError::EvalError(e) => write!(f, "Evaluation error: {}", e),
        }
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            LexError::UnexpectedCharacter(c) => write!(f, "Unexpected character: '{}'", c),
            LexError::UnterminatedString => write!(f, "Unterminated string literal"),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
            ParseError::UnexpectedEof => write!(f, "Unexpected end of input"),
            ParseError::MismatchedParentheses => write!(f, "Mismatched parentheses"),
            ParseError::InvalidExpression => write!(f, "Invalid expression"),
        }
    }
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::DivisionByZero => write!(f, "Division by zero"),
            EvalError::Overflow => write!(f, "Arithmetic overflow"),
            EvalError::TypeError(msg) => write!(f, "Type error: {}", msg),
            EvalError::StackOverflow => write!(f, "Stack overflow"),
        }
    }
}

impl std::error::Error for SobaError {}
impl std::error::Error for LexError {}
impl std::error::Error for ParseError {}
impl std::error::Error for EvalError {}

impl From<LexError> for SobaError {
    fn from(err: LexError) -> Self {
        SobaError::LexError(err)
    }
}

impl From<ParseError> for SobaError {
    fn from(err: ParseError) -> Self {
        SobaError::ParseError(err)
    }
}

impl From<EvalError> for SobaError {
    fn from(err: EvalError) -> Self {
        SobaError::EvalError(err)
    }
}

impl From<LexError> for ParseError {
    fn from(err: LexError) -> Self {
        match err {
            LexError::InvalidNumber(s) => ParseError::UnexpectedToken(format!("invalid number: {}", s)),
            LexError::UnexpectedCharacter(c) => ParseError::UnexpectedToken(format!("unexpected character: '{}'", c)),
            LexError::UnterminatedString => ParseError::UnexpectedToken("unterminated string".to_string()),
        }
    }
}

/// Result type alias for Soba operations
pub type SobaResult<T> = Result<T, SobaError>;
pub type LexResult<T> = Result<T, LexError>;
pub type ParseResult<T> = Result<T, ParseError>;
pub type EvalResult<T> = Result<T, EvalError>;