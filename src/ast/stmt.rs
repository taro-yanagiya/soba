//! Statement and Program AST definitions

use crate::ast::Expr;
use crate::span::Span;

/// A statement in the program
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    /// Expression statement (expression followed by semicolon)
    ExprStatement { expr: Expr, span: Span },
}

/// A program is a sequence of statements
#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub span: Span,
}

impl Statement {
    /// Get the span of this statement
    pub fn span(&self) -> Span {
        match self {
            Statement::ExprStatement { span, .. } => *span,
        }
    }

    /// Create a simple expression statement without span
    pub fn expr_statement(expr: Expr) -> Self {
        Statement::ExprStatement {
            span: expr.span(),
            expr,
        }
    }
}

impl Program {
    /// Create a new program with statements
    pub fn new(statements: Vec<Statement>) -> Self {
        let span = if statements.is_empty() {
            Span::single(crate::span::Position::start())
        } else {
            let start = statements.first().unwrap().span().start;
            let end = statements.last().unwrap().span().end;
            Span::new(start, end)
        };

        Program { statements, span }
    }

    /// Create an empty program
    pub fn empty() -> Self {
        Program {
            statements: Vec::new(),
            span: Span::single(crate::span::Position::start()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Expr;

    #[test]
    fn test_statement_creation() {
        let expr = Expr::int(42);
        let stmt = Statement::expr_statement(expr.clone());
        
        match stmt {
            Statement::ExprStatement { expr: e, .. } => {
                assert_eq!(e, expr);
            }
        }
    }

    #[test]
    fn test_program_creation() {
        let expr1 = Expr::int(1);
        let expr2 = Expr::int(2);
        let stmt1 = Statement::expr_statement(expr1);
        let stmt2 = Statement::expr_statement(expr2);
        
        let program = Program::new(vec![stmt1, stmt2]);
        assert_eq!(program.statements.len(), 2);
    }

    #[test]
    fn test_empty_program() {
        let program = Program::empty();
        assert_eq!(program.statements.len(), 0);
    }
}