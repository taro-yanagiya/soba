use crate::{lexer::Token, Lexer};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Int(i32),
    InfixExpr {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    LOWEST,
    SUM,
    PRODUCT,
}

impl Precedence {
    fn from_token(token: &Token) -> Precedence {
        match token {
            Token::Plus | Token::Minus => Precedence::SUM,
            Token::Asterisk => Precedence::PRODUCT,
            _ => Precedence::LOWEST,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Asterisk,
}

pub struct Parser<'a> {
    lexer: &'a mut dyn Lexer,
    current_token: Option<Token>,
    peek: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut dyn Lexer) -> Parser<'a> {
        let token = lexer.next_token();
        let peek = lexer.next_token();
        Parser {
            lexer,
            current_token: token,
            peek,
        }
    }

    pub fn parse(&mut self) -> Option<Box<Expr>> {
        self.parse_expr(Precedence::LOWEST)
    }

    fn parse_expr(&mut self, precedence: Precedence) -> Option<Box<Expr>> {
        let mut left = match self.current_token.as_ref()? {
            Token::Int(i) => Box::new(Expr::Int(*i)),
            _ => panic!("parse error"),
        };

        while self.peek.is_some()
            && precedence < Precedence::from_token(self.peek.as_ref().unwrap())
        {
            self.next_token();
            left = self.parse_infix(left)?;
        }
        Some(left)
    }

    // 演算子にフォーカスが当たっている状態で呼び出す。Expr::InfixExprを返す。
    fn parse_infix(&mut self, left: Box<Expr>) -> Option<Box<Expr>> {
        let token = self.current_token.as_ref()?;
        let op = match token {
            Token::Plus => Op::Plus,
            Token::Minus => Op::Minus,
            Token::Asterisk => Op::Asterisk,
            _ => panic!("parse error"),
        };
        let op_precedence = Precedence::from_token(token);

        self.next_token();
        let right = self.parse_expr(op_precedence)?;

        Some(Box::new(Expr::InfixExpr { left, op, right }))
    }

    fn next_token(&mut self) {
        self.current_token = std::mem::replace(&mut self.peek, None);
        self.peek = self.lexer.next_token();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeLexer {
        tokens: Vec<Token>,
        position: usize,
    }

    impl Lexer for FakeLexer {
        fn next_token(&mut self) -> Option<Token> {
            let token = self.tokens.get(self.position).cloned();
            self.position += 1;
            token
        }
    }

    #[test]
    fn test_parse_plus_1() {
        assert_parse(
            vec![Token::Int(1), Token::Plus, Token::Int(2)],
            Expr::InfixExpr {
                left: Box::new(Expr::Int(1)),
                op: Op::Plus,
                right: Box::new(Expr::Int(2)),
            },
        )
    }

    #[test]
    fn test_parse_plus_2() {
        assert_parse(
            vec![
                Token::Int(1),
                Token::Plus,
                Token::Int(2),
                Token::Plus,
                Token::Int(3),
            ],
            Expr::InfixExpr {
                left: Box::new(Expr::InfixExpr {
                    left: Box::new(Expr::Int(1)),
                    op: Op::Plus,
                    right: Box::new(Expr::Int(2)),
                }),
                op: Op::Plus,
                right: Box::new(Expr::Int(3)),
            },
        )
    }

    #[test]
    fn test_parse_minus_1() {
        assert_parse(
            vec![Token::Int(1), Token::Minus, Token::Int(2)],
            Expr::InfixExpr {
                left: Box::new(Expr::Int(1)),
                op: Op::Minus,
                right: Box::new(Expr::Int(2)),
            },
        )
    }

    #[test]
    fn test_parse_multiply_1() {
        assert_parse(
            vec![Token::Int(1), Token::Asterisk, Token::Int(2)],
            Expr::InfixExpr {
                left: Box::new(Expr::Int(1)),
                op: Op::Asterisk,
                right: Box::new(Expr::Int(2)),
            },
        )
    }

    #[test]
    fn test_product_precedence_1() {
        assert_parse(
            vec![
                Token::Int(1),
                Token::Plus,
                Token::Int(2),
                Token::Asterisk,
                Token::Int(3),
            ],
            Expr::InfixExpr {
                left: Box::new(Expr::Int(1)),
                op: Op::Plus,
                right: Box::new(Expr::InfixExpr {
                    left: Box::new(Expr::Int(2)),
                    op: Op::Asterisk,
                    right: Box::new(Expr::Int(3)),
                }),
            },
        )
    }

    fn assert_parse(input: Vec<Token>, expect: Expr) {
        let mut lexer = FakeLexer {
            tokens: input,
            position: 0,
        };
        let mut parser = Parser::new(&mut lexer);
        let expr = parser.parse().unwrap();
        assert_eq!(expr, Box::new(expect));
    }
}
