use something_ast::declaration::{self, Declaration, FunctionDeclaration};
use something_ast::expression::Expression;
use something_ast::punctuated::Punctuated;
use something_ast::Node;
use something_frontend_tokenizer::lit::Literal;
use something_frontend_tokenizer::{Parse, *};
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
mod punctuated {
    use something_ast::punctuated::Punctuated;
    use something_dev_tools::tokens;
    use something_frontend_tokenizer::{lit::Literal, tokens, ParsingDisplay, Tokenizer};

    #[test]
    fn punctuated_terminating_test() -> Result<(), Box<dyn std::error::Error>> {
        let mut tokens = Tokenizer::new(include_str!("../cases/punctuated_terminating.txt"))
            .tokens()
            .unwrap();
        dbg!(tokens.peek());
        let tmp = Punctuated::<Literal, tokens::Comma>::parse_terminated(&mut tokens)?;
        dbg!(&tmp);
        println!("{}", tmp.display());
        Ok(())
    }
    #[test]
    fn punctuated_trailing_test() -> Result<(), Box<dyn std::error::Error>> {
        let mut tokens = Tokenizer::new(include_str!("../cases/punctuated_trailing.txt"))
            .tokens()
            .unwrap();
        dbg!(tokens.peek());

        dbg!(Punctuated::<Literal, tokens::Comma>::parse_trailing(
            &mut tokens
        )?);
        Ok(())
    }
    #[test]
    fn punctuated_no_trailing_test() -> Result<(), Box<dyn std::error::Error>> {
        let mut tokens = Tokenizer::new(include_str!("../cases/punctuated_no_trailing.txt"))
            .tokens()
            .unwrap();
        dbg!(tokens.peek());

        dbg!(Punctuated::<Literal, tokens::Comma>::parse_without_trailing(&mut tokens)?);
        Ok(())
    }
}
