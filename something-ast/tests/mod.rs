use something_ast::declaration::{self, Declaration, FunctionDeclaration};
use something_ast::expression::Expression;
use something_ast::Node;
use something_frontend_tokenizer::{tokens::Parse, *};
macro_rules! gen_tests {
    [$($file:literal = $name:ident),*] => {
        $(
            #[test]
            fn $name() {
                let source = include_str!(concat!("../cases/",$file, ".txt"));
                let mut tokens = Tokenizer::new(source).tokens().unwrap();
                let node = Node::parse(&mut tokens).unwrap();
                println!("{:#?}",&node);
            }
        )*
    };
}
gen_tests![
    "fn" = fn_test,
    "var" = var_test,
    "stmt" = stmt_test,
    "call" = call_test,
    "lit" = lit_test,
    "call_binary" = call_binary_test
];
#[test]
fn expr_test() {
    let mut tokens = Tokenizer::new(include_str!("../cases/expr.txt"))
        .tokens()
        .unwrap();
    dbg!(tokens.peek());

    dbg!(Expression::parse(&mut tokens));
}
