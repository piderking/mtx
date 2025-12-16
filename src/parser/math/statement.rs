use nom::{IResult, Parser, branch::alt, bytes::complete::{is_not, tag, take_until}, character::complete::{alpha1, newline}, combinator::map, multi::separated_list1, sequence::{delimited, preceded}};

use crate::{ast::{Comment, Definition, Statement, base::Ident}, parser::math::{expression::{parse_ident, pexp}, whitespace::ws}};






// PARENTHESES - recursion point
pub fn parse_function_def(input: &str) -> IResult<&str, Definition> {
    map((ws(parse_ident), delimited(ws(tag("(")), separated_list1(ws(tag(",")), ws(parse_ident)), ws(tag(")"))), tag("="), ws(pexp) ), |(ident, idents, _, expr )| Definition::Function(ident, idents, expr)).parse(input)
}


// PARENTHESES - recursion point
pub fn parse_constant_def(input: &str) -> IResult<&str, Definition> {
    map((ws(parse_ident), tag("="), ws(pexp) ), |(ident, _, expr )| Definition::Constant(ident, expr)).parse(input)
}

pub fn parse_definition(input: &str) -> IResult<&str, Definition> {
    alt((parse_function_def, parse_constant_def)).parse(input)
}



pub fn parse_single_comment(input: &str) -> IResult<&str, Comment> {
    let (input, (_, v, _) ) = (tag("//"), is_not("\n"), newline).parse(input)?;
    Ok((input, Comment::Single(v.to_string())))
}
pub fn parse_multi_comment(input: &str) -> IResult<&str, Comment> {
    map(delimited(tag("/*"), ws(is_not("*/")), tag("*/")), |f: &str| Comment::Multi(f.to_string())).parse(input)
}


pub fn parse_comment(input: &str) -> IResult<&str, Comment> {
    alt((parse_single_comment, parse_multi_comment)).parse(input)
}

pub fn parse_statement(input: &str) -> IResult<&str, Statement> {
    alt((
        map(parse_definition, Statement::Definition),
        map(parse_comment, Statement::Comment),
        map(pexp, Statement::Expression),
    )).parse(input)

}


macro_rules! panic_if_err {
    ($e:expr) => {
        match $e {
            Ok(v) => println!("{:?}", v),
            Err(error) => panic!("{:?}", error),
    };
}
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_statement() {
        let res = parse_statement("55 * 555 + 4 \n");
        panic_if_err!(res);
    }   

    #[test]
    fn test_parse_comment() {
        let res = parse_comment("// hello \n");
        panic_if_err!(res);
    }
    #[test]
    fn test_multi_comment() {
        let res = parse_comment("/* hello hello*/");
        panic_if_err!(res);
    }
    #[test]
    fn test_pase_function_def() {
        let res = parse_definition("f(x,y,z) = 5");
        panic_if_err!(res);
    }

     #[test]
    fn test_const_def() {
        let res = parse_definition("x = 5");
        panic_if_err!(res);
    }
   
    
}