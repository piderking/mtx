use nom::{IResult, Parser, branch::alt, bytes::complete::tag, character::complete::{alpha0, alpha1, char, digit1, multispace0}, combinator::{all_consuming, map, map_res, peek, recognize}, error::context, multi::{many_till, many0, separated_list1}, number::complete::f32, sequence::{delimited, pair, preceded, separated_pair, tuple}};

use crate::{ast::{Add, Exp, Expression, Multi, Opperation, Root, base::Ident}, parser::math::{value::{parse_float_value, parse_value}, whitespace::ws}, symbols::Symbols};


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


// Exponentiation (highest precedence, right-associative)
pub fn parse_exponentiation(input: &str) -> IResult<&str, Expression> {
    let (input, base) = parse_atom(input)?;
    
    // Check if there's a ^ operator
    let result = preceded(ws(tag("^")), parse_exponentiation).parse(input);
    
    match result {
        Ok((input, exponent)) => {
            // Right-associative: 2^3^4 = 2^(3^4)
            Ok((input, Expression::Opperations(Box::new(Exp{
                base,
                exponent
            }))))
        }
        Err(_) => {
            // No exponent, just return the base
            Ok((input, base))
        }
    }
}

pub fn parse_root(input: &str) -> IResult<&str, Expression> {
    alt((
        // root(n, x) - nth root
        delimited(
            (ws(tag("root")), ws(tag("("))),
            map(
                separated_pair(
                    parse_addition,      // First arg: n (the root degree)
                    ws(tag(",")),
                    parse_addition       // Second arg: x (the radicand)
                ),
                |(n, x)| {
                    Expression::Opperations(Box::new(Root{
                        degree: n,
                        radicand: x
                    }))
                }
            ),
            ws(tag(")"))
        ),
        // sqrt(x) - square root (syntactic sugar for root(2, x))
        delimited(
            (ws(tag("sqrt")), ws(tag("("))),
            map(parse_addition, |arg| {
                Expression::Opperations(Box::new(Root{
                    degree: Expression::Constant(2.0.into()),
                    radicand: arg
                }))
            }),
            ws(tag(")"))
        ),
        // √x - unicode square root
        preceded(
            ws(tag("√")),
            map(parse_atom, |arg| {
                Expression::Opperations(Box::new(Root{
                    degree: Expression::Constant(2.0.into()),
                    radicand: arg
                }))
            })
        ),
    )).parse(input)
}


// ATOMS - lowest level (no operators)
pub fn parse_atom(input: &str) -> IResult<&str, Expression> {
    preceded(
        multispace0,
        alt((
            parse_root,
            parse_parens,
            map(parse_ident, Expression::VariableRef),
            map(parse_value, Expression::Constant),
        ))
    ).parse(input)
}

pub fn parse_multiplication(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_exponentiation(input)?;
    let (input, rest) = many0(
        alt((
            // Explicit multiplication with *
            preceded(ws(tag(Symbols::Multiplication.as_str())), parse_exponentiation),
            // Implicit: followed by (, identifier, or digit
            preceded(
                peek(alt((
                    tag("("),
                    recognize(alpha1),     // Variables like x, y
                    recognize(digit1),     // Numbers like 1, 23
                ))), 
                parse_atom
            ),
        ))
    ).parse(input)?;
    
    if rest.is_empty() {
        Ok((input, first))
    } else {
        let mut terms = vec![first];
        terms.extend(rest);
        Ok((input, Expression::Opperations(Box::new(Multi{terms}))))
    }
}
// ADDITION - lower precedence
pub fn parse_addition(input: &str) -> IResult<&str, Expression> {
    let (input, first) = parse_multiplication(input)?;
    let (input, rest) = many0(preceded(ws(tag(Symbols::Addition.as_str())), parse_multiplication)).parse(input)?;
    
    if rest.is_empty() {
        Ok((input, first))
    } else {
        let mut terms = vec![first];
        terms.extend(rest);
        Ok((input, Expression::Opperations(Box::new(Add{terms}))))
    }
}

// PARENTHESES - recursion point
pub fn parse_parens(input: &str) -> IResult<&str, Expression> {
    delimited(
        ws(tag("(")),
        parse_addition,  // Goes back to top-level expression
        ws(tag(")"))
    ).parse(input)
}

// TOP-LEVEL ENTRY POINT
pub fn pexp(input: &str) -> IResult<&str, Expression> {
    parse_addition(input)
}

    





macro_rules! test_expression {
    ($e:expr) => {
        let res = crate::parser::math::expression::pexp($e);
        
        match res {
            Ok((remaining, expr)) => {
                if remaining.is_empty() {
                    println!("✓ {} => {:?}", $e, expr);
                } else {
                    println!("⚠ {} => {:?} (unparsed: {:?})", $e, expr, remaining);
                }
            }
            Err(err) => {
                println!("✗ {} => Error: {:?}", $e, err);
            }
        }
    };
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    


    #[test]
    fn test_basic_root() {
        test_expression!("root(3, x)");
    }


    #[test]
    fn test_multipilicaction() {
        let input = "(a)1+'a'";
        let result = pexp(input);
        println!("{:?}", result);
        assert!(result.is_ok());
    }
    #[test]
    fn test_nested_addition() {
        let input = "2(a+2)+2";
        let (output, result) = pexp(input).expect("msg");
        println!("{:#?}", result);

        assert_eq!(output.len(), 0)
    }

    #[test]
    fn large_group_addition() {
        let input = "(4+4+1+3)+(a+45)";
        let result = pexp(input);
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_constant_parens(){
        let input = "(   (123 )    )";
        println!("{:?}", pexp(input))
    }

    #[test]
    fn test_addition(){
        let input = "1+(1+2)";
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