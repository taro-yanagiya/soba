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

    fn read_identifier(&mut self) -> LexResult<Token> {
        let start_pos = self.position;
        let mut identifier_chars = Vec::new();

        // Read letters, digits, and underscores
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                identifier_chars.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        let end_pos = self.position;
        let span = Span::new(start_pos, end_pos);
        let identifier: String = identifier_chars.iter().collect();

        // Check for keywords
        let kind = match identifier.as_str() {
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            _ => return Err(LexError::UnexpectedCharacter(identifier_chars[0])), // For now, only support keywords
        };

        Ok(Token::new(kind, span))
    }

    fn read_two_char_token(&mut self, first_char: char, second_char: char, kind: TokenKind) -> LexResult<Token> {
        let start_pos = self.position;
        
        // Consume first character
        self.advance();
        
        // Check if second character matches
        if self.current_char() == Some(second_char) {
            self.advance(); // consume second character
            let end_pos = self.position;
            Ok(Token::new(kind, Span::new(start_pos, end_pos)))
        } else {
            // If second character doesn't match, it's an unexpected character
            Err(LexError::UnexpectedCharacter(first_char))
        }
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
                } else if ch.is_ascii_alphabetic() || ch == '_' {
                    self.read_identifier().map(Some)
                } else {
                    let token = match ch {
                        '+' => self.read_single_char_token(TokenKind::Plus),
                        '-' => self.read_single_char_token(TokenKind::Minus),
                        '*' => self.read_single_char_token(TokenKind::Asterisk),
                        '/' => self.read_single_char_token(TokenKind::Slash),
                        '!' => self.read_single_char_token(TokenKind::Bang),
                        '&' => return self.read_two_char_token('&', '&', TokenKind::AndAnd).map(Some),
                        '|' => return self.read_two_char_token('|', '|', TokenKind::OrOr).map(Some),
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
        let tokens = tokenize("+ - * /").unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].kind, TokenKind::Plus);
        assert_eq!(tokens[1].kind, TokenKind::Minus);
        assert_eq!(tokens[2].kind, TokenKind::Asterisk);
        assert_eq!(tokens[3].kind, TokenKind::Slash);
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
        let tokens = tokenize("3.14 + 2 * (5 - 1) / 2").unwrap();
        assert_eq!(tokens.len(), 11);
    }

    #[test]
    fn test_division_expression() {
        let tokens = tokenize("8 / 2").unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind, TokenKind::Int(8));
        assert_eq!(tokens[1].kind, TokenKind::Slash);
        assert_eq!(tokens[2].kind, TokenKind::Int(2));
    }

    #[test]
    fn test_boolean_literals() {
        let tokens = tokenize("true false").unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::True);
        assert_eq!(tokens[1].kind, TokenKind::False);
    }

    #[test]
    fn test_boolean_expression() {
        let tokens = tokenize("true").unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::True);

        let tokens = tokenize("false").unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::False);
    }

    #[test]
    fn test_logical_operators() {
        let tokens = tokenize("!").unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Bang);

        let tokens = tokenize("&&").unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::AndAnd);

        let tokens = tokenize("||").unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::OrOr);
    }

    #[test]
    fn test_logical_expression() {
        let tokens = tokenize("!true && false || true").unwrap();
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].kind, TokenKind::Bang);
        assert_eq!(tokens[1].kind, TokenKind::True);
        assert_eq!(tokens[2].kind, TokenKind::AndAnd);
        assert_eq!(tokens[3].kind, TokenKind::False);
        assert_eq!(tokens[4].kind, TokenKind::OrOr);
        assert_eq!(tokens[5].kind, TokenKind::True);
    }

    #[test]
    fn test_invalid_single_ampersand() {
        let result = tokenize("&");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_single_pipe() {
        let result = tokenize("|");
        assert!(result.is_err());
    }
}