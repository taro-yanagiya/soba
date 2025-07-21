//! Operator precedence definitions

use crate::lexer::TokenKind;

/// Operator precedence levels
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Precedence {
    Lowest,
    Sum,        // + -
    Product,    // * /
    Unary,      // -x +x
    Group,      // ()
}

impl Precedence {
    /// Get precedence for a token
    pub fn from_token(token: &TokenKind) -> Precedence {
        match token {
            TokenKind::Plus | TokenKind::Minus => Precedence::Sum,
            TokenKind::Asterisk => Precedence::Product,
            TokenKind::LeftParen => Precedence::Group,
            _ => Precedence::Lowest,
        }
    }

    /// Get the precedence level as a number for comparison
    pub fn level(&self) -> u8 {
        match self {
            Precedence::Lowest => 0,
            Precedence::Sum => 1,
            Precedence::Product => 2,
            Precedence::Unary => 3,
            Precedence::Group => 4,
        }
    }
}

impl PartialOrd for Precedence {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.level().cmp(&other.level()))
    }
}

impl Ord for Precedence {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.level().cmp(&other.level())
    }
}

impl Eq for Precedence {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precedence_ordering() {
        assert!(Precedence::Lowest < Precedence::Sum);
        assert!(Precedence::Sum < Precedence::Product);
        assert!(Precedence::Product < Precedence::Unary);
        assert!(Precedence::Unary < Precedence::Group);
    }

    #[test]
    fn test_token_precedence() {
        assert_eq!(Precedence::from_token(&TokenKind::Plus), Precedence::Sum);
        assert_eq!(Precedence::from_token(&TokenKind::Asterisk), Precedence::Product);
        assert_eq!(Precedence::from_token(&TokenKind::LeftParen), Precedence::Group);
        assert_eq!(Precedence::from_token(&TokenKind::Int(1)), Precedence::Lowest);
    }
}