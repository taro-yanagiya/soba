# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Soba is a programming language implementation written in Rust. It's an expression-based language that supports arithmetic, logical, and comparison operations with integer, float, and boolean values.

## Common Development Commands

### Building and Running
- `cargo build` - Build the project
- `cargo run` - Run the interactive REPL
- `cargo test` - Run all tests
- `cargo fmt` - Format code
- `cargo clippy` - Lint code

### Testing
- `cargo test` - Run all unit tests
- `cargo test <test_name>` - Run specific test
- `cargo test --lib` - Run library tests only

## Architecture Overview

The codebase follows a standard interpreter architecture with clear separation of concerns:

### Core Pipeline
1. **Lexing** (`src/lexer/`) - Tokenizes input strings into tokens
2. **Parsing** (`src/parser/`) - Uses Pratt parsing to build AST from tokens  
3. **Evaluation** (`src/evaluator/`) - Evaluates AST expressions to produce values

### Key Modules

- **`src/lexer/`** - Tokenization with `SobaLexer` implementing `Lexer` trait
- **`src/parser/`** - Pratt parser with operator precedence handling
- **`src/ast/`** - AST node definitions (`Expr`, `BinaryOp`, `UnaryOp`)
- **`src/value.rs`** - Runtime value system (`Value` enum with Int/Float/Bool variants)
- **`src/evaluator/`** - Expression evaluation with comprehensive operation support
- **`src/error.rs`** - Unified error handling across all modules
- **`src/span.rs`** - Source position tracking for error reporting

### Value System
The `Value` enum supports:
- Arithmetic operations (add, subtract, multiply, divide)
- Logical operations (and, or, not)  
- Comparison operations (==, !=, <, >, <=, >=)
- Type coercion between numeric types
- Truthiness evaluation

### Parser Architecture
Uses Pratt parsing (operator precedence parsing) for expression parsing with precedence levels defined in `src/parser/precedence.rs`.

### Interactive REPL
The main binary (`src/main.rs`) provides an interactive REPL using `rustyline` with:
- Command history (stored in `.soba_history`)
- Exit command support
- Expression evaluation and result display

## Testing Approach

Tests are embedded within each module using `#[cfg(test)]`. Key test areas:
- Value operations and type coercion
- Expression evaluation
- Error handling scenarios
- Parser correctness

## Entry Points

- **Library usage**: Use `eval_string()` function from `src/lib.rs`
- **Interactive mode**: Run `cargo run` to start REPL
- **Testing**: Individual modules have comprehensive test suites