//! Token definitions for the lexer

use crate::span::Span;

/// A token with position information
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

/// Token types
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Literals
    Int(i32),
    Float(f64),
    
    // Operators
    Plus,
    Minus,
    Asterisk,
    
    // Delimiters
    LeftParen,
    RightParen,
    
    // Special
    Eof,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Create a token without position information (for testing)
    pub fn simple(kind: TokenKind) -> Self {
        Self {
            kind,
            span: Span::single(crate::span::Position::start()),
        }
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Int(i) => write!(f, "{i}"),
            TokenKind::Float(fl) => write!(f, "{fl}"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Asterisk => write!(f, "*"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::Eof => write!(f, "EOF"),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}