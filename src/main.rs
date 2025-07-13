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
        Expr::Grouped(inner) => eval(inner),
        Expr::UnaryExpr { op, operand } => {
            let value = eval(operand);
            match op {
                parser::UnaryOp::Plus => value,
                parser::UnaryOp::Minus => -value,
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

    #[test]
    fn test_eval_grouped() {
        do_eval("(1 + 2) * 3", 9);
        do_eval("1 + (2 * 3)", 7);
        do_eval("(1 + 2) * (3 + 4)", 21);
        do_eval("((1 + 2) * 3)", 9);
    }

    #[test]
    fn test_eval_unary() {
        do_eval("-1 + 3", 2);
        do_eval("+3 - 1", 2);
        do_eval("2 + (-4)", -2);
        do_eval("(-2) * 5", -10);
        do_eval("+5", 5);
        do_eval("-10", -10);
    }

    fn do_eval(input: &str, expect: i32) {
        let mut lexer = SobaLexer::new(input.chars().collect());
        let mut parser = parser::Parser::new(&mut lexer);
        let result = eval(&parser.parse().unwrap());
        assert_eq!(result, expect);
    }
}
