use something_frontend_tokenizer::Tokenizer;

fn main() {
    let mut tokenizer = Tokenizer::new(include_str!("./code.txt"));
    let tokens = tokenizer.tokens().unwrap();
    println!("{}", tokens);
}
