use rustpython_parser::ast::Identifier;

#[derive(Debug)]
pub struct Ident {
    inner: String,
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
#[derive(Debug)]
pub struct Variable {
    inner: String,
}

impl From<String> for Variable {
    fn from(value: String) -> Self {
        Self { inner: value }
    }
}
impl From<Identifier> for Variable {
    fn from(value: Identifier) -> Self {
        Self {
            inner: String::from(value.as_str()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    // Decimal Value for Char
    Char(i32),
    Number(f32),
}

impl From<char> for Value {
    fn from(value: char) -> Value {
        Value::Char((value as u32) as i32)
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
