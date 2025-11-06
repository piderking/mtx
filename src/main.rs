use mtx::ast::AST;
use rustpython_parser::{Mode, lexer::lex, parse_tokens};
use std::fs;

fn main() {
    let python_source = r#"
def is_odd(i):
    return bool(i & 1)
"#;
    let tokens = lex(python_source, Mode::Module);
    match parse_tokens(tokens, Mode::Module, "<embedded>") {
        Ok(n) => println!("{:?}", n),
        Err(e) => panic!("{}", e),
    }
}
