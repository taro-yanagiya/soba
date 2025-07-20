mod lexer;
mod parser;

use parser::Expr;
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

pub use crate::lexer::Lexer;
use crate::{lexer::SobaLexer, parser::Parser};
fn main() -> rustyline::Result<()> {
    println!("This is the Soba programming language!");
    
    let mut rl = DefaultEditor::new()?;
    
    // Set maximum history size to 1000 entries
    rl.set_max_history_size(1000)?;
    
    // Load history from file
    let history_file = ".soba_history";
    if rl.load_history(history_file).is_err() {
        // History file doesn't exist, that's fine
    }
    
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                // Add to history
                let _ = rl.add_history_entry(&line);
                
                if line.trim() == "exit" {
                    break;
                }
                
                if line.trim().is_empty() {
                    continue;
                }
                
                let mut lexer = SobaLexer::new(line.chars().collect());
                let mut parser = Parser::new(&mut lexer);
                
                match parser.parse() {
                    Some(expr) => {
                        println!("{}", eval(&expr));
                    }
                    None => {
                        println!("Parse error");
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    
    // Save history to file
    let _ = rl.save_history(history_file);
    
    Ok(())
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
