use something_ast::ast;

use crate::prelude::*;

impl TypeCheck<(), BlockCtx> for FunctionDeclaration {
    fn type_check(&self, ctx: ()) -> BlockCtx {
        let mut ctx = BlockCtx::default();
        for param in self.params.iter() {
            let ((type_name, name), _) = param.clone();
            let ty = type_name.type_check(()).unwrap();
            ctx.set(name, ty);
        }
        for node in self.body.iter() {
            node.type_check(&mut ctx);
        }
        ctx
    }
}
#[test]
fn test_decl() {
    let (fn_decl, tokens): (FunctionDeclaration, _) = ast!(
        r###"
fn main(number p1, string p2) {
    let a: string = "hello";
    let b = a + p2;
    let c = 2;
    let d = c + p1;
    
} -> void"###
    );
    let ctx = fn_decl.type_check(());
    dbg!(ctx);
}
