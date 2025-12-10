
use std::fmt::Debug;

use nom::{
    IResult, Parser, branch::alt, bytes::complete::{is_not, tag}, character::complete::char, multi::{many0, separated_list0}, sequence::delimited
};

#[derive(Debug, PartialEq)]
pub enum Parentheses<'a> {
    Content(&'a str),
    Nested(Vec<Parentheses<'a>>),
}


// Main function to parse the entire input
pub fn parse_parens(input: &str) -> IResult<&str, Vec<Parentheses>> {
    // We expect the whole input to be parsable as an expression list
    expression_list(input)
}

// Parses a sequence of content or nested expressions
fn expression_list(input: &str) -> IResult<&str, Vec<Parentheses>> {
    many0(alt((nested_expression, content_fragment))).parse(input)
}


// Parses a single segment of text content (anything that is not a parenthesis)
fn content_fragment(input: &str) -> IResult<&str, Parentheses> {
    is_not("()").map(Parentheses::Content).parse(input)
}

// Parses a nested expression within parentheses
fn nested_expression(input: &str) -> IResult<&str, Parentheses> {
    delimited(
        char('('),          // Match opening parenthesis
        expression_list,    // Recursively parse the content inside
        char(')'),          // Match closing parenthesis
    )
    .map(Parentheses::Nested) // Map the inner Vec<Expression> into a Nested variant
    .parse(input)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_recursive() {
    let input = "start + (middle + (inner + aaa) + here) + end";

    match parse_parens(input) {
        Ok((remaining, result_vec)) => {
            println!("Remaining input: '{}'", remaining);
            println!("Parsed result: {:#?}", result_vec);
        }
        Err(e) => {
            eprintln!("Error parsing: {:?}", e);
        }
    }

}
    
}
