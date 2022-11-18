use std::{fmt::Display, result};
pub mod chunk;

pub type Result<T> = result::Result<T, TryFromValueError>;
#[derive(Debug)]
pub struct TryFromValueError {
    pub expected: String,
    pub got: String,
}
impl TryFromValueError {
    fn new<T>(expected: &str, got: &str) -> Result<T> {
        Err(Self {
            expected: expected.into(),
            got: got.into(),
        })
    }
}
impl Display for TryFromValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expected {}, found {}", self.expected, self.got)
    }
}

#[derive(PartialEq, PartialOrd, Eq, Debug, Clone, Copy)]
pub enum Value {
    Number(i32),
    Bool(bool),
    Nil,
}
impl Value {
    pub fn is_falsy(&self) -> bool {
        match self {
            Self::Nil => true,
            Self::Bool(b) => !b,
            Self::Number(_) => false,
        }
    }
}
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bool(b) => b.to_string(),
                Self::Number(n) => n.to_string(),
                Self::Nil => "nil".to_string(),
            }
        )
    }
}

impl From<i32> for Value {
    fn from(n: i32) -> Self {
        Self::Number(n)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl TryFrom<Value> for i32 {
    type Error = TryFromValueError;
    fn try_from(value: Value) -> result::Result<Self, Self::Error> {
        let error = |got: &str| TryFromValueError::new("number", got);
        match value {
            Value::Number(n) => Ok(n),
            Value::Nil => error("nil"),
            Value::Bool(_) => error("bool"),
        }
    }
}

impl TryFrom<Value> for bool {
    type Error = TryFromValueError;
    fn try_from(value: Value) -> result::Result<Self, Self::Error> {
        let error = |got: &str| TryFromValueError::new("bool", got);
        match value {
            Value::Bool(b) => Ok(b),
            Value::Nil => error("nil"),
            Value::Number(_) => error("number"),
        }
    }
}
