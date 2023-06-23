use something_ast::tokenizer::Tokenizer;

fn main() {
    let mut tokenizer = Tokenizer::new(include_str!("./code.txt"));
    let tokens = tokenizer.tokens().unwrap();
    devprintln!("{}", tokens);
}
