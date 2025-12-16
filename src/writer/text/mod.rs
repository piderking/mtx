use std::{char, fmt::format};

use crate::ast::{
    Comment, Definition, Expression, Statement, base::{Ident, Value}
};

// For Parsing the Files into our AST
const FILE_TYPE: &'static str = ".mtx";

// Implemement ToString For Them All In This Directory

impl ToString for Ident {
    fn to_string(&self) -> String {
        self.inner.clone()
    }
}


impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Char(n) => format!("'{}'", char::from_u32(*n as u32).unwrap() ),
            Value::Number(n) => n.to_string(),
        }
    }
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::Opperations(opperation) => opperation.stringify(),
            Expression::FunctionCall(ident, vars) => format!(
                "{}({})",
                ident.to_string(),
                vars.iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Expression::VariableRef(var) => var.to_string(),
            Expression::Constant(value) => value.to_string(),
            Expression::List(exp) => format!(
                "{}",
                exp.iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Expression::Empty => stringify!().to_string(),
        }
    }
}

impl ToString for Definition {
    fn to_string(&self) -> String {
        match self {
            Definition::Function(ident, vars, expression) => {
                format!(
                    "{}({}) = {}",
                    ident.to_string(),
                    vars.iter()
                        .map(|f| f.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                    expression.to_string()
                )
            }
            Definition::Constant(ident, expression) => {
                format!("{} = {}", ident.to_string(), expression.to_string())
            }
        }
    }
}

impl ToString for Comment {
    fn to_string(&self) -> String {
        match self {
            Comment::Single(n) => n.clone(),
            Comment::Multi(n) => n.clone(),
        }
    }
}
impl ToString for Statement {
    fn to_string(&self) -> String {
        match self {
            Statement::Definition(definition) => definition.to_string(),
            Statement::Expression(expression) => expression.to_string(),
            Statement::Comment(comment) => comment.to_string(),
            Statement::System(system) => {
                todo!("When used as a compiled language... Not implemented now...")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn empty_stringify() {
        assert_eq!("".to_string(), Expression::Empty.to_string())
    }

    #[test]
    fn constant_stringify() {
        assert_eq!(
            "10".to_string(),
            Expression::Constant(Value::Number(10.0)).to_string()
        )
    }

    #[test]
    fn variable_stringify() {
        assert_eq!(
            "x".to_string(),
            // Works Same as Variable Just Into trait doesn't work without implied types
            Expression::VariableRef(format!("x").into()).to_string()
        )
    }
    #[test]
    fn function_call_stringify() {
        assert_eq!(
            "f(x,y)".to_string(),
            // Works Same as Variable Just Into trait doesn't work without implied types
            Expression::FunctionCall(
                format!("f").into(),
                vec![
                    Expression::VariableRef(format!("x").into()),
                    Expression::VariableRef(format!("y").into())
                ]
            )
            .to_string()
        )
    }
    #[test]
    fn addition_opperation_stringify() {
        assert_eq!(
            "x".to_string(),
            // Works Same as Variable Just Into trait doesn't work without implied types
            Expression::VariableRef(format!("x").into()).to_string()
        )
    }
}
