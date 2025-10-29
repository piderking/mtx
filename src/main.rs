use mtx::ast::AST;
use syn::{parse_file, Expr, ExprWhile};
use quote::quote;
use std::fs;

fn main() {
    let file_path = "conv.rs"; // Replace with your file path
    let contents = fs::read_to_string(file_path);

    match contents {
        Ok(n) => match parse_file(&n){
            Ok(n) => {
                let ast: AST = n.into();
                println!("File Parsed Sucessfully!")
            },
            Err(e) => panic!("Parsed with error {}",  e),
        },
        Err(e) => panic!("Couldn't read file: {}", e)
    };
    
}
