use something_ast::declaration::{self, Declaration};
use something_frontend_tokenizer::{tokens::Parse, *};

fn main() {
    let mut tokens = Tokenizer::new("let x = 5").tokens().unwrap();
    println!("{}", tokens);
    let var = Declaration::parse(&mut tokens).unwrap();
    dbg!(var);
}
