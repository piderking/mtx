use crate::ast::Statement;

#[derive(Debug, PartialEq)]
pub struct Metadata {
    pub author: Option<String>,
    pub author_email: Option<String>,
    pub url: Option<String>,
    pub title: Option<String>,
}

pub enum ParseMode {
    Module,
    Frame,
}

// File Level ("imports and etc....")
#[derive(Debug)]
pub enum Module {
    // Entry Point
    Entry {
        metadata: Metadata,
        statements: Vec<Statement>,
    },

    // Provides MTX Functions
    Module {
        metadata: Metadata,
        statements: Vec<Statement>,
    },

    // View Port (section just added directly into)
    Frame {
        metadata: Metadata,
        statements: Vec<Statement>,
    },
}
