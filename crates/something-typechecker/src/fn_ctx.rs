use crate::{
    prelude::*,
    traits::{ResolveType, Scope},
};
use something_frontend::prelude::*;

use something_ast::Node;
#[derive(Debug, Clone)]
pub struct FnScope {
    pub ast: FunctionDeclaration,
    pub scope: BlockScope,
}

impl Scope for FnScope {
    fn get(&self, name: &Ident) -> Option<Type> {
        self.scope.get(name)
    }

    fn set(&mut self, name: &Ident, ty: Type) {
        self.scope.set(name, ty)
    }
}

impl FnScope {
    fn new(ast: FunctionDeclaration) -> Self {
        let scope = BlockScope::new();
        let mut fn_scope = Self {
            ast: ast.clone(),
            scope,
        };
        for ((ty, name), _) in ast.params.iter() {
            fn_scope.scope.set(name, ty.try_into().unwrap())
        }
        for node in ast.body.iter() {
            fn_scope.fn_decl_node(node)
        }
        fn_scope
    }
    fn fn_decl_node(&mut self, node: &Node) {
        match node {
            Node::Statement(_) => todo!(),
            Node::Declaration(declaration) => match declaration {
                something_frontend::Declaration::Function(_) => todo!(),
                something_frontend::Declaration::Var(var_decl) => self.var_decl(var_decl),
            },
        }
    }
    fn var_decl(&mut self, var_decl: &VariableDeclaration) {
        let (annotation, ty) = var_decl.resolve_type();
        if annotation != ty {
            panic!("type mismatch")
        }
        self.scope.set(&var_decl.name, ty)
    }
}

#[test]
pub fn test() {
    let (fn_decl, tokens): (FunctionDeclaration, _) = ast!(
        r#"
fn main(number num) { 
    let a: number = num + 1;
    let error: string = num;
    test_call(a);
} -> void"#
    );
    let fn_ctx = FnScope::new(fn_decl);
    dbg!(fn_ctx);
}
