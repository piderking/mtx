use std::fmt::Debug;

use crate::ast::expressions::Expression;



pub trait SystemFunction where Self: Debug {
    

}

#[derive(Debug)]
pub enum System {
    Print(Expression),
}

