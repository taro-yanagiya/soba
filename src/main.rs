mod lexer;
mod parser;

use parser::Expr;
use std::io::{self, Write};

pub use crate::lexer::Lexer;
use crate::{lexer::SobaLexer, parser::Parser};
fn main() {
    println!("This is the Soba programming language!");
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("failed to read line");

        if input == "exit\n" {
            break;
        }

        let mut lexer = SobaLexer::new(input.chars().collect());
        let mut parser = Parser::new(&mut lexer);

        let expr = parser.parse();

        if let Some(expr) = expr {
            println!("{}", eval(&expr))
        }
    }
}

fn eval(expr: &Expr) -> i32 {
    match expr {
        Expr::Int(n) => *n,
        Expr::InfixExpr { left, op, right } => {
            let left = eval(left);
            let right = eval(right);
            match op {
                parser::Op::Plus => left + right,
                parser::Op::Minus => left - right,
                parser::Op::Asterisk => left * right,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::SobaLexer;

    use super::*;

    #[test]
    fn test_eval() {
        do_eval("1 + 2", 3);
        do_eval("1 + 2 + 3", 6);
        do_eval("1 + 2 + 3 - 4", 2);
        do_eval("3 * 3 + 1", 10);
        do_eval("1 + 2 * 3", 7);
    }

    fn do_eval(input: &str, expect: i32) {
        let mut lexer = SobaLexer::new(input.chars().collect());
        let mut parser = parser::Parser::new(&mut lexer);
        let result = eval(&parser.parse().unwrap());
        assert_eq!(result, expect);
    }
}
