//! Abstract Syntax Tree expression definitions

use crate::span::Span;

/// AST node for expressions
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    /// Integer literal
    Int {
        value: i32,
        span: Span,
    },
    /// Floating-point literal  
    Float {
        value: f64,
        span: Span,
    },
    /// Boolean literal
    Bool {
        value: bool,
        span: Span,
    },
    /// Binary infix expression (e.g., 1 + 2)
    InfixExpr {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
        span: Span,
    },
    /// Grouped expression (e.g., (1 + 2))
    Grouped {
        inner: Box<Expr>,
        span: Span,
    },
    /// Unary expression (e.g., -1, +5)
    UnaryExpr {
        op: UnaryOp,
        operand: Box<Expr>,
        span: Span,
    },
}

/// Binary operators
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinaryOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    LogicalAnd,
    LogicalOr,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    // Future: Modulo, etc.
}

/// Unary operators
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnaryOp {
    Plus,
    Minus,
    LogicalNot,
    // Future: other unary operators
}

impl Expr {
    /// Get the span of this expression
    pub fn span(&self) -> Span {
        match self {
            Expr::Int { span, .. }
            | Expr::Float { span, .. }
            | Expr::Bool { span, .. }
            | Expr::InfixExpr { span, .. }
            | Expr::Grouped { span, .. }
            | Expr::UnaryExpr { span, .. } => *span,
        }
    }

    /// Create a simple integer expression without span
    pub fn int(value: i32) -> Self {
        Expr::Int {
            value,
            span: Span::single(crate::span::Position::start()),
        }
    }

    /// Create a simple float expression without span
    pub fn float(value: f64) -> Self {
        Expr::Float {
            value,
            span: Span::single(crate::span::Position::start()),
        }
    }

    /// Create a simple boolean expression without span
    pub fn bool(value: bool) -> Self {
        Expr::Bool {
            value,
            span: Span::single(crate::span::Position::start()),
        }
    }
}

impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Plus => write!(f, "+"),
            BinaryOp::Minus => write!(f, "-"),
            BinaryOp::Multiply => write!(f, "*"),
            BinaryOp::Divide => write!(f, "/"),
            BinaryOp::LogicalAnd => write!(f, "&&"),
            BinaryOp::LogicalOr => write!(f, "||"),
            BinaryOp::Equal => write!(f, "=="),
            BinaryOp::NotEqual => write!(f, "!="),
            BinaryOp::Less => write!(f, "<"),
            BinaryOp::Greater => write!(f, ">"),
            BinaryOp::LessEqual => write!(f, "<="),
            BinaryOp::GreaterEqual => write!(f, ">="),
        }
    }
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Plus => write!(f, "+"),
            UnaryOp::Minus => write!(f, "-"),
            UnaryOp::LogicalNot => write!(f, "!"),
        }
    }
}