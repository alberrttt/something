use something_ast::tokenizer::Tokenizer;

macro_rules! devprintln {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            print!(concat!("[",file!(), ":", line!(), "]: "));
            println!($($arg)*);

        }
    }
}
fn main() {
    let mut tokenizer = Tokenizer::new(include_str!("./code.txt"));
    let tokens = tokenizer.tokens().unwrap();
    devprintln!("{}", tokens);
}
