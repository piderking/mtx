use nom::{IResult, Parser, branch::alt, bytes::complete::tag, character::complete::{alpha0, alpha1, char, digit1, multispace0}, combinator::{all_consuming, map, map_res, peek, recognize}, error::context, multi::{many_till, many0, separated_list1}, number::complete::f32, sequence::{delimited, pair, preceded, separated_pair, tuple}};

use crate::{ast::{Add, Exp, Expression, Index, Multi, Opperation, Root, base::Ident}, parser::math::{value::{parse_float_value, parse_value}, whitespace::ws}, symbols::Symbols};


// Variable = x, y, z
// Ident = Function
// Ident(Vec<Variable>)
//


pub fn parse_ident(input: &str) -> IResult<&str, Ident> {
    log::debug!(">>> parse_ident {:?}", input);

    match alpha1(input) {
        Ok((out, e)) => Ok((out, e.to_string().into())),
        Err(e) => Err(e),
    }
}




// Exponentiation (highest precedence, right-associative)
pub fn parse_exponentiation(input: &str) -> IResult<&str, Expression> {
     log::debug!(">>> parse_exponentiation {:?}", input);

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

pub fn parse_index(input: &str) -> IResult<&str, Expression> {
    log::debug!(">>> parse_index {:?}", input);

   let (input, base_name) = alpha1(input)?;
   let base_ident = Ident { inner: base_name.to_string() };

   let result = ((
    tag("_"),
    alt((
        map(parse_value, Expression::Constant),
        map(alpha1, |s: &str| Expression::VariableRef(Ident{ inner: s.to_string()}))
    ))
   )).parse(input);

   match result {
    Ok((input, (_, index))) => Ok((input, Expression::Opperations(Box::new(
        Index {
            base: Expression::VariableRef(base_ident),
            index, 
        }
    )))),
    Err(_) => {
        Ok((input, Expression::VariableRef(base_ident)))
    }
   }
}


pub fn parse_root(input: &str) -> IResult<&str, Expression> {
    log::debug!(">>> parse_root {:?}", input);

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
    log::debug!(">>> parse_atom {:?}", input);
    preceded(
        multispace0,
        alt((

            parse_root,
            parse_parens,
            parse_index,
            parse_list,

            // variable ref is integrated into index
            //.map(parse_ident, Expression::VariableRef),
            map(parse_value, Expression::Constant),
            //map(ws(tag("")), |_| Expression::Empty)
        ))
    ).parse(input)
}

pub fn parse_multiplication(input: &str) -> IResult<&str, Expression> {
    log::debug!(">>> parse_multi {:?}", input);

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
    log::debug!(">>> parse_addition {:?}", input);

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
    log::debug!(">>> parse_paren {:?}", input);

    delimited(
        ws(tag("(")),
        parse_addition,  // Goes back to top-level expression
        ws(tag(")"))
    ).parse(input)
}

pub fn parse_list(input: &str) -> IResult<&str, Expression> {
    log::debug!(">>> parse_list {:?}", input);

    let (input, _ ) = ws(tag("[")).parse(input)?;
    let (input, items ) = separated_list1(ws(tag(",")), parse_addition).parse(input)?;
    let (input, _ ) = ws(tag("]")).parse(input)?;
    Ok((input, Expression::List(items)))
}
// TOP-LEVEL ENTRY POINT
pub fn pexp(input: &str) -> IResult<&str, Expression> {
    log::debug!(">>> pexp {:?}", input);
    // LISTS CAN ONLY EXSIST
    parse_addition(input)
}

    





macro_rules! test_expression {
    ($e:expr) => {
        #[cfg(debug_assertions)]
        {
                // `try_init()` prevents errors if called multiple times in parallel tests
                let _ = env_logger::builder().is_test(true).try_init();
                  let res = crate::parser::math::expression::pexp($e);
        
                match res {
                    Ok((remaining, expr)) => {
                        if remaining.is_empty() {
                            log::debug!("✓ {} => {:?}", $e, expr);
                        } else {
                            log::debug!("⚠ {} => {:?} (unparsed: {:?})", $e, expr, remaining);
                        }
                    }
                    Err(err) => {
                        log::debug!("✗ {} => Error: {:?}", $e, err);
                    }
                }
            };
        };
      
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test_index() {
        test_expression!("x_x");
        // test_expression!("x_(x+1)");
    }
    #[test]
    fn test_advanced() {
        test_expression!("");
    }
    #[test]
    fn test_basic_list() {
        test_expression!("[1+(a*3)]");
    }

    #[test]
    fn test_basic_root() {
        test_expression!("root(3, x)");
    }


    #[test]
    fn test_multipilicaction() {
        let input = "(a)1+'a'";
        let result = pexp(input);
        log::debug!("{:?}", result);
        assert!(result.is_ok());
    }
    #[test]
    fn test_nested_addition() {
        let input = "2(a+2)+2";
        let (output, result) = pexp(input).expect("msg");
        log::debug!("{:#?}", result);

        assert_eq!(output.len(), 0)
    }

    #[test]
    fn large_group_addition() {
        let input = "(4+4+1+3)+(a+45)";
        let result = pexp(input);
        log::debug!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_constant_parens(){
        let input = "(   (123 )    )";
        log::debug!("{:?}", pexp(input))
    }

    #[test]
    fn test_addition(){
        let input = "1+(1+2)";
        log::debug!("{:?}", pexp(input))
    }
    #[test]
    fn test_constant(){
        let input = "1";
        log::debug!("{:?}", pexp(input))
    }
    
    #[test]
    fn parse_ident_name() {
        assert_eq!(parse_ident("a").unwrap().1, Ident{ inner: format!("a")} )
    }
   
    
}