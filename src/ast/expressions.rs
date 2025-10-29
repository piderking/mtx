use std::fmt::{self, Debug, Formatter};
use super::base::{Value, Variable};


pub trait Transform where Self: Debug  {
    // How many of the next "expression" it needs
    fn size(&self) -> usize {
        return 1
    }
}



#[derive(Debug)]
pub struct Multiply {
    
}


#[derive(Debug)]
pub struct Function {
    // Transforms == Opperations
    transform: Vec<Box<dyn Transform>>,
    expression: Vec<Expression>
}

#[derive(Debug)]
pub enum Expression {
    FunctionCall(Function),
    Constant(Value),
    Variable(Variable),
    Variables(Vec<Variable>)
}

impl From<syn::Expr> for Expression {
    fn from(value: syn::Expr) -> Self {
        match value {
            syn::Expr::Lit(n) => {
                match n.lit {
                    syn::Lit::Str(lit_str) => todo!(),
                    syn::Lit::ByteStr(lit_byte_str) => todo!(),
                    syn::Lit::CStr(lit_cstr) => todo!(),
                    syn::Lit::Byte(lit_byte) => todo!(),
                    syn::Lit::Char(lit_char) => todo!(),
                    syn::Lit::Int(lit_int) => Expression::Constant(Value::Number(lit_int.base10_parse::<f32>().expect("Needs to be str?"))),
                    syn::Lit::Float(lit_float) => Expression::Constant(Value::Number(lit_float.base10_parse::<f32>().expect("Needs to be str?"))),
                    _ => todo!(),
                } 
            }
            _ => todo!("Converting Expr -> Expression"),
        }
    }
}