use crate::prelude::*;
use something_frontend::VariableDeclaration;
use something_frontend_tokenizer::list::List;
macro_rules! handle_result {
    ($expr:expr) => {
        match $expr {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
                panic!();
            }
        }
    };
}

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
        handle_result!(var.type_check(&mut ctx));
    }
}
