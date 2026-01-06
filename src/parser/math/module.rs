use crate::{
    ast::module::{Metadata, Module, ParseMode},
    parser::math::{statement::parse_statement, whitespace::ws},
};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::{
        complete::{is_not, tag},
        take_while1,
    },
    character::complete::{alpha1, line_ending, multispace0},
    combinator::{map, opt, peek},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
};

fn comment_ending(input: &str) -> IResult<&str, ()> {
    map((ws(tag(";")), opt(line_ending)), |_| ()).parse(input)
}

fn nothing(input: &str) -> IResult<&str, ()> {
    Ok((input, ()))
}
//metadata comment block
fn parse_metadata_comment(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    delimited(
        ws(tag("/*")),
        map(
            (
                separated_list0(
                    // Changed from separated_list1
                    comment_ending,
                    map(
                        (is_not(":"), ws(tag(":")), is_not(";\n*")),
                        |(key, _, value)| (key, value),
                    ),
                ),
                opt(comment_ending), // Allow optional trailing semicolon
            ),
            |(list, _)| list,
        ),
        ws(tag("*/")),
    )
    .parse(input)
}

pub fn parse_metadata(input: &str) -> IResult<&str, Metadata> {
    map(parse_metadata_comment, |pairs| {
        let mut metadata = Metadata {
            author: None,
            author_email: None,
            url: None,
            title: None,
        };

        for (key, value) in pairs {
            match key {
                "author" => metadata.author = Some(value.trim().to_string()),
                "author_email" => metadata.author_email = Some(value.trim().to_string()),
                "url" => metadata.url = Some(value.trim().to_string()),
                "title" => metadata.title = Some(value.trim().to_string()),
                _ => {} // Ignore unknown keys
            }
        }

        metadata
    })
    .parse(input)
}

pub fn parse_module(input: &str, is_entry: bool, mode: ParseMode) -> IResult<&str, Module> {
    map(
        (
            opt(parse_metadata), // Make metadata optional
            multispace0,         // Consume any whitespace/newlines between metadata and statements
            separated_list1(line_ending, parse_statement),
        ),
        |(metadata, _, statements)| {
            let metadata = metadata.unwrap_or(Metadata {
                author: None,
                author_email: None,
                url: None,
                title: None,
            });

            if is_entry {
                Module::Entry {
                    metadata,
                    statements,
                }
            } else {
                match mode {
                    ParseMode::Module => Module::Module {
                        statements,
                        metadata,
                    },
                    ParseMode::Frame => Module::Frame {
                        metadata,
                        statements,
                    },
                }
            }
        },
    )
    .parse(input)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_module() {
        let input = "x
d
            ";
        let (_, metadata) = parse_module(input, true, ParseMode::Frame).unwrap();
        println!("{:?}", metadata)
    }
    #[test]
    fn test_parse_metadata_empty() {
        let input = "";
        let (_, metadata) = parse_metadata(input).unwrap();
        assert_eq!(
            Metadata {
                author: Option::None,
                author_email: Option::None,
                url: Option::None,
                title: Option::None,
            },
            metadata
        )
    }
    use super::*;

    #[test]
    fn test_parse_metadata() {
        let input = "/*
            author:piderking;author_email:piderking8@gmail.com;
        url:https://github.com/piderking/mtx;title:MTX Example
        */";
        let (_, metadata) = parse_metadata(input).unwrap();
        assert_eq!(
            Metadata {
                author: Some("piderking".to_string()),
                author_email: Some("piderking8@gmail.com".to_string()),
                url: Some("https://github.com/piderking/mtx".to_string()),
                title: Some("MTX Example".to_string()),
            },
            metadata
        )
    }
}
