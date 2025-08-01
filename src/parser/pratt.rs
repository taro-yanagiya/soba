//! Parser implementation using Pratt parsing

use super::precedence::Precedence;
use crate::ast::{BinaryOp, Expr, Program, Statement, UnaryOp};
use crate::error::{ParseError, ParseResult};
use crate::lexer::{Lexer, Token, TokenKind};

/// Soba language parser
pub struct Parser<L: Lexer> {
    lexer: L,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl<L: Lexer> Parser<L> {
    pub fn new(mut lexer: L) -> ParseResult<Self> {
        let current_token = lexer.next_token().map_err(ParseError::from)?;
        let peek_token = lexer.next_token().map_err(ParseError::from)?;

        Ok(Parser {
            lexer,
            current_token,
            peek_token,
        })
    }

    fn next_token(&mut self) -> ParseResult<()> {
        self.current_token = self.peek_token.take();
        self.peek_token = self.lexer.next_token().map_err(ParseError::from)?;
        Ok(())
    }

    /// Parse a single expression (test-only method)
    /// This method is only available in test builds and is used for testing
    /// individual expression parsing without requiring a full program structure.
    #[cfg(test)]
    pub fn parse_expression(&mut self) -> ParseResult<Expr> {
        self.parse_expression_with_precedence(Precedence::Lowest)
    }

    pub fn parse_program(&mut self) -> ParseResult<Program> {
        let mut statements = Vec::new();

        while self.current_token.is_some() {
            let expr = self.parse_expression_with_precedence(Precedence::Lowest)?;
            let span = expr.span();
            let stmt = Statement::ExprStatement { expr, span };
            statements.push(stmt);

            // Check if there's a semicolon
            if matches!(
                self.peek_token.as_ref().map(|t| &t.kind),
                Some(TokenKind::Semicolon)
            ) {
                self.next_token()?; // move to semicolon
                self.next_token()?; // consume semicolon and move to next token
            } else {
                // No semicolon - this should be the last statement
                break;
            }
        }

        Ok(Program::new(statements))
    }

    fn parse_expression_with_precedence(&mut self, precedence: Precedence) -> ParseResult<Expr> {
        let mut left = self.parse_prefix()?;

        while let Some(ref peek) = self.peek_token {
            let peek_precedence = Precedence::from_token(&peek.kind);
            if precedence >= peek_precedence {
                break;
            }

            self.next_token()?;
            left = self.parse_infix(left)?;
        }

        Ok(left)
    }

    fn parse_prefix(&mut self) -> ParseResult<Expr> {
        match &self.current_token {
            Some(token) => match &token.kind {
                TokenKind::Int(value) => Ok(Expr::Int {
                    value: *value,
                    span: token.span,
                }),
                TokenKind::Float(value) => Ok(Expr::Float {
                    value: *value,
                    span: token.span,
                }),
                TokenKind::True => Ok(Expr::Bool {
                    value: true,
                    span: token.span,
                }),
                TokenKind::False => Ok(Expr::Bool {
                    value: false,
                    span: token.span,
                }),
                TokenKind::LeftParen => self.parse_grouped_expression(),
                TokenKind::Plus | TokenKind::Minus | TokenKind::Bang => {
                    self.parse_unary_expression()
                }
                _ => Err(ParseError::UnexpectedToken(token.to_string())),
            },
            None => Err(ParseError::UnexpectedEof),
        }
    }

    fn parse_infix(&mut self, left: Expr) -> ParseResult<Expr> {
        match &self.current_token {
            Some(token) => {
                let op = match token.kind {
                    TokenKind::Plus => BinaryOp::Plus,
                    TokenKind::Minus => BinaryOp::Minus,
                    TokenKind::Asterisk => BinaryOp::Multiply,
                    TokenKind::Slash => BinaryOp::Divide,
                    TokenKind::AndAnd => BinaryOp::LogicalAnd,
                    TokenKind::OrOr => BinaryOp::LogicalOr,
                    TokenKind::Equal => BinaryOp::Equal,
                    TokenKind::NotEqual => BinaryOp::NotEqual,
                    TokenKind::Less => BinaryOp::Less,
                    TokenKind::Greater => BinaryOp::Greater,
                    TokenKind::LessEqual => BinaryOp::LessEqual,
                    TokenKind::GreaterEqual => BinaryOp::GreaterEqual,
                    _ => return Err(ParseError::UnexpectedToken(token.to_string())),
                };

                let _op_span = token.span;
                let precedence = Precedence::from_token(&token.kind);

                self.next_token()?;
                let right = self.parse_expression_with_precedence(precedence)?;

                let span = left.span().merge(right.span());

                Ok(Expr::InfixExpr {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                    span,
                })
            }
            None => Err(ParseError::UnexpectedEof),
        }
    }

    fn parse_grouped_expression(&mut self) -> ParseResult<Expr> {
        let start_span = self.current_token.as_ref().unwrap().span;

        self.next_token()?; // consume '('
        let expr = self.parse_expression_with_precedence(Precedence::Lowest)?;

        if !matches!(
            self.peek_token.as_ref().map(|t| &t.kind),
            Some(TokenKind::RightParen)
        ) {
            return Err(ParseError::MismatchedParentheses);
        }

        self.next_token()?; // move to ')'
        let end_span = self.current_token.as_ref().unwrap().span;
        let span = start_span.merge(end_span);

        Ok(Expr::Grouped {
            inner: Box::new(expr),
            span,
        })
    }

    fn parse_unary_expression(&mut self) -> ParseResult<Expr> {
        let token = self.current_token.as_ref().unwrap();
        let op = match token.kind {
            TokenKind::Plus => UnaryOp::Plus,
            TokenKind::Minus => UnaryOp::Minus,
            TokenKind::Bang => UnaryOp::LogicalNot,
            _ => return Err(ParseError::UnexpectedToken(token.to_string())),
        };

        let op_span = token.span;

        self.next_token()?;
        let operand = self.parse_expression_with_precedence(Precedence::Unary)?;

        let span = op_span.merge(operand.span());

        Ok(Expr::UnaryExpr {
            op,
            operand: Box::new(operand),
            span,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::SobaLexer;

    fn parse_expression_string(input: &str) -> ParseResult<Expr> {
        let lexer = SobaLexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer)?;
        parser.parse_expression()
    }

    fn parse_program_string(input: &str) -> ParseResult<Program> {
        let lexer = SobaLexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer)?;
        parser.parse_program()
    }

    #[test]
    fn test_parse_integer() {
        let expr = parse_expression_string("42").unwrap();
        assert!(matches!(expr, Expr::Int { value: 42, .. }));
    }

    #[test]
    fn test_parse_float() {
        let expr = parse_expression_string("3.14").unwrap();
        assert!(matches!(expr, Expr::Float { value, .. } if (value - 3.14).abs() < 1e-10));
    }

    #[test]
    fn test_parse_addition() {
        let expr = parse_expression_string("1 + 2").unwrap();
        assert!(matches!(
            expr,
            Expr::InfixExpr {
                op: BinaryOp::Plus,
                ..
            }
        ));
    }

    #[test]
    fn test_parse_precedence() {
        let expr = parse_expression_string("1 + 2 * 3").unwrap();
        if let Expr::InfixExpr {
            left, op, right, ..
        } = expr
        {
            assert_eq!(op, BinaryOp::Plus);
            assert!(matches!(left.as_ref(), Expr::Int { value: 1, .. }));
            assert!(matches!(
                right.as_ref(),
                Expr::InfixExpr {
                    op: BinaryOp::Multiply,
                    ..
                }
            ));
        } else {
            panic!("Expected infix expression");
        }
    }

    #[test]
    fn test_parse_grouped() {
        let expr = parse_expression_string("(1 + 2)").unwrap();
        assert!(matches!(expr, Expr::Grouped { .. }));
    }

    #[test]
    fn test_parse_unary() {
        let expr = parse_expression_string("-5").unwrap();
        assert!(matches!(
            expr,
            Expr::UnaryExpr {
                op: UnaryOp::Minus,
                ..
            }
        ));
    }

    #[test]
    fn test_parse_division() {
        let expr = parse_expression_string("8 / 2").unwrap();
        assert!(matches!(
            expr,
            Expr::InfixExpr {
                op: BinaryOp::Divide,
                ..
            }
        ));
    }

    #[test]
    fn test_parse_division_precedence() {
        let expr = parse_expression_string("2 + 8 / 4").unwrap();
        if let Expr::InfixExpr {
            left, op, right, ..
        } = expr
        {
            assert_eq!(op, BinaryOp::Plus);
            assert!(matches!(left.as_ref(), Expr::Int { value: 2, .. }));
            assert!(matches!(
                right.as_ref(),
                Expr::InfixExpr {
                    op: BinaryOp::Divide,
                    ..
                }
            ));
        } else {
            panic!("Expected infix expression");
        }
    }

    #[test]
    fn test_parse_boolean_true() {
        let expr = parse_expression_string("true").unwrap();
        assert!(matches!(expr, Expr::Bool { value: true, .. }));
    }

    #[test]
    fn test_parse_boolean_false() {
        let expr = parse_expression_string("false").unwrap();
        assert!(matches!(expr, Expr::Bool { value: false, .. }));
    }

    #[test]
    fn test_parse_logical_not() {
        let expr = parse_expression_string("!true").unwrap();
        assert!(matches!(
            expr,
            Expr::UnaryExpr {
                op: UnaryOp::LogicalNot,
                ..
            }
        ));
    }

    #[test]
    fn test_parse_logical_and() {
        let expr = parse_expression_string("true && false").unwrap();
        assert!(matches!(
            expr,
            Expr::InfixExpr {
                op: BinaryOp::LogicalAnd,
                ..
            }
        ));
    }

    #[test]
    fn test_parse_logical_or() {
        let expr = parse_expression_string("true || false").unwrap();
        assert!(matches!(
            expr,
            Expr::InfixExpr {
                op: BinaryOp::LogicalOr,
                ..
            }
        ));
    }

    #[test]
    fn test_parse_logical_precedence() {
        // true || false && true should parse as true || (false && true)
        let expr = parse_expression_string("true || false && true").unwrap();
        if let Expr::InfixExpr {
            left, op, right, ..
        } = expr
        {
            assert_eq!(op, BinaryOp::LogicalOr);
            assert!(matches!(left.as_ref(), Expr::Bool { value: true, .. }));
            assert!(matches!(
                right.as_ref(),
                Expr::InfixExpr {
                    op: BinaryOp::LogicalAnd,
                    ..
                }
            ));
        } else {
            panic!("Expected infix expression");
        }
    }

    #[test]
    fn test_parse_comparison_equal() {
        let expr = parse_expression_string("5 == 5").unwrap();
        assert!(matches!(
            expr,
            Expr::InfixExpr {
                op: BinaryOp::Equal,
                ..
            }
        ));
    }

    #[test]
    fn test_parse_comparison_not_equal() {
        let expr = parse_expression_string("5 != 3").unwrap();
        assert!(matches!(
            expr,
            Expr::InfixExpr {
                op: BinaryOp::NotEqual,
                ..
            }
        ));
    }

    #[test]
    fn test_parse_comparison_less() {
        let expr = parse_expression_string("3 < 5").unwrap();
        assert!(matches!(
            expr,
            Expr::InfixExpr {
                op: BinaryOp::Less,
                ..
            }
        ));
    }

    #[test]
    fn test_parse_comparison_greater() {
        let expr = parse_expression_string("5 > 3").unwrap();
        assert!(matches!(
            expr,
            Expr::InfixExpr {
                op: BinaryOp::Greater,
                ..
            }
        ));
    }

    #[test]
    fn test_parse_comparison_less_equal() {
        let expr = parse_expression_string("3 <= 5").unwrap();
        assert!(matches!(
            expr,
            Expr::InfixExpr {
                op: BinaryOp::LessEqual,
                ..
            }
        ));
    }

    #[test]
    fn test_parse_comparison_greater_equal() {
        let expr = parse_expression_string("5 >= 3").unwrap();
        assert!(matches!(
            expr,
            Expr::InfixExpr {
                op: BinaryOp::GreaterEqual,
                ..
            }
        ));
    }

    #[test]
    fn test_parse_comparison_precedence() {
        // 1 + 2 < 5 should parse as (1 + 2) < 5
        let expr = parse_expression_string("1 + 2 < 5").unwrap();
        if let Expr::InfixExpr {
            left, op, right, ..
        } = expr
        {
            assert_eq!(op, BinaryOp::Less);
            assert!(matches!(
                left.as_ref(),
                Expr::InfixExpr {
                    op: BinaryOp::Plus,
                    ..
                }
            ));
            assert!(matches!(right.as_ref(), Expr::Int { value: 5, .. }));
        } else {
            panic!("Expected infix expression");
        }
    }

    #[test]
    fn test_parse_comparison_with_logical() {
        // 1 < 2 && 3 > 2 should parse as (1 < 2) && (3 > 2)
        let expr = parse_expression_string("1 < 2 && 3 > 2").unwrap();
        if let Expr::InfixExpr {
            left, op, right, ..
        } = expr
        {
            assert_eq!(op, BinaryOp::LogicalAnd);
            assert!(matches!(
                left.as_ref(),
                Expr::InfixExpr {
                    op: BinaryOp::Less,
                    ..
                }
            ));
            assert!(matches!(
                right.as_ref(),
                Expr::InfixExpr {
                    op: BinaryOp::Greater,
                    ..
                }
            ));
        } else {
            panic!("Expected infix expression");
        }
    }

    #[test]
    fn test_parse_single_statement() {
        let program = parse_program_string("2 + 3;").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Statement::ExprStatement { expr, .. } => {
                assert!(matches!(
                    expr,
                    Expr::InfixExpr {
                        op: BinaryOp::Plus,
                        ..
                    }
                ));
            }
        }
    }

    #[test]
    fn test_parse_multiple_statements() {
        let program = parse_program_string("1 + 2; 3 * 4; 5;").unwrap();
        assert_eq!(program.statements.len(), 3);
        
        // First statement: 1 + 2
        match &program.statements[0] {
            Statement::ExprStatement { expr, .. } => {
                assert!(matches!(
                    expr,
                    Expr::InfixExpr {
                        op: BinaryOp::Plus,
                        ..
                    }
                ));
            }
        }
        
        // Second statement: 3 * 4
        match &program.statements[1] {
            Statement::ExprStatement { expr, .. } => {
                assert!(matches!(
                    expr,
                    Expr::InfixExpr {
                        op: BinaryOp::Multiply,
                        ..
                    }
                ));
            }
        }
        
        // Third statement: 5
        match &program.statements[2] {
            Statement::ExprStatement { expr, .. } => {
                assert!(matches!(expr, Expr::Int { value: 5, .. }));
            }
        }
    }

    #[test]
    fn test_parse_empty_program() {
        let program = parse_program_string("").unwrap();
        assert_eq!(program.statements.len(), 0);
    }

    #[test]
    fn test_parse_statement_without_semicolon_as_last() {
        let program = parse_program_string("2 + 3").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Statement::ExprStatement { expr, .. } => {
                assert!(matches!(
                    expr,
                    Expr::InfixExpr {
                        op: BinaryOp::Plus,
                        ..
                    }
                ));
            }
        }
    }

    #[test]
    fn test_parse_mixed_semicolons() {
        let program = parse_program_string("1 + 2; 3 * 4").unwrap();
        assert_eq!(program.statements.len(), 2);
        
        // First statement: 1 + 2 (with semicolon)
        match &program.statements[0] {
            Statement::ExprStatement { expr, .. } => {
                assert!(matches!(
                    expr,
                    Expr::InfixExpr {
                        op: BinaryOp::Plus,
                        ..
                    }
                ));
            }
        }
        
        // Second statement: 3 * 4 (without semicolon, last statement)
        match &program.statements[1] {
            Statement::ExprStatement { expr, .. } => {
                assert!(matches!(
                    expr,
                    Expr::InfixExpr {
                        op: BinaryOp::Multiply,
                        ..
                    }
                ));
            }
        }
    }
}
