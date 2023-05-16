use prelude::*;
use something_ast::{traits::Children, Ast, TopLevelNode};
use traits::TypeCheck;

pub struct TypeChecker {
    pub symbols: Vec<Symbol>,
    pub ast: Ast,
}

impl TypeChecker {
    pub fn new(ast: Ast) -> Self {
        Self {
            symbols: Vec::new(),
            ast,
        }
    }
    /// Should only be called once per typechecker
    pub fn link_global_symbols(&mut self) {
        for child in self.ast.children() {
            match child {
                TopLevelNode::FunctionDeclaration(function_declaration) => {
                    self.symbols.push(Symbol::from(function_declaration))
                }
            }
        }
    }
}
impl TypeCheck<Ast> for TypeChecker {
    fn type_check(&mut self, ast: Ast) -> Result<(), Box<dyn std::error::Error>> {
        todo!();
        Ok(())
    }
}
mod prelude;
mod symbol;
mod traits;

#[test]
fn test() {
    let ast: Ast = Ast::from("fn main() {} -> void fn x() {} -> int");
    let mut type_checker = TypeChecker::new(ast);
    type_checker.link_global_symbols();
    dbg!(&type_checker.symbols);
}
