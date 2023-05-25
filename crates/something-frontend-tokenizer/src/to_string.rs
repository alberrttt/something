use crate::Tokens;

impl Tokens {
    pub fn to_source_string(&self) -> String {
        let mut s = String::new();
        for token in self.0.iter() {
            s.push_str(format!("{token}").as_str());
        }
        s
    }
}
#[test]
fn var() {
    let tokens = Tokens::from("let a = x + 2 + 3 - 4;");
    println!("{}", tokens.to_source_string());
}
