pub mod base;
pub mod expressions;
use rustpython_parser::{
    ast::{Expr, ModModule, StmtReturn},
    text_size::TextRange,
};
use std::fmt::Debug;
use tokio_util::io::ReaderStream;

use crate::{
    ast::base::{Ident, Value, Variable},
    symbols::{self, Symbols},
};

// Mathematical Functions
pub trait Opperation
where
    Self: Debug,
{
    fn stringify(&self) -> String;
}

// Hold the Values
#[derive(Debug)]
pub struct Add {
    terms: Vec<Expression>,
}
impl Opperation for Add {
    fn stringify(&self) -> String {
        self.terms
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(format!(" {} ", Symbols::Addition.as_str()).as_str())
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
    VariableRef(Variable),
    Constant(Value),
    Empty, // 0
}

impl Expression {
    pub fn from_return_value(ret: StmtReturn) -> Expression {
        match ret.value {
            Option::Some(n) => Expression::from(*n),
            Option::None => Expression::Empty,
        }
    }
}
#[derive(Debug)]
pub enum System {
    Print(Expression),
}

#[derive(Debug)]
pub enum Definition {
    Function(Ident, Vec<Variable>, Expression),
    Constant(Ident, Expression),
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
