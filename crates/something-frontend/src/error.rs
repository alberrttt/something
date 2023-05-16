use std::rc::Rc;

use super::prelude::*;
#[derive(Debug)]
pub enum Error {
    ExpectedToken(Token),
    ExpectedEnd,
    ExpectedAst(Box<dyn Parse>),
}
