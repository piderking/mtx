pub mod base;
pub mod expressions;
pub mod module;
pub mod opperations;
pub mod system_func;

use crate::{
    ast::base::{Ident, Value},
    symbols::Symbols,
};
use std::fmt::Debug;

use expressions::Expression;

#[derive(Debug)]
pub enum Definition {
    //
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
    // TODO move system calls to expression
    // System(System),
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
