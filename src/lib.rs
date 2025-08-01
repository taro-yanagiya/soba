//! Soba Programming Language

pub mod ast;
pub mod error;
pub mod evaluator;
pub mod lexer;
pub mod parser;
pub mod span;
pub mod value;

// Re-export commonly used types
pub use ast::{BinaryOp, Expr, Program, Statement, UnaryOp};
pub use error::{EvalError, LexError, ParseError, SobaError, SobaResult};
pub use evaluator::{eval_expr, eval_program, eval_statement};
pub use lexer::{Lexer, SobaLexer, Token, TokenKind};
pub use parser::{Parser, Precedence};
pub use span::{Position, Span};
pub use value::Value;


/// Evaluate a string containing a program (multiple statements) and return the result
pub fn eval_program_string(input: &str) -> SobaResult<Value> {
    let lexer = SobaLexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer).map_err(SobaError::ParseError)?;

    let program = parser.parse_program().map_err(SobaError::ParseError)?;
    eval_program(&program).map_err(SobaError::EvalError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_program_string_single_with_semicolon() {
        assert_eq!(eval_program_string("2 + 3;").unwrap(), Value::Float(5.0));
    }

    #[test]
    fn test_eval_program_string_single_without_semicolon() {
        assert_eq!(eval_program_string("2 + 3").unwrap(), Value::Float(5.0));
    }

    #[test]
    fn test_eval_program_string_multiple_with_semicolons() {
        assert_eq!(eval_program_string("1 + 2; 3 * 4; 10;").unwrap(), Value::Int(10));
    }

    #[test]
    fn test_eval_program_string_multiple_last_without_semicolon() {
        assert_eq!(eval_program_string("1 + 2; 3 * 4; 10").unwrap(), Value::Int(10));
    }

    #[test]
    fn test_eval_program_string_empty() {
        assert_eq!(eval_program_string("").unwrap(), Value::Int(0));
    }

    #[test]
    fn test_eval_program_string_complex() {
        assert_eq!(
            eval_program_string("2 + 3; 4 * 5; (10 - 2) / 2").unwrap(),
            Value::Float(4.0)
        );
    }

    #[test]
    fn test_eval_program_string_mixed_semicolons() {
        assert_eq!(
            eval_program_string("5 + 5; 2 * 3").unwrap(),
            Value::Float(6.0)
        );
    }
}
