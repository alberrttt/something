use crate::{prelude::*, traits::InferType};
use something_frontend::prelude::*;

use something_ast::Node;
#[derive(Debug, Clone)]
pub struct FnCtx {
    pub ast: FunctionDeclaration,
    pub scope: BlockScope,
}

impl FnCtx {
    fn new(ast: FunctionDeclaration) -> Self {
        let scope = BlockScope::new();
        let mut fn_ctx = Self {
            ast: ast.clone(),
            scope,
        };
        for ((ty, name), _) in ast.params.iter() {
            fn_ctx.scope.set(name, ty.try_into().unwrap())
        }
        for node in ast.body.iter() {
            fn_ctx.fn_decl_node(node)
        }
        fn_ctx
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
        let (annotation, ty) = var_decl.infer_type();
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
    let fn_ctx = FnCtx::new(fn_decl);
    dbg!(fn_ctx);
}
