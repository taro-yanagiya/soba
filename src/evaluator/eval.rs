//! Expression evaluation

use crate::ast::{BinaryOp, Expr, Program, Statement, UnaryOp};
use crate::error::EvalResult;
use crate::value::Value;

/// Evaluate an expression AST node
pub fn eval_expr(expr: &Expr) -> EvalResult<Value> {
    match expr {
        Expr::Int { value, .. } => Ok(Value::Int(*value)),
        Expr::Float { value, .. } => Ok(Value::Float(*value)),
        Expr::Bool { value, .. } => Ok(Value::Bool(*value)),

        Expr::InfixExpr {
            left, op, right, ..
        } => {
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
                // Comparison operations - evaluate both sides
                BinaryOp::Equal
                | BinaryOp::NotEqual
                | BinaryOp::Less
                | BinaryOp::Greater
                | BinaryOp::LessEqual
                | BinaryOp::GreaterEqual => {
                    let left_val = eval_expr(left)?;
                    let right_val = eval_expr(right)?;

                    match op {
                        BinaryOp::Equal => left_val.equal_to(right_val),
                        BinaryOp::NotEqual => left_val.not_equal_to(right_val),
                        BinaryOp::Less => left_val.less_than(right_val),
                        BinaryOp::Greater => left_val.greater_than(right_val),
                        BinaryOp::LessEqual => left_val.less_equal(right_val),
                        BinaryOp::GreaterEqual => left_val.greater_equal(right_val),
                        _ => unreachable!(),
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

/// Evaluate a statement AST node
pub fn eval_statement(stmt: &Statement) -> EvalResult<Value> {
    match stmt {
        Statement::ExprStatement { expr, .. } => eval_expr(expr),
    }
}

/// Evaluate a program AST node
/// Returns the value of the last statement, or a default value for empty programs
pub fn eval_program(program: &Program) -> EvalResult<Value> {
    if program.statements.is_empty() {
        // Return a default value for empty programs
        return Ok(Value::Int(0));
    }

    let mut last_value = Value::Int(0);
    for stmt in &program.statements {
        last_value = eval_statement(stmt)?;
    }

    Ok(last_value)
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
        use crate::error::EvalError;
        use crate::span::{Position, Span};

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

    #[test]
    fn test_eval_comparison_equal() {
        use crate::span::{Position, Span};

        let expr = Expr::InfixExpr {
            left: Box::new(Expr::int(5)),
            op: BinaryOp::Equal,
            right: Box::new(Expr::int(5)),
            span: Span::single(Position::start()),
        };

        assert_eq!(eval_expr(&expr).unwrap(), Value::Bool(true));
    }

    #[test]
    fn test_eval_comparison_not_equal() {
        use crate::span::{Position, Span};

        let expr = Expr::InfixExpr {
            left: Box::new(Expr::int(5)),
            op: BinaryOp::NotEqual,
            right: Box::new(Expr::int(3)),
            span: Span::single(Position::start()),
        };

        assert_eq!(eval_expr(&expr).unwrap(), Value::Bool(true));
    }

    #[test]
    fn test_eval_comparison_less() {
        use crate::span::{Position, Span};

        let expr = Expr::InfixExpr {
            left: Box::new(Expr::int(3)),
            op: BinaryOp::Less,
            right: Box::new(Expr::int(5)),
            span: Span::single(Position::start()),
        };

        assert_eq!(eval_expr(&expr).unwrap(), Value::Bool(true));
    }

    #[test]
    fn test_eval_comparison_greater() {
        use crate::span::{Position, Span};

        let expr = Expr::InfixExpr {
            left: Box::new(Expr::int(5)),
            op: BinaryOp::Greater,
            right: Box::new(Expr::int(3)),
            span: Span::single(Position::start()),
        };

        assert_eq!(eval_expr(&expr).unwrap(), Value::Bool(true));
    }

    #[test]
    fn test_eval_comparison_less_equal() {
        use crate::span::{Position, Span};

        let expr = Expr::InfixExpr {
            left: Box::new(Expr::int(3)),
            op: BinaryOp::LessEqual,
            right: Box::new(Expr::int(5)),
            span: Span::single(Position::start()),
        };

        assert_eq!(eval_expr(&expr).unwrap(), Value::Bool(true));
    }

    #[test]
    fn test_eval_comparison_greater_equal() {
        use crate::span::{Position, Span};

        let expr = Expr::InfixExpr {
            left: Box::new(Expr::int(5)),
            op: BinaryOp::GreaterEqual,
            right: Box::new(Expr::int(5)),
            span: Span::single(Position::start()),
        };

        assert_eq!(eval_expr(&expr).unwrap(), Value::Bool(true));
    }

    #[test]
    fn test_eval_mixed_types_comparison() {
        use crate::span::{Position, Span};

        let expr = Expr::InfixExpr {
            left: Box::new(Expr::int(5)),
            op: BinaryOp::Equal,
            right: Box::new(Expr::float(5.0)),
            span: Span::single(Position::start()),
        };

        assert_eq!(eval_expr(&expr).unwrap(), Value::Bool(true));
    }

    #[test]
    fn test_eval_statement() {
        let expr = Expr::int(42);
        let stmt = Statement::expr_statement(expr);
        assert_eq!(eval_statement(&stmt).unwrap(), Value::Int(42));
    }

    #[test]
    fn test_eval_empty_program() {
        let program = Program::empty();
        assert_eq!(eval_program(&program).unwrap(), Value::Int(0));
    }

    #[test]
    fn test_eval_single_statement_program() {
        let expr = Expr::int(42);
        let stmt = Statement::expr_statement(expr);
        let program = Program::new(vec![stmt]);
        assert_eq!(eval_program(&program).unwrap(), Value::Int(42));
    }

    #[test]
    fn test_eval_multiple_statement_program() {
        use crate::span::{Position, Span};

        // Create statements: 1 + 2; 3 * 4; 10;
        let stmt1 = Statement::expr_statement(Expr::InfixExpr {
            left: Box::new(Expr::int(1)),
            op: BinaryOp::Plus,
            right: Box::new(Expr::int(2)),
            span: Span::single(Position::start()),
        });

        let stmt2 = Statement::expr_statement(Expr::InfixExpr {
            left: Box::new(Expr::int(3)),
            op: BinaryOp::Multiply,
            right: Box::new(Expr::int(4)),
            span: Span::single(Position::start()),
        });

        let stmt3 = Statement::expr_statement(Expr::int(10));

        let program = Program::new(vec![stmt1, stmt2, stmt3]);
        
        // Should return the value of the last statement (10)
        assert_eq!(eval_program(&program).unwrap(), Value::Int(10));
    }
}
