#[derive(Debug)]
pub struct Key {
    inner: String
}

#[derive(Debug)]
pub struct Variable {
    inner: String
}

#[derive(Debug)]
pub enum Value {
    Char(i32),
    Number(f32)

}

// Transforms Like 
// Multiply
// Derivative
// Fraction



#[derive(Debug)]
pub struct Function {
    // Transforms == Opperations
    transform: Vec<Box<dyn Transform>>,
    expression: Vec<Expression>
}

#[derive(Debug)]
pub enum Expression {
    Function(Function),
    Constant(Value),
    Variable(Variable),
    Variables(Vec<Variable>)
}


#[derive(Debug)]
pub enum System {
    Print(Expression)
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
    Definition{key: Key, vars: Vec<Variable>, expression: Expression },
    

    Expression(Expression),


    // SYstem Commands
    System(System)

}

#[derive(Debug)]
pub struct AST {
    statements: Vec<Statement>
}

impl AST {
    pub fn from(str: String) -> AST {
        return AST {
            statements: vec![]
        }
    }
    pub fn new() -> AST {
        return AST {
            statements: vec![]
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