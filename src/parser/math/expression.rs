use nom::{IResult, Parser, bytes::complete::tag, character::complete::alpha1, combinator::map, multi::{many_till, separated_list1}, sequence::{delimited, tuple}};

use crate::{ast::{Expression, base::{Ident}}, parser::math::value::parse_float_value};


// Variable = x, y, z
// Ident = Function
// Ident(Vec<Variable>)
//
pub fn parse_variable_name(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}


pub fn parse_ident(input: &str) -> IResult<&str, Ident> {
    match parse_variable_name(input) {
        Ok((out, e)) => Ok((out, e.to_string().into())),
        Err(e) => Err(e),
    }
}



pub fn parse_opperations(input: &str) -> IResult<&str, Expression> {
    todo!()
}



pub fn parse_direct_functioncall(input: &str) -> IResult<&str, (Ident, Vec<Expression>)> {
    (parse_ident, delimited(tag("("), separated_list1(tag(","), parse_section), tag(")"))).parse(input)
    // Function Call = F(x)
}


pub fn parse_variableref(input: &str) -> IResult<&str, Expression> {
    // parse for function calls and then for variables
    map(alpha1, |f: &str| Expression::VariableRef(f.to_string().into())).parse(input)
}


pub fn parse_constant(input: &str) -> IResult<&str, Expression> {
    map(parse_float_value, |f| Expression::Constant(f)).parse(input)
}

pub fn parse_empty(input: &str) -> IResult<&str, Expression> {
// Parse Expression Here
    if input.is_empty(){
        return Ok((input, Expression::Empty))
    }
    // was empty when it shouldn't of been
    Err(nom::Err::Incomplete(nom::Needed::Unknown) )

}

// Expression
pub fn parse_section(input: &str) -> IResult<&str, Expression> {
    
    // pull pattern
    //multiple different patterns into different variations of enum nom rust
    

    todo!()
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    
    #[test]
    fn parse_ident_name() {
        assert_eq!(parse_ident("a").unwrap().1, Ident{ inner: format!("a")} )
    }
    
}