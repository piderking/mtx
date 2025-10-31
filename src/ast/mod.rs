pub mod base;
pub mod expressions;
use tokio_util::io::ReaderStream;

use expressions::Transform;

use crate::ast::{
    base::{Key, Variable},
    expressions::Expression,
};
use std::fmt::Debug;

// Transforms Like
// Multiply
// Derivative
// Fraction

#[derive(Debug)]
pub enum System {
    Print(Expression),
}

#[derive(Debug)]
pub enum Statement {
    // TODO Optional
    // Type Defintiions (LINTING )
    //Structure(),

    // Constant Definitions
    // X = 5
    // X = ( f(z) )
    // F'(X)
    Definition {
        key: Key,
        vars: Vec<Variable>,
        expression: Expression,
    },

    Expression(Expression),

    // System Commands
    System(System),
}
impl From<syn::Item> for Statement {
    fn from(value: syn::Item) -> Self {
        match value {
            // const MAX: u16 = 5555
            syn::Item::Const(item_const) => Self::Definition {
                key: item_const.ident.into(),
                vars: Vec::new(),
                expression: item_const.expr.into(),
            },
            //syn::Item::Enum(item_enum) => todo!(),
            //syn::Item::ExternCrate(item_extern_crate) => todo!(),
            syn::Item::Fn(func) => Self::Definition {
                key: func.sig.ident.into(),
                vars: func
                    .sig
                    .inputs
                    .iter()
                    .filter_map(|arg| match arg {
                        syn::FnArg::Typed(pt) => {
                            if let syn::Pat::Ident(ident) = *pt.pat {
                                Option::Some(ident.ident.into())
                            } else {
                                Option::None
                            }
                        }
                        _ => Option::None,
                    })
                    .collect(),
                expression: ,
            },
            //syn::Item::ForeignMod(item_foreign_mod) => todo!(),
            //syn::Item::Impl(item_impl) => todo!(),
            //syn::Item::Macro(item_macro) => todo!(),
            //syn::Item::Mod(item_mod) => todo!(),
            // static BIKE: Shed = Shed(42);
            syn::Item::Static(item_static) => todo!(),
            //syn::Item::Struct(item_struct) => todo!(),
            //syn::Item::Trait(item_trait) => todo!(),
            //syn::Item::TraitAlias(item_trait_alias) => todo!(),
            //syn::Item::Type(item_type) => todo!(),
            //syn::Item::Union(item_union) => todo!(),
            //syn::Item::Use(item_use) => todo!(),
            //syn::Item::Verbatim(token_stream) => todo!(),
            _ => todo!("Not Implemented Yet"),
        }
    }
}

#[derive(Debug)]
pub struct AST {
    statements: Vec<Statement>,
}

impl From<syn::File> for AST {
    fn from(value: syn::File) -> Self {
        AST {
            statements: value.items.iter().map(|f| f.clone().into()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn empty_ast() {
        let ast: AST = AST::new();
        println!("{:?}", ast)

        //assert_eq!(add(1, 2), 3);
    }
}

