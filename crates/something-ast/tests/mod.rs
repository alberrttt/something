use something_ast::ast::expression::Expression;

use something_ast::ast::Node;

use something_ast::tokenizer::{Parse, *};
macro_rules! gen_tests {
    [$($file:literal = $name:ident),*] => {
        $(
            #[test]
            fn $name() {
                let source = include_str!(concat!("../cases/",$file, ".txt"));
                let mut tokens = Tokenizer::new(source).tokens().unwrap();
                let node = Node::parse(&mut tokens).unwrap();
                println!("{:#?}",&node);
                println!("{}",node.display());
            }
        )*
    };
}
gen_tests![
    "fn" = fn_test,
    "var" = var_test,
    "stmt" = stmt_test,
    "call" = call_test,
    "lit" = lit_test
];
#[test]
fn binary_test() {
    let mut tokens = Tokenizer::new(include_str!("../cases/binary.txt"))
        .tokens()
        .unwrap();

    dbg!(Expression::parse(&mut tokens).unwrap());
}
#[test]
fn call_binary_test() {
    let mut tokens = Tokenizer::new(include_str!("../cases/call_binary.txt"))
        .tokens()
        .unwrap();
    dbg!(tokens.peek());

    dbg!(Expression::parse(&mut tokens).unwrap().display());
}
#[test]
fn expr_test() {
    let mut tokens = Tokenizer::new(include_str!("../cases/expr.txt"))
        .tokens()
        .unwrap();
    dbg!(tokens.peek());

    dbg!(Expression::parse(&mut tokens));
}
#[test]
fn if_expr_test() {
    let mut tokens = Tokenizer::new(include_str!("../cases/if.txt"))
        .tokens()
        .unwrap();
    dbg!(tokens.peek());

    dbg!(Expression::parse(&mut tokens));
}
mod punctuated {
    use something_ast::ast::punctuated::Punctuated;

    use something_ast::tokenizer::{prelude::*, Tokenizer};

    #[test]
    fn punctuated_terminating_test() {
        let mut tokens = Tokenizer::new(include_str!("../cases/punctuated_terminating.txt"))
            .tokens()
            .unwrap();
        dbg!(tokens.peek());
        let tmp = Punctuated::<Literal, token::Comma>::parse_terminated(&mut tokens).unwrap();
        dbg!(&tmp);
        println!("{}", tmp.display());
    }
    #[test]
    fn punctuated_trailing_test() {
        let mut tokens = Tokenizer::new(include_str!("../cases/punctuated_trailing.txt"))
            .tokens()
            .unwrap();
        dbg!(tokens.peek());

        dbg!(Punctuated::<Literal, Comma>::parse_trailing(&mut tokens).unwrap());
    }
    #[test]
    fn punctuated_no_trailing_test() {
        let mut tokens = Tokenizer::new(include_str!("../cases/punctuated_no_trailing.txt"))
            .tokens()
            .unwrap();
        dbg!(tokens.peek());

        dbg!(Punctuated::<Literal, Comma>::parse_without_trailing(&mut tokens).unwrap());
    }
}
