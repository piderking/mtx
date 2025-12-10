pub mod base;
pub mod expressions;
use rustpython_parser::{
    ast::{Expr, ModModule, StmtReturn},
    text_size::TextRange,
};
use std::fmt::Debug;
use tokio_util::io::ReaderStream;

use crate::{
    ast::base::{Ident, Value},
    symbols::{Symbols},
};

// Mathematical Functions
pub trait Opperation
where
    Self: Debug,
{
    fn stringify(&self) -> String;
    fn ty(&self) -> &str;
}

// Hold the Values
#[derive(Debug)]
pub struct Add {
    pub terms: Vec<Expression>,
}
// Hold the Values
#[derive(Debug)]
pub struct Multi {
    pub terms: Vec<Expression>,
}



// Hold the Values
#[derive(Debug)]
pub struct Root {
    pub degree: Expression,
    pub radicand: Expression,
    
}

#[derive(Debug)]
pub struct Exp {
    pub exponent: Expression,
    pub base: Expression,
    
}

// Hold the Values
#[derive(Debug)]
pub struct Sub {
    pub first: Expression,
    pub second: Expression
}
#[derive(Debug)]
pub struct Div {
    pub first: Expression,
    pub second: Expression}

impl Opperation for Add {
    fn stringify(&self) -> String {
        self.terms
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(format!(" {} ", Symbols::Addition.as_str()).as_str())
    }
    
    fn ty(&self) -> &str {
        "add"
    }
}

impl Opperation for Multi {
    fn stringify(&self) -> String {
        self.terms
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(format!(" {} ", Symbols::Multiplication.as_str()).as_str())
    }
    fn ty(&self) -> &str {
        "multi"
    }
}

impl Opperation for Root {
    fn stringify(&self) -> String {
       todo!()
    }
    fn ty(&self) -> &str {
        "root"
    }
}

impl Opperation for Exp {
    fn stringify(&self) -> String {
        todo!()
    }
    fn ty(&self) -> &str {
        "exp"
    }
}
// Root

#[derive(Debug)]
pub enum Expression {
    // Mathematic Opperations
    // Advanced Mathematical Opperations
    Opperations(Box<dyn Opperation>),
    // User Definied Functions
    FunctionCall(Ident, Vec<Expression>),
    VariableRef(Ident),
    Constant(Value),
    Empty, // 0
}

#[derive(Debug)]
pub enum System {
    Print(Expression),
}

#[derive(Debug)]
pub enum Definition {
    Function(Ident, Vec<Ident>, Expression),
    Constant(Ident, Expression),
}
#[derive(Debug)]
pub enum Comment {
    Single(String),
    Multi(String),
}

#[derive(Debug)]
pub enum Statement {
    // TODO Optional
    // Type Defintiions (LINTING )
    //Structure(),

    // Constant Definitions
    // s =
    // Function Definitions
    // f(x)
    Definition(Definition),

    // Print
    Expression(Expression),
    
    Comment(Comment),
    // System Commands
    System(System),
}

#[derive(Debug)]
pub struct AST {
    // AST is for TODO PROCESS AST -> MATHEMATICS
    // imports: Vec<>
    // global_definitions: Vec<Definition>
    //
    statements: Vec<Statement>,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn empty_ast() {
        //assert_eq!(add(1, 2), 3);
    }
}
