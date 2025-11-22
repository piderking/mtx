// Test Functions Here
use rustpython_parser::{Mode, lexer::lex, parse_tokens};
use std::fs;

pub(crate) fn parse_python_code(python_source: &'static str) -> rustpython_parser::ast::Mod {
    let tokens = lex(python_source, Mode::Module);
    match parse_tokens(tokens, Mode::Module, "<embedded>") {
        Ok(n) => n,
        Err(e) => panic!("{}", e),
    }
}
