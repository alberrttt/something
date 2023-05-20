use something_ast::prelude::*;
use something_typechecker::{prelude::*, TypeChecker};
fn main() {
    let ast: Ast = Ast::from(
        "fn main(number hello) {
            1 + 2;
            return hello > 0;
        } -> bool",
    );
    let mut type_checker = TypeChecker::new(ast);
    type_checker.link_global_symbols();

    for (symbol, fn_decl) in type_checker.fn_decl.iter_mut() {
        println!("Symbol name: {}", symbol);
        println!("Function declaration: {}", fn_decl);

        match fn_decl.type_check(()) {
            Ok(ok) => {}
            Err(err) => {
                println!("{}", err);
                panic!()
            }
        };
    }
}
