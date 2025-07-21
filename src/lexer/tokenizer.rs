//! Tokenizer implementation

use crate::error::{LexError, LexResult};
use crate::span::{Position, Span};
use super::token::{Token, TokenKind};

/// Trait for lexical analysis
pub trait Lexer {
    fn next_token(&mut self) -> LexResult<Option<Token>>;
}

/// Soba language tokenizer
pub struct SobaLexer {
    input: Vec<char>,
    position: Position,
    current_index: usize,
}

impl SobaLexer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input,
            position: Position::start(),
            current_index: 0,
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.current_index).copied()
    }

    fn peek_char(&self) -> Option<char> {
        self.input.get(self.current_index + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.current_char() {
            self.position.advance(ch);
            self.current_index += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> LexResult<Token> {
        let start_pos = self.position;
        let mut number_chars = Vec::new();
        let mut has_dot = false;

        // Handle leading dot (.5)
        if self.current_char() == Some('.') {
            has_dot = true;
            number_chars.push(self.advance().unwrap());
        }

        // Read digits
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() {
                number_chars.push(self.advance().unwrap());
            } else if ch == '.' && !has_dot {
                has_dot = true;
                number_chars.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        let end_pos = self.position;
        let span = Span::new(start_pos, end_pos);
        let number_str: String = number_chars.iter().collect();

        if has_dot {
            number_str
                .parse::<f64>()
                .map(|f| Token::new(TokenKind::Float(f), span))
                .map_err(|_| LexError::InvalidNumber(number_str))
        } else {
            number_str
                .parse::<i32>()
                .map(|i| Token::new(TokenKind::Int(i), span))
                .map_err(|_| LexError::InvalidNumber(number_str))
        }
    }

    fn read_single_char_token(&mut self, kind: TokenKind) -> Token {
        let start_pos = self.position;
        self.advance();
        let end_pos = self.position;
        Token::new(kind, Span::new(start_pos, end_pos))
    }
}

impl Lexer for SobaLexer {
    fn next_token(&mut self) -> LexResult<Option<Token>> {
        self.skip_whitespace();

        match self.current_char() {
            None => Ok(None), // EOF
            Some(ch) => {
                if ch.is_ascii_digit() || ch == '.' {
                    self.read_number().map(Some)
                } else {
                    let token = match ch {
                        '+' => self.read_single_char_token(TokenKind::Plus),
                        '-' => self.read_single_char_token(TokenKind::Minus),
                        '*' => self.read_single_char_token(TokenKind::Asterisk),
                        '(' => self.read_single_char_token(TokenKind::LeftParen),
                        ')' => self.read_single_char_token(TokenKind::RightParen),
                        _ => return Err(LexError::UnexpectedCharacter(ch)),
                    };
                    Ok(Some(token))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(input: &str) -> LexResult<Vec<Token>> {
        let mut lexer = SobaLexer::new(input.chars().collect());
        let mut tokens = Vec::new();
        
        while let Some(token) = lexer.next_token()? {
            tokens.push(token);
        }
        
        Ok(tokens)
    }

    #[test]
    fn test_integers() {
        let tokens = tokenize("123").unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Int(123));
    }

    #[test]
    fn test_floats() {
        let tokens = tokenize("3.14").unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Float(3.14));

        let tokens = tokenize(".5").unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Float(0.5));
    }

    #[test]
    fn test_operators() {
        let tokens = tokenize("+ - *").unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind, TokenKind::Plus);
        assert_eq!(tokens[1].kind, TokenKind::Minus);
        assert_eq!(tokens[2].kind, TokenKind::Asterisk);
    }

    #[test]
    fn test_parentheses() {
        let tokens = tokenize("(1 + 2)").unwrap();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].kind, TokenKind::LeftParen);
        assert_eq!(tokens[1].kind, TokenKind::Int(1));
        assert_eq!(tokens[2].kind, TokenKind::Plus);
        assert_eq!(tokens[3].kind, TokenKind::Int(2));
        assert_eq!(tokens[4].kind, TokenKind::RightParen);
    }

    #[test]
    fn test_expression() {
        let tokens = tokenize("3.14 + 2 * (5 - 1)").unwrap();
        assert_eq!(tokens.len(), 9);
    }
}