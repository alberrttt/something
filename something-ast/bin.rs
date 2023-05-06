use something_ast::{declaration::FunctionDeclaration, expression::Expression};
use something_frontend_tokenizer::{Parse, ParsingDisplay};

fn main() {
    let mut tokens = something_frontend_tokenizer::Tokenizer::new(include_str!("./cases/fn.txt"))
        .tokens()
        .unwrap();
    let parsed = FunctionDeclaration::parse(&mut tokens).unwrap();
    dbg!(&parsed);
    println!("{}", parsed.display());
}
