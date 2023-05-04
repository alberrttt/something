use something_ast::{declaration::FunctionDeclaration, expression::Expression};
use something_frontend_tokenizer::Parse;

fn main() {
    let mut tokens = something_frontend_tokenizer::Tokenizer::new(include_str!("./cases/fn.txt"))
        .tokens()
        .unwrap();
    dbg!(FunctionDeclaration::parse(&mut tokens));
}
