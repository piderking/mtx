use rustpython_parser::ast::{Expr, ExprName, Identifier, Mod, ModModule};

use crate::ast::{AST, Definition, Expression, Statement,};

impl From<Identifier> for Variable {
    fn from(value: Identifier) -> Self {
        Self {
            inner: String::from(value.as_str()),
        }
    }
}

impl From<ExprName> for Variable {
    fn from(value: ExprName) -> Self {
        value.id.into()
    }
}
impl From<Mod> for AST {
    fn from(value: Mod) -> Self {
        match value {
            Mod::Module(mod_module) => todo!(),
            Mod::Interactive(mod_interactive) => todo!(),
            Mod::Expression(mod_expression) => todo!(),
            Mod::FunctionType(mod_function_type) => panic!("unsupported feature for now"),
        }
    }
}
impl From<Expr> for Expression {
    fn from(expr: Expr) -> Self {
        match expr {
            Expr::BoolOp(expr_bool_op) => todo!("unsupported type: lists"),
            Expr::NamedExpr(expr_named_expr) => {
                panic!("unsupported, no assignment allowed in expressions")
            }
            Expr::BinOp(expr_bin_op) => todo!(),
            Expr::UnaryOp(expr_unary_op) => todo!(),
            Expr::Lambda(expr_lambda) => todo!(),
            Expr::IfExp(expr_if_exp) => todo!(),
            Expr::Dict(expr_dict) => todo!(),
            Expr::Set(expr_set) => todo!(),
            Expr::ListComp(expr_list_comp) => todo!(),
            Expr::SetComp(expr_set_comp) => todo!(),
            Expr::DictComp(expr_dict_comp) => todo!(),
            Expr::GeneratorExp(expr_generator_exp) => todo!(),
            Expr::Await(expr_await) => todo!(),
            Expr::Yield(expr_yield) => todo!(),
            Expr::YieldFrom(expr_yield_from) => todo!(),
            Expr::Compare(expr_compare) => todo!(),
            Expr::Call(expr_call) => todo!(),
            Expr::FormattedValue(expr_formatted_value) => todo!(),
            Expr::JoinedStr(expr_joined_str) => todo!(),
            Expr::Constant(expr_constant) => todo!(),
            Expr::Attribute(expr_attribute) => todo!(),
            Expr::Subscript(expr_subscript) => todo!(),
            Expr::Starred(expr_starred) => todo!(),
            Expr::Name(expr_name) => todo!(),
            Expr::List(expr_list) => todo!(),
            Expr::Tuple(expr_tuple) => todo!(),
            Expr::Slice(expr_slice) => todo!(),
        }
    }
}
impl From<ModModule> for Statement {
    fn from(value: ModModule) -> Self {
        match value.body.iter().nth(0) {
            Some(n) => match n {
                rustpython_parser::ast::Stmt::FunctionDef(func_def) => {
                    Statement::Definition(Definition::Function(
                        func_def.name.clone().into(),
                        func_def // Implement All Other Arg than just positional args
                            .args
                            .posonlyargs
                            .iter()
                            .map(|i| i.def.arg.clone().into())
                            .collect(),
                        {
                            // Do Stuff with the body of the function
                            // Iterate Through Body
                            // Value of Return Result
                            match func_def.body.last() {
                                Some(n) => match n {
                                    rustpython_parser::ast::Stmt::Return(ret) => {
                                        Expression::from_return_value(ret.clone())
                                    }
                                    _ => todo!("Value MUST be returned..."),
                                },

                                None => Expression::Empty,
                            }
                        },
                    ))
                }
                // rustpython_parser::ast::Stmt::AsyncFunctionDef(stmt_async_function_def) => todo!(),
                _ => todo!("Not Implemented Yet"),
            },
            None => panic!("Empty Mod Module"),
        }
    }
}
#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_python_expression() {}
}
