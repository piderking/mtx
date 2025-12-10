use nom::{IResult, Parser, branch::alt, bytes::complete::tag, character::complete::{alpha0, alpha1, char, multispace0}, combinator::{all_consuming, map, map_res}, error::context, multi::{many_till, separated_list1}, number::complete::f32, sequence::{delimited, pair, tuple}};

use crate::{ast::{Add, Expression, Multi, Opperation, base::Ident}, parser::math::{value::{parse_float_value, parse_value}, whitespace::ws}, symbols::Symbols};


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

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    todo!()
}



pub fn parse_adition(input: &str) -> IResult<&str, Add> {
    // Seperate Terms by + 
    //separated_list1(tag("+"), alt( (parse_terms, parse_term)  )).parse(input)
    todo!("")
}

pub fn parse_parens(input: &str) -> IResult<&str, Expression> {
    delimited(
        tag("("),
        ws(parse_term), // Handle whitespace
        tag(")")
    ).parse(input)
    
}

pub fn parse_term (input: &str) -> IResult<&str, Expression> {
    alt((
        // Parse Variable and Constants First (SO DOESN'T INFINITE LOOP) 
        // Parse Variable
        map(parse_ident, Expression::VariableRef),
        // Parse Constants Last
        map(parse_value, |f| {
            println!("Parsed constant: {:?}", f);
            Expression::Constant(f)
        }),

        // Parse Parenthesis First 
        parse_parens,

        // Parse Multiplication (Sign)

        

        // Parse Addition
    )).parse(input)
}


    

pub fn pexp(input: &str) -> IResult<&str, Expression> {
    delimited(multispace0, parse_term, multispace0).parse(input)
    
}





    



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test_constant_parens(){
        let input = "(   123)";
        println!("{:?}", pexp(input))
    }
    #[test]
    fn test_constant(){
        let input = "1";
        println!("{:?}", pexp(input))
    }
    
    #[test]
    fn parse_ident_name() {
        assert_eq!(parse_ident("a").unwrap().1, Ident{ inner: format!("a")} )
    }
   
    
}