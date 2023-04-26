use something_ast::declaration::{self, Declaration, FunctionDeclaration};
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
                dbg!(&node);
            }
        )*
    };
}
gen_tests!["fn" = fn_test, "var" = var_test, "expr" = expr];
