use something_frontend::tokenizer::Tokenizer;
fn main() {
    let mut tokenizer = Tokenizer::new(include_str!("./code.txt"));
    let tokens = tokenizer.all_tokens().unwrap();
    dbg!(tokens);
}
