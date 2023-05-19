use std::{collections::HashMap, rc::Rc};

use prelude::*;
use something_ast::{prelude::Binary, statement::Statement, traits::Children, Ast, TopLevelNode};
use something_frontend_tokenizer::ParsingDisplay;
use traits::TypeCheck;

pub struct TypeChecker {
    pub symbols: Vec<Rc<Symbol>>,
    pub fn_decl: HashMap<Rc<Symbol>, Function>,
    pub ast: Ast,
}

impl TypeChecker {
    pub fn new(ast: Ast) -> Self {
        Self {
            symbols: Vec::new(),
            fn_decl: HashMap::new(),
            ast,
        }
    }
    /// Should only be called once per typechecker
    pub fn link_global_symbols(&mut self) {
        for child in self.ast.children() {
            match child {
                TopLevelNode::FunctionDeclaration(function_declaration) => {
                    let symbol = Symbol::from(function_declaration);
                    self.symbols.push(Rc::new(symbol));
                    let symbol = self.symbols.last().unwrap().clone();
                    let fn_type: Function = function_declaration.into();
                    self.fn_decl.insert(symbol, fn_type);
                }
            }
        }
    }
}
impl TypeCheck<Ast, ()> for TypeChecker {
    fn type_check(&mut self, ast: Ast, _: ()) -> Result<(), Box<dyn std::error::Error>> {
        todo!();
        Ok(())
    }
}

mod context;
pub mod prelude;
mod primitives;
mod symbol;
mod traits;

impl From<Statement> for Type {
    fn from(value: Statement) -> Self {
        match value {
            Statement::Return(_, expression, _) => expression.into(),
            Statement::Expression(expression, _) => Self::void(),
        }
    }
}
impl From<Binary> for Type {
    fn from(value: Binary) -> Self {
        use something_ast::prelude::Operator::*;
        match value.operator {
            Plus | Minus | Multiply | Divide => Self::number(),
            PlusEqual | MinusEqual | MultiplyEqual | DivideEqual => Self::number(),
            EqualEqual | BangEqual => Self::boolean(),
            Greater | Less | GreaterEqual | LessEqual => Self::boolean(),
            _ => todo!(),
        }
    }
}
