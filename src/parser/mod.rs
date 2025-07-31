//! Parser module
//!
//! This module contains the parser implementation and precedence handling.

pub mod pratt;
pub mod precedence;

pub use pratt::Parser;
pub use precedence::Precedence;
