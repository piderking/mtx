## Parser

Parse from Left to Right

### Expression (In Order of How Parsed)
(EXPRESSION)
EXPRESSION + EXPRESSION
f(EXPRESSION)

x
4

#### How?
EXAMPLE: (f(x) + C) / 4
Div {"(f(x) + C)", "4"}
Div { Paren { "f(x) + C" }, "4" }
Div { Paren { Add {"f(x)" + "4" } }, "4" }
Div { Paren { Add { FuncCall { "x" }  + "4" } }, "4" }
Div { Paren { Add { FuncCall { VarRef { "x" } }  + "4" } }, "4" }
Div { Paren { Add { FuncCall { VarRef { "x" } }  +  Constant {4} } }, "4" }
Div { Paren { Add { FuncCall { VarRef { "x" } }  +  Constant {4} } }, Constant { 4 } }


Find the outer most whats the outermost function.
-> recursive until it repeats (splitting str)



// 

// Vec Based Parsers 
pub fn parse_addition(input: &str) -> IResult<&str, Add>{
    let symbol = Symbols::Addition.as_str();

    map((parse_section, tag(symbol), separated_list1(tag(symbol), parse_section)), 
    |(first, _, following)| {
        let mut v = vec![first];
        v.extend(following);
        
        Add{terms: v}
    }).parse(input)
}
pub fn parse_multiplication(input: &str) -> IResult<&str, Multi>{
        let symbol = Symbols::Multiplication.as_str();

    map((parse_section, tag(symbol), separated_list1(tag(symbol), parse_section)), 
    |(first, _, following)| {
        let mut v = vec![first];
        v.extend(following);
        
        Multi{terms: v}
    }).parse(input)
}

pub fn parse_opperations(input: &str) -> IResult<&str, Expression> {
    /*
    Opperations:
    - Parentheses
    - Exponetial
    - Root
    - Multiplication
    - Division
    - Addition
    - Subtraction

    
     */
    // find outer most opperation
    //alt((parse_addition, parse_multiplication))
    todo!()
}




pub fn parse_functioncall_raw(input: &str) -> IResult<&str, (Ident, Vec<Expression>)> {
    (parse_ident, delimited(tag("("), separated_list1(tag(","), parse_section), tag(")"))).parse(input)
    // Function Call = F(x)
}

pub fn parse_functioncall(input: &str) -> IResult<&str, Expression> {
    map(parse_functioncall_raw, |(i,v)| Expression::FunctionCall(i, v)).parse(input)

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