use something_dev_tools::Node;

use crate::prelude::*;
#[derive(Debug, Clone, PartialEq, Eq, Node)]
pub struct Paren<T: Node> {
    pub open: LeftParen,
    pub inner: T,
    pub close: RightParen,
}
#[test]
fn paren() {
    let tokens = TokenStream::from("(ident)");
    let mut parser = Parser::from(tokens);
    let paren = parser.parse::<Paren<Ident>>().unwrap();
    dbg!(&paren);
    assert_eq!(paren.inner.to_string(), "ident".to_string(),);
}
