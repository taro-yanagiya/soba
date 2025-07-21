#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Int(i32),
    Float(f64),
    Plus,
    Minus,
    Asterisk,
    LeftParen,
    RightParen,
}

pub trait Lexer {
    fn next_token(&mut self) -> Option<Token>;
}

pub struct SobaLexer {
    input: Vec<char>,
    position: usize,
}

impl SobaLexer {
    pub fn new(input: Vec<char>) -> SobaLexer {
        SobaLexer { input, position: 0 }
    }

    fn peek(&self) -> Option<&char> {
        self.input.get(self.position + 1)
    }

    fn forward(&mut self) {
        self.position += 1;
    }

    fn current(&self) -> Option<&char> {
        self.input.get(self.position)
    }
}

impl Lexer for SobaLexer {
    fn next_token(&mut self) -> Option<Token> {
        if self.position >= self.input.len() {
            return None;
        }

        while self.current().is_some() && self.current().unwrap().is_whitespace() {
            self.forward();
        }

        let current = self.current()?;

        let token = if is_number(current) || *current == '.' {
            let mut number = vec![*current];
            let mut has_dot = *current == '.';
            
            while self.peek().is_some() {
                let next_char = self.peek().unwrap();
                if is_number(next_char) {
                    self.forward();
                    number.push(*self.current().unwrap());
                } else if *next_char == '.' && !has_dot {
                    has_dot = true;
                    self.forward();
                    number.push(*self.current().unwrap());
                } else {
                    break;
                }
            }
            
            let number_str = String::from_iter(number);
            if has_dot {
                number_str.parse::<f64>().ok().map(Token::Float)
            } else {
                number_str.parse::<i32>().ok().map(Token::Int)
            }
        } else {
            match current {
                '+' => Some(Token::Plus),
                '-' => Some(Token::Minus),
                '*' => Some(Token::Asterisk),
                '(' => Some(Token::LeftParen),
                ')' => Some(Token::RightParen),
                _ => None,
            }
        };

        self.forward();
        token
    }
}

fn is_number(c: &char) -> bool {
    c.is_digit(10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plus_1() {
        let mut lexer = SobaLexer::new("1 +2".chars().collect());
        assert_eq!(lexer.next_token(), Some(Token::Int(1)));
        assert_eq!(lexer.next_token(), Some(Token::Plus));
        assert_eq!(lexer.next_token(), Some(Token::Int(2)));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn plus_2() {
        let mut lexer = SobaLexer::new("1 + 2 +  3".chars().collect());
        assert_eq!(lexer.next_token(), Some(Token::Int(1)));
        assert_eq!(lexer.next_token(), Some(Token::Plus));
        assert_eq!(lexer.next_token(), Some(Token::Int(2)));
        assert_eq!(lexer.next_token(), Some(Token::Plus));
        assert_eq!(lexer.next_token(), Some(Token::Int(3)));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn minus_1() {
        let mut lexer = SobaLexer::new("1 - 2".chars().collect());
        assert_eq!(lexer.next_token(), Some(Token::Int(1)));
        assert_eq!(lexer.next_token(), Some(Token::Minus));
        assert_eq!(lexer.next_token(), Some(Token::Int(2)));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn multiply_1() {
        let mut lexer = SobaLexer::new("1 * 2".chars().collect());
        assert_eq!(lexer.next_token(), Some(Token::Int(1)));
        assert_eq!(lexer.next_token(), Some(Token::Asterisk));
        assert_eq!(lexer.next_token(), Some(Token::Int(2)));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn paren_1() {
        let mut lexer = SobaLexer::new("(1 + 2)".chars().collect());
        assert_eq!(lexer.next_token(), Some(Token::LeftParen));
        assert_eq!(lexer.next_token(), Some(Token::Int(1)));
        assert_eq!(lexer.next_token(), Some(Token::Plus));
        assert_eq!(lexer.next_token(), Some(Token::Int(2)));
        assert_eq!(lexer.next_token(), Some(Token::RightParen));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn float_1() {
        let mut lexer = SobaLexer::new("3.14".chars().collect());
        assert_eq!(lexer.next_token(), Some(Token::Float(3.14)));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn float_2() {
        let mut lexer = SobaLexer::new(".5".chars().collect());
        assert_eq!(lexer.next_token(), Some(Token::Float(0.5)));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn float_3() {
        let mut lexer = SobaLexer::new("5.".chars().collect());
        assert_eq!(lexer.next_token(), Some(Token::Float(5.0)));
        assert_eq!(lexer.next_token(), None);
    }
}
