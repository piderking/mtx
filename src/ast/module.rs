use crate::ast::Statement;

pub struct Metadata {
    pub author: Option<String>,
    pub author_email: Option<String>,
    pub url: Option<String>,
    pub title: Option<String>,
}
// File Level ("imports and etc....")
pub enum Module {
    // Entry Point
    Entry {
        metadata: Metadata,
        statements: Vec<Statement>,
    },

    // Provides MTX Functions
    Module {
        statements: Vec<Statement>,
    },

    // View Port (section just added directly into)
    Frame {
        metadata: Metadata,
        statements: Vec<Statement>,
    },
}
