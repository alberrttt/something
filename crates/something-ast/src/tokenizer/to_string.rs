use super::prelude::*;

pub trait ToSourceString {
    fn to_source_string(&self) -> String;
}
impl TokenStream {
    pub fn to_source_string(&self) -> String {
        let mut result = String::new();
        let iter = self.iter().peekable();
        let mut tokens = self.iter().peekable();
        for token in iter {
            tokens.next();
            let offset = match tokens.peek() {
                Some(next) => next.span().start - token.span().end,
                None => 0,
            };
            let whitespace = " ".repeat(offset);
            result.push_str(format!("{token}{whitespace}").as_str());
        }
        result
    }
}
#[test]
fn var() {
    let tokens = TokenStream::from("let a = x + 2 + 3 - 4;");
    println!("{}", tokens.to_source_string());
}
