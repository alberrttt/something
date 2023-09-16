use something_dev_tools::Node;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Node)]
pub struct Function {
    fn_token: Fn,
    ident: Ident,
}
