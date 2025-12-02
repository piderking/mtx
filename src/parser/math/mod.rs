use nom::{
    Err, IResult, Input, branch::alt, bytes::complete::tag, character::complete::char,
    combinator::map, number::complete::float,
};

use crate::ast::base::Value;

// A parser that recognizes a float and maps it to Value::Float
pub fn parse_value_float(input: &str) -> IResult<&str, f32> {
    let (rest, value) = float(input)?;

    // STRICT mode: only accept it if it consumed the *entire* input token
    if rest.is_empty() {
        return Ok(("", value));
    }

    return Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Float,
    )));
}
// A parser that recognizes a float and maps it to Value::Float
pub fn parse_value_char(input: &str) -> IResult<&str, char> {
    if let Option::Some(c) = input.chars().next() {
        return Ok(("", c));
    }

    return Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Char,
    )));
}

pub fn parse_value(input: &str) -> IResult<&str, Value>{
    
    if let Ok((out, f)) = parse_value_float(input){
        return Ok((out, f.into()))
    } else if let Ok((out, c)) = parse_value_char(input) {
        return Ok((out, c.into()))
    }  else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Char,
        )))
    }
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn parse_constant_float() {
        
        let (remaining_input, output): (_, Value) = parse_value("12").expect("Should Work");
        assert_eq!(output, Value::Number(12.0));
    }
    #[test]
    fn parse_constant_char() {
        let (remaining_input, output): (_, Value) = parse_value("a").expect("Should Work");
        assert_eq!(output, 'a'.into());
    }
}
