use std::{fmt::Display, result};
pub mod chunk;
pub mod objects;
pub mod types;
use objects::ObjectPointer;
pub mod prelude {
    pub use super::{
        super::err::TryFromValueError,
        chunk::{Chunk, OpCode},
        objects::{ObjString, Object, ObjectPointer, StringPointer},
        Number, Result as ValResult, Type,
    };
}

use prelude::*;
pub type Result<T> = result::Result<T, TryFromValueError>;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeId {
    Number,
    Bool,
    String,
    Module,
    #[default]
    Nil,
    Custom(StringPointer),
}

pub type Number = i32;
#[derive(Default, PartialEq, PartialOrd, Eq, Debug, Clone, Copy)]
pub enum Type {
    Number(Number),
    Bool(bool),
    Object(ObjectPointer),
    #[default]
    Nil,
}
impl Type {
    pub fn is_falsy(&self) -> bool {
        match self {
            Self::Nil => true,
            Self::Bool(b) => !b,
            Self::Number(_) => false,
            Self::Object(_) => false,
        }
    }
    pub fn types_equal(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Bool(_), Type::Bool(_))
            | (Type::Number(_), Type::Number(_))
            | (Type::Nil, _) => true,
            (Type::Object(old_ptr), Type::Object(new)) => match (old_ptr, new) {
                (ObjectPointer::String(_), ObjectPointer::String(_)) => true,
            },
            _ => false,
        }
    }
}
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bool(b) => b.to_string(),
                Self::Number(n) => n.to_string(),
                Self::Nil => "nil".to_string(),
                Self::Object(o) => format!("{}", o),
            }
        )
    }
}

impl From<i32> for Type {
    fn from(n: i32) -> Self {
        Self::Number(n)
    }
}

impl From<bool> for Type {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl TryFrom<Type> for i32 {
    type Error = TryFromValueError;
    fn try_from(value: Type) -> result::Result<Self, Self::Error> {
        let error = |got: &str| TryFromValueError::new("number", got);
        match value {
            Type::Number(n) => Ok(n),
            Type::Nil => error("nil"),
            Type::Bool(_) => error("bool"),
            Type::Object(_) => error("object"),
        }
    }
}

impl TryFrom<Type> for bool {
    type Error = TryFromValueError;
    fn try_from(value: Type) -> result::Result<Self, Self::Error> {
        let error = |got: &str| TryFromValueError::new("bool", got);
        match value {
            Type::Bool(b) => Ok(b),
            Type::Nil => error("nil"),
            Type::Number(_) => error("number"),
            Type::Object(_) => error("object"),
        }
    }
}
