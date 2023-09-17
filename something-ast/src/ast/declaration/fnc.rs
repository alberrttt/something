use crate::prelude::*;
use something_dev_tools::Node;
#[derive(Debug, Clone, PartialEq, Eq, Node)]
pub struct Function {
    fn_token: Fn,
    ident: Ident,
    paren: Paren<Vec<Ident>>,
}

#[test]
fn test_fn() {
    let function = node!(Function, "fn ident(x y z)");
    println!("{:#?}", function);
}
