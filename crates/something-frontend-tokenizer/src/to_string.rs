use crate::Tokens;

impl Tokens {
    pub fn to_source_string(&self) -> String {
        let mut s = String::new();
        let iter = self.0.iter().peekable();
        let mut tokens = self.0.iter().peekable();
        for token in iter {
            tokens.next();
            let offset = match tokens.peek() {
                Some(next) => next.span().start - token.span().end,
                None => 0,
            };
            let whitespace = " ".repeat(offset);
            s.push_str(format!("{token}{whitespace}").as_str());
        }
        s
    }
}
#[test]
fn var() {
    let tokens = Tokens::from("let a = x + 2 + 3 - 4;");
    println!("{}", tokens.to_source_string());
}
