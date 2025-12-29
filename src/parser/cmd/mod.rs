use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag, character::complete::alpha1,
    combinator::map,
};

#[derive(Debug, Clone)]
pub enum ArgType {
    // -s or --super
    Flag(FlagData),

    // -s=""
    Argument(ArgData),
}

#[derive(Debug, Clone)]
pub struct ArgData {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct FlagData {
    pub name: String,
}

// Command Line Parser
pub fn parse_arg(input: &str) -> IResult<&str, ArgData> {
    todo!()
}
// Command Line Parser
pub fn parse_flag(input: &str) -> IResult<&str, FlagData> {
    map((tag("-"), alpha1), |(_, d): (&str, &str)| FlagData {
        name: d.to_string(),
    })
    .parse(input)
}

// Command Line Parser
pub fn parse_arg_type(input: &str) -> IResult<&str, ArgType> {
    alt((
        map(parse_arg, ArgType::Argument),
        map(parse_flag, ArgType::Flag),
    ))
    .parse(input)
}
