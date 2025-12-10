use nom::character::complete::multispace0;
use nom::error::ParseError;
use nom::sequence::delimited;
use nom::{IResult, Parser};

pub fn ws<'a, O, E: ParseError<&'a str>>(
    mut inner: impl FnMut(&'a str) -> IResult<&'a str, O, E>
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
{
    move |input| delimited(multispace0, &mut inner, multispace0).parse(input)
}