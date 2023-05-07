use something_ast::{declaration::FunctionDeclaration, expression::Expression, Ast};
use something_frontend_tokenizer::{Parse, ParsingDisplay};

fn main() {
    let mut tokens = something_frontend_tokenizer::Tokenizer::new(include_str!("./cases/fn.txt"))
        .tokens()
        .unwrap();
    let parsed = Ast::parse(&mut tokens).unwrap();
    dbg!(&parsed);
    println!("{}", parsed.display());
}
