use something_ast::declaration::{self, Declaration, FunctionDeclaration};
use something_frontend_tokenizer::{tokens::Parse, *};

#[test]
fn main() {
    let mut tokens = Tokenizer::new("[targets[]]: fn x() {}").tokens().unwrap();
    for token in tokens.iter() {
        println!("{:?}", token);
    }
    let var = FunctionDeclaration::parse(&mut tokens).unwrap();
    dbg!(var);
}
