use crate::prelude::*;
use something_frontend::VariableDeclaration;
use something_frontend_tokenizer::list::List;
#[test]
fn vars() {
    let vars: List<VariableDeclaration> = something_ast::ast!(
        "
        let a: number = 0;
        let b: bool = a;
    "
    );
    let mut ctx = BlockCtx::default();
    let mut typechecker = TypeChecker::default();
    for var in vars.iter() {
        var.type_check(&mut ctx);
    }
}
