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

        for decl in fn_decl.fn_ast.as_ref().body.iter() {
            let node_type: Type = match decl {
                something_ast::Node::Statement(stmt) => stmt.clone().into(),
                something_ast::Node::Declaration(_) => todo!(),
            };
            println!("Node type: {}", node_type);
        }

        fn_decl.type_check((), ()).unwrap();
    }
}
