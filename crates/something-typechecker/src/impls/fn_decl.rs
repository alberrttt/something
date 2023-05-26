use crate::prelude::*;

impl TypeCheck<(), BlockScope> for FunctionDeclaration {
    fn type_check(&self, _ctx: ()) -> BlockScope {
        self.try_into().unwrap()
    }
}
impl TryFrom<&FunctionDeclaration> for BlockScope {
    type Error = TypeError;

    fn try_from(value: &FunctionDeclaration) -> Result<Self, Self::Error> {
        let mut ctx = BlockScope::default();
        for param in value.params.iter() {
            let ((type_name, name), _) = param.clone();
            let ty = type_name.type_check(()).unwrap();
            ctx.set(name, ty);
        }
        for node in value.body.iter() {
            node.type_check(&mut ctx);
        }
        Ok(ctx)
    }
}
#[test]
fn test_decl() {
    use something_ast::ast;
    let (fn_decl, _tokens): (FunctionDeclaration, _) = ast!(
        r###"
fn main(number p1, string p2) {
    let a = "hello";
    let b = a + p2;
    let c = 2;
    let d = c + p1;    
} -> void"###
    );
    let ctx = fn_decl.type_check(());
    dbg!(ctx);
}
