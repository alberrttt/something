use something_ast::declaration::{self, Declaration};
use something_frontend_tokenizer::{tokens::Parse, *};

fn main() {
    let mut tokens = Tokenizer::new("fn x").tokens().unwrap();
    println!("{}", tokens);
    let var = Declaration::parse(&mut tokens).unwrap();
    dbg!(var);
}
