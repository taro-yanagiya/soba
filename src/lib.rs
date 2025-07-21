//! Soba Programming Language

pub mod error;
pub mod span;
pub mod value;
pub mod ast;
pub mod lexer;
pub mod parser;
pub mod evaluator;

// Re-export commonly used types
pub use error::{SobaError, SobaResult, LexError, ParseError, EvalError};
pub use span::{Position, Span};
pub use value::Value;
pub use ast::{Expr, BinaryOp, UnaryOp};
pub use lexer::{Token, TokenKind, SobaLexer, Lexer};
pub use parser::{Parser, Precedence};
pub use evaluator::eval_expr;

/// Evaluate a string expression and return the result
pub fn eval_string(input: &str) -> SobaResult<Value> {
    let lexer = SobaLexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer).map_err(SobaError::ParseError)?;
    
    let expr = parser.parse().map_err(SobaError::ParseError)?;
    eval_expr(&expr).map_err(SobaError::EvalError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_string() {
        assert_eq!(eval_string("2 + 3").unwrap(), Value::Float(5.0));
        assert_eq!(eval_string("10 - 4").unwrap(), Value::Float(6.0));
        assert_eq!(eval_string("3 * 4").unwrap(), Value::Float(12.0));
        assert_eq!(eval_string("(1 + 2) * 3").unwrap(), Value::Float(9.0));
    }

    #[test]
    fn test_eval_float() {
        assert_eq!(eval_string("3.14").unwrap(), Value::Float(3.14));
        assert_eq!(eval_string("1.5 + 2.5").unwrap(), Value::Float(4.0));
        assert_eq!(eval_string("3 + 2.5").unwrap(), Value::Float(5.5));
    }
}