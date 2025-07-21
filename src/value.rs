//! Value system for the Soba programming language

use crate::error::{EvalError, EvalResult};
use std::fmt;

/// Runtime values in Soba
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(i32),
    Float(f64),
    Bool(bool),
}

impl Value {
    /// Get the type name of this value
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::Bool(_) => "bool",
        }
    }

    /// Convert to f64 for arithmetic operations
    pub fn as_f64(&self) -> f64 {
        match self {
            Value::Int(i) => *i as f64,
            Value::Float(f) => *f,
            Value::Bool(b) => if *b { 1.0 } else { 0.0 },
        }
    }

    /// Convert to integer if possible
    pub fn as_int(&self) -> Option<i32> {
        match self {
            Value::Int(i) => Some(*i),
            Value::Float(f) => {
                if f.fract() == 0.0 && *f >= i32::MIN as f64 && *f <= i32::MAX as f64 {
                    Some(*f as i32)
                } else {
                    None
                }
            }
            Value::Bool(b) => Some(if *b { 1 } else { 0 }),
        }
    }

    /// Check if this value is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::Bool(b) => *b,
        }
    }

    // Arithmetic operations
    pub fn add_value(self, other: Value) -> EvalResult<Value> {
        let result = self.as_f64() + other.as_f64();
        Ok(Value::Float(result))
    }

    pub fn subtract_value(self, other: Value) -> EvalResult<Value> {
        let result = self.as_f64() - other.as_f64();
        Ok(Value::Float(result))
    }

    pub fn multiply_value(self, other: Value) -> EvalResult<Value> {
        let result = self.as_f64() * other.as_f64();
        Ok(Value::Float(result))
    }

    pub fn divide_value(self, other: Value) -> EvalResult<Value> {
        let other_val = other.as_f64();
        if other_val == 0.0 {
            Err(EvalError::DivisionByZero)
        } else {
            let result = self.as_f64() / other_val;
            Ok(Value::Float(result))
        }
    }

    pub fn negate(self) -> EvalResult<Value> {
        match self {
            Value::Int(i) => {
                i.checked_neg()
                    .map(Value::Int)
                    .ok_or(EvalError::Overflow)
            }
            Value::Float(f) => Ok(Value::Float(-f)),
            Value::Bool(_) => Err(EvalError::TypeError("Cannot negate boolean value".to_string())),
        }
    }

    pub fn positive(self) -> EvalResult<Value> {
        Ok(self)
    }

    // Logical operations
    pub fn logical_not(self) -> EvalResult<Value> {
        Ok(Value::Bool(!self.is_truthy()))
    }

    pub fn logical_and(self, other: Value) -> EvalResult<Value> {
        if !self.is_truthy() {
            Ok(Value::Bool(false))
        } else {
            Ok(Value::Bool(other.is_truthy()))
        }
    }

    pub fn logical_or(self, other: Value) -> EvalResult<Value> {
        if self.is_truthy() {
            Ok(Value::Bool(true))
        } else {
            Ok(Value::Bool(other.is_truthy()))
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{i}"),
            Value::Float(fl) => {
                // Display integers as integers even when they're floats
                if fl.fract() == 0.0 && *fl >= i32::MIN as f64 && *fl <= i32::MAX as f64 {
                    write!(f, "{}", *fl as i64)
                } else {
                    write!(f, "{fl}")
                }
            }
            Value::Bool(b) => write!(f, "{b}"),
        }
    }
}

impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Value::Int(i)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(f)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic() {
        let a = Value::Int(5);
        let b = Value::Float(2.5);

        assert_eq!(a.clone().add_value(b.clone()).unwrap(), Value::Float(7.5));
        assert_eq!(a.clone().subtract_value(b.clone()).unwrap(), Value::Float(2.5));
        assert_eq!(a.clone().multiply_value(b.clone()).unwrap(), Value::Float(12.5));
        assert_eq!(a.clone().divide_value(b.clone()).unwrap(), Value::Float(2.0));
    }

    #[test]
    fn test_division_by_zero() {
        let a = Value::Int(5);
        let b = Value::Int(0);
        assert!(matches!(a.divide_value(b), Err(EvalError::DivisionByZero)));
    }

    #[test]
    fn test_display() {
        assert_eq!(Value::Int(42).to_string(), "42");
        assert_eq!(Value::Float(3.14).to_string(), "3.14");
        assert_eq!(Value::Float(5.0).to_string(), "5");
    }

    #[test]
    fn test_logical_not() {
        assert_eq!(Value::Bool(true).logical_not().unwrap(), Value::Bool(false));
        assert_eq!(Value::Bool(false).logical_not().unwrap(), Value::Bool(true));
        assert_eq!(Value::Int(0).logical_not().unwrap(), Value::Bool(true));
        assert_eq!(Value::Int(5).logical_not().unwrap(), Value::Bool(false));
    }

    #[test]
    fn test_logical_and() {
        assert_eq!(Value::Bool(true).logical_and(Value::Bool(true)).unwrap(), Value::Bool(true));
        assert_eq!(Value::Bool(true).logical_and(Value::Bool(false)).unwrap(), Value::Bool(false));
        assert_eq!(Value::Bool(false).logical_and(Value::Bool(true)).unwrap(), Value::Bool(false));
        assert_eq!(Value::Bool(false).logical_and(Value::Bool(false)).unwrap(), Value::Bool(false));
    }

    #[test]
    fn test_logical_or() {
        assert_eq!(Value::Bool(true).logical_or(Value::Bool(true)).unwrap(), Value::Bool(true));
        assert_eq!(Value::Bool(true).logical_or(Value::Bool(false)).unwrap(), Value::Bool(true));
        assert_eq!(Value::Bool(false).logical_or(Value::Bool(true)).unwrap(), Value::Bool(true));
        assert_eq!(Value::Bool(false).logical_or(Value::Bool(false)).unwrap(), Value::Bool(false));
    }

    #[test]
    fn test_is_truthy() {
        // Boolean values
        assert!(Value::Bool(true).is_truthy());
        assert!(!Value::Bool(false).is_truthy());
        
        // Integer values
        assert!(Value::Int(1).is_truthy());
        assert!(Value::Int(-1).is_truthy());
        assert!(!Value::Int(0).is_truthy());
        
        // Float values
        assert!(Value::Float(1.0).is_truthy());
        assert!(Value::Float(-1.0).is_truthy());
        assert!(!Value::Float(0.0).is_truthy());
    }
}