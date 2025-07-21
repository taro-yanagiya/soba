//! Parser module
//!
//! This module contains the parser implementation and precedence handling.

pub mod precedence;
pub mod parser;

pub use precedence::Precedence;
pub use parser::Parser;