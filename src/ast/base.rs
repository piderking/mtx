use std::fmt::Debug;

use rustpython_parser::ast::{ExprName, Identifier};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ident {
    pub inner: String,
}

impl From<String> for Ident {
    fn from(value: String) -> Self {
        Self { inner: value }
    }
}
impl From<Identifier> for Ident {
    fn from(value: Identifier) -> Self {
        Self {
            inner: String::from(value.as_str()),
        }
    }
}

#[derive(PartialEq)]
pub enum Value {
    // Decimal Value for Char
    Char(i32),
    Number(f32),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Char(arg0) => f
                .debug_tuple("Char")
                .field(&format!(
                    "{} as {}",
                    (char::from_u32(*arg0 as u32).unwrap()),
                    arg0
                ))
                .finish(),
            Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
        }
    }
}

impl From<char> for Value {
    fn from(value: char) -> Value {
        Value::Char((value as u32) as i32)
    }
}
impl From<f32> for Value {
    fn from(value: f32) -> Value {
        Value::Number(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Value {
        Value::Number(value as f32)
    }
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn value_from_char() {
        const C: char = 'a';
        assert_eq!(Value::Char(97), C.into())
    }
}
