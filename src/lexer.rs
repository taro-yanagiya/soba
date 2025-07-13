#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Int(i32),
    Plus,
    Minus,
    Asterisk,
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

        let token = if is_number(current) {
            let mut number = vec![*current];
            while self.peek().is_some() && is_number(self.peek().unwrap()) {
                self.forward();
                number.push(*self.current().unwrap());
            }
            String::from_iter(number)
                .parse()
                .ok()
                .and_then(|n| Some(Token::Int(n)))
        } else {
            match current {
                '+' => Some(Token::Plus),
                '-' => Some(Token::Minus),
                '*' => Some(Token::Asterisk),
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
}
