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
    use crate::prelude::*;
    use something_frontend_tokenizer::prelude::*;
    let (vars, _): (List<VariableDeclaration>, _) = something_ast::ast!(
        "
        let a: number = 0;
        let b: bool = a;
    "
    );
    let mut ctx = BlockScope::default();
    let _typechecker = TypeChecker::default();
    for var in vars.iter() {
        handle_result!(var.type_check(&mut ctx));
    }
}
