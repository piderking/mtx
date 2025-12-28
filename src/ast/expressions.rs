
use std::fmt::Debug;
use crate::{
    ast::{base::{Ident, Value}, opperations::Opperation, system_func::System},
    symbols::Symbols,
};



#[derive(Debug)]
pub enum Expression {
    // Mathematic Opperations
    // Advanced Mathematical Opperations
    Opperations(Box<dyn Opperation>),


    List(Vec<Expression>),
    


    // User Definied Functions
    FunctionCall(Ident, Vec<Expression>),

    // System
    System(Box<System>),

    VariableRef(Ident),
    Constant(Value),
    Empty, // 0
}


