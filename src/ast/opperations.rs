
use std::fmt::Debug;
use crate::{
    ast::{base::{Ident, Value}, expressions::Expression},
    symbols::Symbols,
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
    pub second: Expression
}

#[derive(Debug)]
pub struct Index {
    pub base: Expression,
    pub index: Expression
}


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

impl Opperation for Index {
    fn stringify(&self) -> String {
       format!("{}_{}", self.base.to_string(), self.index.to_string() )
    }
    fn ty(&self) -> &str {
        "index"
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
