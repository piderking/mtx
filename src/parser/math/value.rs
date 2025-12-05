use nom::Parser;
use nom::bytes::complete::take_until;
use nom::character::anychar;
use nom::{
    Err, IResult, Input, branch::alt, bytes::complete::tag, character::complete::char,
    combinator::map, number::complete::float,
};
use nom::sequence::delimited;


use crate::ast::base::Value;

// A parser that recognizes a float and maps it to Value::Float
pub fn parse_float(input: &str) -> IResult<&str, f32> {
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
pub fn parse_char(input: &str) -> IResult<&str, char> {
    anychar(input)
}

// A parser that recognizes a float and maps it to Value::Float
pub fn parse_char_value(input: &str) -> IResult<&str, Value> {
    let mut parser = delimited(tag("'"), parse_char, tag("'"));
    let value = parser.parse(input);
    if let Ok((out, c)) = value{
        Ok((out, c.into()))
    } else {
        Err(value.err().unwrap())
    }

}
// A parser that recognizes a float and maps it to Value::Float
pub fn parse_float_value(input: &str) -> IResult<&str, Value> {
    map(
        parse_float, 
        |f| f.into()
    ).parse(input)
    

}



pub fn parse_value(input: &str) -> IResult<&str, Value>{
    alt((parse_float_value, parse_char_value)).parse(input)
    
    
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn parse_constant_float() {
        
        let (remaining_input, output): (_, Value) = parse_value("12.0").expect("Should Work");
        assert_eq!(output, Value::Number(12.0));
    }
    #[test]
    fn parse_constant_char() {
        let (remaining_input, output): (_, Value) = parse_value("'a'").expect("Should Work");
        assert_eq!(output, 'a'.into());
    }
}
