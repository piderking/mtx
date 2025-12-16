use nom::{IResult, Parser, bytes::complete::tag, character::complete::alpha1, combinator::map, multi::separated_list1, sequence::delimited};

use crate::{ast::{Definition, base::Ident}, parser::math::{expression::{parse_ident, pexp}, whitespace::ws}};



// PARENTHESES - recursion point
pub fn parse_function_def(input: &str) -> IResult<&str, Definition> {
    map((ws(parse_ident), delimited(ws(tag("(")), separated_list1(ws(tag(",")), ws(parse_ident)), ws(tag(")"))), tag("="), ws(pexp) ), |(ident, idents, _, expr )| Definition::Function(ident, idents, expr)).parse(input)
}


// PARENTHESES - recursion point
pub fn parse_constant_def(input: &str) -> IResult<&str, Definition> {
    map((ws(parse_ident), tag("="), ws(pexp) ), |(ident, _, expr )| Definition::Constant(ident, expr)).parse(input)
}




#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn parse_variable_name() {
    }
   
    
}