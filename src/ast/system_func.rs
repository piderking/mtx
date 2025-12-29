use std::{fmt::Debug, path::Path};

use crate::ast::expressions::Expression;

#[derive(Debug)]
pub enum System {
    Print(Expression),

    // Import as a module (take its function)
    Import(String),

    // Import all of its content (frames)
    Frame(String),
}
