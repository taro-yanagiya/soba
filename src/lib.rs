//! Soba Programming Language
//! 
//! A simple arithmetic expression language with support for:
//! - Integer and floating-point numbers
//! - Basic arithmetic operations (+, -, *, /)
//! - Unary operators (+, -)
//! - Parentheses for grouping
//! - Interactive REPL

pub mod error;
pub mod span;
pub mod value;

// Include the existing modules for now
mod lexer;
mod parser;

// Re-export commonly used types
pub use error::{SobaError, SobaResult, LexError, ParseError, EvalError};
pub use span::{Position, Span};
pub use value::Value;
pub use lexer::{Token, SobaLexer, Lexer};
pub use parser::{Expr, Parser, Op, UnaryOp, Precedence};

/// Evaluate a string expression and return the result
pub fn eval_string(input: &str) -> SobaResult<Value> {
    let mut lexer = SobaLexer::new(input.chars().collect());
    let mut parser = Parser::new(&mut lexer);
    
    match parser.parse() {
        Some(expr) => eval_expr(&expr),
        None => Err(SobaError::ParseError(ParseError::InvalidExpression)),
    }
}

/// Evaluate an expression AST node
pub fn eval_expr(expr: &Expr) -> SobaResult<Value> {
    match expr {
        Expr::Int(n) => Ok(Value::Int(*n)),
        Expr::Float(f) => Ok(Value::Float(*f)),
        Expr::InfixExpr { left, op, right } => {
            let left_val = eval_expr(left)?;
            let right_val = eval_expr(right)?;
            
            match op {
                Op::Plus => left_val.add(right_val).map_err(SobaError::EvalError),
                Op::Minus => left_val.subtract(right_val).map_err(SobaError::EvalError),
                Op::Asterisk => left_val.multiply(right_val).map_err(SobaError::EvalError),
            }
        }
        Expr::Grouped(inner) => eval_expr(inner),
        Expr::UnaryExpr { op, operand } => {
            let val = eval_expr(operand)?;
            match op {
                UnaryOp::Plus => val.positive().map_err(SobaError::EvalError),
                UnaryOp::Minus => val.negate().map_err(SobaError::EvalError),
            }
        }
    }
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

    #[test]
    fn test_eval_unary() {
        assert_eq!(eval_string("-5").unwrap(), Value::Int(-5));
        assert_eq!(eval_string("+3").unwrap(), Value::Int(3));
        assert_eq!(eval_string("-1.5").unwrap(), Value::Float(-1.5));
    }
}