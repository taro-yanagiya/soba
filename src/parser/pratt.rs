//! Parser implementation using Pratt parsing

use crate::ast::{Expr, BinaryOp, UnaryOp};
use crate::error::{ParseError, ParseResult};
use crate::lexer::{Lexer, Token, TokenKind};
use super::precedence::Precedence;

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

    pub fn parse(&mut self) -> ParseResult<Expr> {
        self.parse_expression(Precedence::Lowest)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> ParseResult<Expr> {
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
                TokenKind::Int(value) => {
                    Ok(Expr::Int {
                        value: *value,
                        span: token.span,
                    })
                }
                TokenKind::Float(value) => {
                    Ok(Expr::Float {
                        value: *value,
                        span: token.span,
                    })
                }
                TokenKind::LeftParen => self.parse_grouped_expression(),
                TokenKind::Plus | TokenKind::Minus => self.parse_unary_expression(),
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
                    _ => return Err(ParseError::UnexpectedToken(token.to_string())),
                };

                let _op_span = token.span;
                let precedence = Precedence::from_token(&token.kind);
                
                self.next_token()?;
                let right = self.parse_expression(precedence)?;
                
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
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        if !matches!(self.peek_token.as_ref().map(|t| &t.kind), Some(TokenKind::RightParen)) {
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
            _ => return Err(ParseError::UnexpectedToken(token.to_string())),
        };

        let op_span = token.span;
        
        self.next_token()?;
        let operand = self.parse_expression(Precedence::Unary)?;
        
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

    fn parse_string(input: &str) -> ParseResult<Expr> {
        let lexer = SobaLexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer)?;
        parser.parse()
    }

    #[test]
    fn test_parse_integer() {
        let expr = parse_string("42").unwrap();
        assert!(matches!(expr, Expr::Int { value: 42, .. }));
    }

    #[test]
    fn test_parse_float() {
        let expr = parse_string("3.14").unwrap();
        assert!(matches!(expr, Expr::Float { value, .. } if (value - 3.14).abs() < 1e-10));
    }

    #[test]
    fn test_parse_addition() {
        let expr = parse_string("1 + 2").unwrap();
        assert!(matches!(expr, Expr::InfixExpr { op: BinaryOp::Plus, .. }));
    }

    #[test]
    fn test_parse_precedence() {
        let expr = parse_string("1 + 2 * 3").unwrap();
        if let Expr::InfixExpr { left, op, right, .. } = expr {
            assert_eq!(op, BinaryOp::Plus);
            assert!(matches!(left.as_ref(), Expr::Int { value: 1, .. }));
            assert!(matches!(right.as_ref(), Expr::InfixExpr { op: BinaryOp::Multiply, .. }));
        } else {
            panic!("Expected infix expression");
        }
    }

    #[test]
    fn test_parse_grouped() {
        let expr = parse_string("(1 + 2)").unwrap();
        assert!(matches!(expr, Expr::Grouped { .. }));
    }

    #[test]
    fn test_parse_unary() {
        let expr = parse_string("-5").unwrap();
        assert!(matches!(expr, Expr::UnaryExpr { op: UnaryOp::Minus, .. }));
    }

    #[test]
    fn test_parse_division() {
        let expr = parse_string("8 / 2").unwrap();
        assert!(matches!(expr, Expr::InfixExpr { op: BinaryOp::Divide, .. }));
    }

    #[test]
    fn test_parse_division_precedence() {
        let expr = parse_string("2 + 8 / 4").unwrap();
        if let Expr::InfixExpr { left, op, right, .. } = expr {
            assert_eq!(op, BinaryOp::Plus);
            assert!(matches!(left.as_ref(), Expr::Int { value: 2, .. }));
            assert!(matches!(right.as_ref(), Expr::InfixExpr { op: BinaryOp::Divide, .. }));
        } else {
            panic!("Expected infix expression");
        }
    }
}