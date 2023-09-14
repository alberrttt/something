use something_dev_tools::Node;

use crate::token;

#[derive(Debug, Clone, PartialEq, Eq, Node)]
pub struct Function {
    fn_token: token::Fn,
}
