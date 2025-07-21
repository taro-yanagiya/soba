//! Expression evaluation

use crate::ast::{Expr, BinaryOp, UnaryOp};
use crate::error::EvalResult;
use crate::value::Value;

/// Evaluate an expression AST node
pub fn eval_expr(expr: &Expr) -> EvalResult<Value> {
    match expr {
        Expr::Int { value, .. } => Ok(Value::Int(*value)),
        Expr::Float { value, .. } => Ok(Value::Float(*value)),
        Expr::Bool { value, .. } => Ok(Value::Bool(*value)),
        
        Expr::InfixExpr { left, op, right, .. } => {
            match op {
                // Arithmetic operations - evaluate both sides
                BinaryOp::Plus | BinaryOp::Minus | BinaryOp::Multiply | BinaryOp::Divide => {
                    let left_val = eval_expr(left)?;
                    let right_val = eval_expr(right)?;
                    
                    match op {
                        BinaryOp::Plus => left_val.add_value(right_val),
                        BinaryOp::Minus => left_val.subtract_value(right_val),
                        BinaryOp::Multiply => left_val.multiply_value(right_val),
                        BinaryOp::Divide => left_val.divide_value(right_val),
                        _ => unreachable!(),
                    }
                }
                // Logical operations - short-circuit evaluation
                BinaryOp::LogicalAnd => {
                    let left_val = eval_expr(left)?;
                    if !left_val.is_truthy() {
                        Ok(Value::Bool(false))
                    } else {
                        let right_val = eval_expr(right)?;
                        left_val.logical_and(right_val)
                    }
                }
                BinaryOp::LogicalOr => {
                    let left_val = eval_expr(left)?;
                    if left_val.is_truthy() {
                        Ok(Value::Bool(true))
                    } else {
                        let right_val = eval_expr(right)?;
                        left_val.logical_or(right_val)
                    }
                }
            }
        }
        
        Expr::Grouped { inner, .. } => eval_expr(inner),
        
        Expr::UnaryExpr { op, operand, .. } => {
            let val = eval_expr(operand)?;
            match op {
                UnaryOp::Plus => val.positive(),
                UnaryOp::Minus => val.negate(),
                UnaryOp::LogicalNot => val.logical_not(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Expr;

    #[test]
    fn test_eval_integer() {
        let expr = Expr::int(42);
        assert_eq!(eval_expr(&expr).unwrap(), Value::Int(42));
    }

    #[test]
    fn test_eval_float() {
        let expr = Expr::float(3.14);
        assert_eq!(eval_expr(&expr).unwrap(), Value::Float(3.14));
    }

    #[test]
    fn test_eval_addition() {
        use crate::span::{Position, Span};
        
        let expr = Expr::InfixExpr {
            left: Box::new(Expr::int(2)),
            op: BinaryOp::Plus,
            right: Box::new(Expr::int(3)),
            span: Span::single(Position::start()),
        };
        
        assert_eq!(eval_expr(&expr).unwrap(), Value::Float(5.0));
    }

    #[test]
    fn test_eval_unary_minus() {
        use crate::span::{Position, Span};
        
        let expr = Expr::UnaryExpr {
            op: UnaryOp::Minus,
            operand: Box::new(Expr::int(5)),
            span: Span::single(Position::start()),
        };
        
        assert_eq!(eval_expr(&expr).unwrap(), Value::Int(-5));
    }

    #[test]
    fn test_eval_division() {
        use crate::span::{Position, Span};
        
        let expr = Expr::InfixExpr {
            left: Box::new(Expr::int(8)),
            op: BinaryOp::Divide,
            right: Box::new(Expr::int(2)),
            span: Span::single(Position::start()),
        };
        
        assert_eq!(eval_expr(&expr).unwrap(), Value::Float(4.0));
    }

    #[test]
    fn test_eval_division_by_zero() {
        use crate::span::{Position, Span};
        use crate::error::EvalError;
        
        let expr = Expr::InfixExpr {
            left: Box::new(Expr::int(5)),
            op: BinaryOp::Divide,
            right: Box::new(Expr::int(0)),
            span: Span::single(Position::start()),
        };
        
        assert!(matches!(eval_expr(&expr), Err(EvalError::DivisionByZero)));
    }

    #[test]
    fn test_eval_boolean_true() {
        let expr = Expr::bool(true);
        assert_eq!(eval_expr(&expr).unwrap(), Value::Bool(true));
    }

    #[test]
    fn test_eval_boolean_false() {
        let expr = Expr::bool(false);
        assert_eq!(eval_expr(&expr).unwrap(), Value::Bool(false));
    }

    #[test]
    fn test_eval_logical_not() {
        use crate::span::{Position, Span};
        
        let expr = Expr::UnaryExpr {
            op: UnaryOp::LogicalNot,
            operand: Box::new(Expr::bool(true)),
            span: Span::single(Position::start()),
        };
        
        assert_eq!(eval_expr(&expr).unwrap(), Value::Bool(false));
    }

    #[test]
    fn test_eval_logical_and_true() {
        use crate::span::{Position, Span};
        
        let expr = Expr::InfixExpr {
            left: Box::new(Expr::bool(true)),
            op: BinaryOp::LogicalAnd,
            right: Box::new(Expr::bool(true)),
            span: Span::single(Position::start()),
        };
        
        assert_eq!(eval_expr(&expr).unwrap(), Value::Bool(true));
    }

    #[test]
    fn test_eval_logical_and_false() {
        use crate::span::{Position, Span};
        
        let expr = Expr::InfixExpr {
            left: Box::new(Expr::bool(false)),
            op: BinaryOp::LogicalAnd,
            right: Box::new(Expr::bool(true)),
            span: Span::single(Position::start()),
        };
        
        assert_eq!(eval_expr(&expr).unwrap(), Value::Bool(false));
    }

    #[test]
    fn test_eval_logical_or_true() {
        use crate::span::{Position, Span};
        
        let expr = Expr::InfixExpr {
            left: Box::new(Expr::bool(true)),
            op: BinaryOp::LogicalOr,
            right: Box::new(Expr::bool(false)),
            span: Span::single(Position::start()),
        };
        
        assert_eq!(eval_expr(&expr).unwrap(), Value::Bool(true));
    }

    #[test]
    fn test_eval_logical_or_false() {
        use crate::span::{Position, Span};
        
        let expr = Expr::InfixExpr {
            left: Box::new(Expr::bool(false)),
            op: BinaryOp::LogicalOr,
            right: Box::new(Expr::bool(false)),
            span: Span::single(Position::start()),
        };
        
        assert_eq!(eval_expr(&expr).unwrap(), Value::Bool(false));
    }
}