use crate::traits::Node;

use super::{
    nodes::{
        declaration::{function::Function, variable::Variable},
        statement::Statement,
    },
    Parser,
};
use parmesan_common::Spanned;
use parmesan_dev_macros::Spanned;

#[derive(Debug, Clone, PartialEq, Spanned)]
pub enum Item<'a> {
    Variable(Variable<'a>),
    Function(Function<'a>),
}
impl<'a> Node<'a> for Item<'a> {
    fn parse(parser: &mut super::ParseStream<'a>) -> Result<Self, crate::error::ParseError<'a>>
    where
        Self: Sized,
    {
        if let Ok(ok) = parser.step(Variable::parse) {
            return Ok(Self::Variable(ok));
        }
        if let Ok(ok) = parser.step(Function::parse) {
            return Ok(Self::Function(ok));
        }

        Err(crate::error::ParseError::ExpectedNode(
            crate::error::ExpectedNode {
                got: "Item",
                expected: "Variable or Function",
            },
        ))
    }
}

#[test]
fn test_var() {
    let mut parser = Parser::new("let x = 1;");
    let var: Item = <Item as Node>::parse(&mut parser.stream()).unwrap();
    dbg!(var);
}

#[test]
fn test_fn() {
    let mut parser = Parser::new("fn x() {}");
    let var: Item = <Item as Node>::parse(&mut parser.stream()).unwrap();
    dbg!(var);
}
